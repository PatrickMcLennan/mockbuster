use crate::operations::{aggregate_ratings, comments, ratings, tmdb_movies};
use crate::utils::document::{Document, DocumentProps};
use actix_session::Session;
use actix_web::{
    get,
    web::{Data, Path},
    Error as ActixError, HttpResponse,
};
use actix_web_flash_messages::IncomingFlashMessages;
use movie_view::movie_view::{Movie, Props};
use sea_orm::{DatabaseConnection, DbErr};
use serde_json::json;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

#[get("/movie/{tmdb_id}")]
async fn get(
    path: Path<u32>,
    http_client: Data<reqwest_middleware::ClientWithMiddleware>,
    db: Data<DatabaseConnection>,
    session: Session,
    messages: IncomingFlashMessages,
) -> Result<HttpResponse, ActixError> {
    let user_id = match session.get::<i32>("id") {
        Ok(v) => match v {
            Some(id) => id as i32,
            None => {
                return Ok(HttpResponse::Found()
                    .append_header(("Location", "/login"))
                    .finish());
            }
        },
        Err(error) => {
            println!("SessionGetError: {}", error);
            return Ok(HttpResponse::Found()
                .append_header(("Location", "/login"))
                .finish());
        }
    };

    let tmdb_id = path.into_inner();

    let tmdb_movie_result = match tmdb_movies::fetch::execute(
        tmdb_id.clone(),
        Some(http_client.as_ref().clone()),
    )
    .await
    {
        Ok(v) => v,
        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(
                &json!({"message": "This movie is unavailable at the moment; please try again later"}),
            ));
        }
    };

    let movie_clone = tmdb_movie_result.clone();

    let comments = match comments::fetch::by_tmdb_id::execute(tmdb_id, db.get_ref().clone()).await {
        Ok(v) => v,
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
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
    let user_rating = ratings
        .clone()
        .into_iter()
        .find(|rating| rating.1.as_ref().unwrap().id == user_id);

    let mut iterated_messages = messages.iter();
    let mut alert_copy: Option<String> = None;
    let mut alert_styles: Option<String> = None;
    match iterated_messages.len() {
        1 => {
            let first = iterated_messages.nth(0).unwrap();
            alert_copy = Some(first.content().to_string());
            alert_styles = Some(first.level().to_string());
            ()
        }
        _ => (),
    };

    let content = spawn_blocking(|| {
        use tokio::runtime::Builder;
        let set = LocalSet::new();
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<Movie>::with_props(|| Props {
                movie: Some(movie_clone),
                aggregate_rating: aggregate_rating,
                alert_copy: alert_copy,
                alert_styles: alert_styles,
                comments: Some(comments),
                user_score: match &user_rating {
                    Some(v) => Some(v.0.score.clone()),
                    None => None,
                },
                user_rated_date: match user_rating {
                    Some(v) => Some(v.0.created_at.format("%d-%m-%Y").to_string()),
                    None => None,
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
            wasm_assets: "movieView.js".to_string(),
            description: tmdb_movie_result.overview,
            title: tmdb_movie_result.title,
            content,
        })))
}
