use crate::operations::{aggregate_ratings, comments, ratings, tmdb_movies};
use crate::utils::document::{Document, DocumentProps};
use actix_session::Session;
use actix_web::{
    post,
    web::{Data, Form, Path},
    Error as ActixError, HttpResponse,
};
use movie_view::movie_view::{Movie, Props};
use sea_orm::{DatabaseConnection, DbErr};
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

#[derive(serde::Deserialize)]
struct RatingForm {
    pub score: f32,
    pub comment: Option<String>,
}

#[post("/movie/{tmdb_id}")]
async fn post(
    path: Path<u32>,
    http_client: Data<reqwest_middleware::ClientWithMiddleware>,
    Form(form): Form<RatingForm>,
    session: Session,
    db: Data<DatabaseConnection>,
) -> Result<HttpResponse, ActixError> {
    let tmdb_id = path.into_inner();
    let mut alert_styles = String::new();
    let mut alert_copy = String::new();

    let tmdb_movie_result = match tmdb_movies::fetch::execute(
        tmdb_id.clone(),
        Some(http_client.as_ref().clone()),
    )
    .await
    {
        Ok(res) => res,
        Err(e) => {
            println!("[ERROR -- /movie/{} POST]: {}", tmdb_id, e);
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish());
        }
    };

    let movie_clone = tmdb_movie_result.clone();

    let user_id = match session.get("id") {
        Ok(v) => match v {
            Some(id) => id,
            None => {
                return Ok(HttpResponse::Found()
                    .append_header(("Location", "/login"))
                    .finish())
            }
        },
        Err(_) => {
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish())
        }
    };

    match form.comment {
        Some(comment) => {
            match comments::create::execute(comment, user_id, tmdb_id, db.get_ref().clone()).await {
                Ok(_) => (),
                Err(_) => panic!(),
            }
        }
        None => (),
    }

    match ratings::create::execute(form.score.clone(), user_id, tmdb_id, db.get_ref().clone()).await
    {
        Ok(_) => {
            alert_styles = "success".to_string();
            alert_copy = format!(
                "Success!  You've rated {} {} / 10",
                tmdb_movie_result.title, form.score
            );
        }
        Err(_) => {
            alert_styles = "danger".to_string();
            alert_copy = "You have already rated this movie -- you cannot rate a movie twice, and you cannot change your score.".to_string()
        }
    };

    let ratings = match ratings::fetch::by_movie::execute(tmdb_id, db.get_ref().clone()).await {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let aggregate_rating =
        match aggregate_ratings::fetch::execute(tmdb_id, db.get_ref().clone()).await {
            Ok(v) => Some(v),
            Err(e) => match e {
                DbErr::RecordNotFound(id) if id == tmdb_id.to_string() => None,
                _ => return Ok(HttpResponse::InternalServerError().finish()),
            },
        };

    let comments = match comments::fetch::by_tmdb_id::execute(tmdb_id, db.get_ref().clone()).await {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };
    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();
        let rt = Builder::new_current_thread().enable_all().build().unwrap();
        let form_score_clone = form.score.clone();

        set.block_on(&rt, async {
            yew::ServerRenderer::<Movie>::with_props(move || Props {
                aggregate_rating: aggregate_rating,
                comments: Some(comments),
                movie: Some(movie_clone),
                alert_copy: if alert_copy.len() >= 1 {
                    Some(alert_copy)
                } else {
                    None
                },
                alert_styles: if alert_styles.len() >= 1 {
                    Some(alert_styles.clone())
                } else {
                    None
                },
                user_score: if alert_styles == "success" {
                    Some(form_score_clone)
                } else {
                    None
                },
                user_rated_date: if alert_styles == "success" {
                    Some(chrono::Utc::now().format("%d-%m-%Y").to_string())
                } else {
                    None
                },
                ratings: Some(ratings),
            })
            .render()
            .await
        })
    })
    .await
    .expect("the thread has failed.");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(Document::new(DocumentProps {
            description: "Rate a movie".to_string(),
            wasm_assets: "movieView.js".to_string(),
            title: tmdb_movie_result.title,
            content,
        })))
}
