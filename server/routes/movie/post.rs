use crate::operations::{ratings, tmdb_movies};
use actix_session::Session;
use actix_web::{
    post,
    web::{Data, Form, Path},
    HttpResponse,
    Error as ActixError
};
use sea_orm::{prelude::*, DatabaseConnection};
use tokio::task::spawn_blocking;
use serde_json::json;
use tokio::task::LocalSet;
use movie_view::movie_view::{Movie, Props};

#[derive(serde::Deserialize)]
struct RatingForm {
    pub score: f32,
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

    let tmdb_movie_result = match tmdb_movies::fetch::execute(tmdb_id.clone(), Some(http_client.as_ref().clone())).await {
        Ok(res) => res,
        Err(e) => {
            println!("[ERROR -- /movie/{} POST]: {}", tmdb_id, e);
            // return Redirect::to(format!("/movie/{tmdb_id}?banner=not_found")).see_other();
            return Ok(HttpResponse::InternalServerError().json(
                &json!({"message": "This movie is unavailable at the moment; please try again later"}),
            ));
        }
    };

    let movie_clone = tmdb_movie_result.clone();

    let user_id = match session.get("id") {
        Ok(v) => match v {
            Some(id) => id,
            None => return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish())
        },
        Err(_) => return Ok(HttpResponse::Found().append_header(("Location", "/login")).finish())
    };

    match ratings::create::execute(form.score, user_id, tmdb_id, db.get_ref().clone()).await {
        // Ok(_) => Redirect::to(format!("/movie/{tmdb_id}?banner=success")),
        Ok(_) => (),
        Err(db_err) => match db_err {
            DbErr::RecordNotInserted => {
                ()
                // Redirect::to(format!("/movie/{tmdb_id}?banner=already_rated")).see_other()
            }
            _ => {
                ()
                // Redirect::to(format!("/movie/{tmdb_id}?banner=failure")).see_other()
            },
        },
    };

    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<Movie>::with_props(|| Props {
                movie: Some(movie_clone),
            })
            .render()
            .await
        })
    })
    .await
    .expect("the thread has failed.");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(format!(
            r#"
			<html lang="en">
				<head>
					<meta charset="UTF-8" />
					<meta http-equiv="X-UA-Compatible" content="IE=edge" />
					<meta name="viewport" content="width=device-width, initial-scale=1.0" />
					<script defer src="/assets/bootstrap.js"></script>
					<link rel="stylesheet" href="/assets/bootstrap.css" />
					<title>{} | mockbuster</title>
					<script defer src="/assets/movieView.js"></script>
				</head>
				<body>
					{}
				</body>
			</html>
		"#,
            tmdb_movie_result.title, content
        )))
}
