use actix_session::Session;
use actix_web::web::Redirect;
use actix_web::{
    post,
    web::{Data, Form, Path},
    Error as ActixError,
};
use actix_web_flash_messages::FlashMessage;
use kafka_producer::KafkaProducer;
use models::tmdb_movies::movie_id_result::MovieIdResult;
use operations::{comments, ratings, tmdb_movies};
use sea_orm::{DatabaseConnection, DbErr};

#[derive(serde::Deserialize)]
struct RatingForm {
    pub score: Option<f32>,
    pub comment: Option<String>,
}

#[post("/movie/{tmdb_id}")]
async fn post(
    path: Path<u32>,
    http_client: Data<reqwest_middleware::ClientWithMiddleware>,
    Form(form): Form<RatingForm>,
    session: Session,
    db: Data<DatabaseConnection>,
    kafka_producer: Data<KafkaProducer>,
) -> Result<Redirect, ActixError> {
    let mut redirect_url = None;

    let user_id = match session.get("id") {
        Ok(Some(id)) => id,
        _ => {
            redirect_url = Some("/login".to_string());
            None
        }
    };

    let tmdb_id = path.into_inner();

    if redirect_url.is_none() {
        let mut tmdb_movie_result: Option<MovieIdResult> = None;
        match tmdb_movies::fetch::execute(tmdb_id.clone(), Some(http_client.as_ref().clone())).await
        {
            Ok(res) => {
                tmdb_movie_result = Some(res);
                ()
            }
            _ => {
                redirect_url = Some("/500".to_string());
                ()
            }
        };

        if redirect_url.is_none() {
            if let Some(comment) = form.comment {
                if comment.len() >= 1 {
                    match comments::create::execute(
                        comment,
                        user_id.unwrap(),
                        tmdb_id,
                        db.get_ref().clone(),
                        kafka_producer.get_ref().clone(),
                    )
                    .await
                    {
                        Err(_) => redirect_url = Some("/500".to_string()),
                        _ => (),
                    }
                }
            }

            if let Some(score) = form.score {
                match ratings::create::execute(
                    score.clone(),
                    user_id.unwrap(),
                    tmdb_id,
                    db.get_ref().clone(),
                )
                .await
                {
                    Ok(_) => {
                        FlashMessage::success(format!(
                            "Success! You've rated {} {} / 10",
                            tmdb_movie_result.unwrap().title,
                            score
                        ))
                        .send();
                    }
                    Err(error) => {
                        let message = match error {
                            DbErr::RecordNotFound(_score) if _score == score.to_string() => "You've already rated this movie. You cannot rate a movie twice and you cannot change your score.".to_string(),
                            _ => "Rating movies is not allowed at this time, please try again alter".to_string(),
                        };
                        FlashMessage::error(message).send();
                    }
                };
            }

            if redirect_url.is_none() {
                redirect_url = Some(format!("/movie/{}", tmdb_id));
            }
        }
    }

    Ok(Redirect::to(redirect_url.unwrap_or("/".to_string())).see_other())
}
