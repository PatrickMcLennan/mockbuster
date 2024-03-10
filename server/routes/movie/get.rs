use actix_web::{
    get,
    web::{Data, Path},
    Error as ActixError, HttpResponse,
};
use movie_view::movie_view::{Movie, Props};
use serde_json::json;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

use crate::operations::tmdb_movies;

#[get("/movie/{tmdb_id}")]
async fn get(
    path: Path<u32>,
    http_client: Data<reqwest::Client>,
) -> Result<HttpResponse, ActixError> {
    let tmdb_id = path.into_inner();

    let tmdb_movie_result = match tmdb_movies::fetch::execute(
        tmdb_id.clone(),
        Some(http_client.as_ref().clone()),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            println!("[ERROR -- /movie/{} GET]: {}", tmdb_id, e);
            return Ok(HttpResponse::InternalServerError().json(
                &json!({"message": "This movie is unavailable at the moment; please try again later"}),
            ));
        }
    };

    let movie_clone = tmdb_movie_result.clone();

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
