use actix_web::{
    get,
    web::{Data, Query},
    Error as ActixError, HttpResponse,
};
use serde_json::json;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

use search_view::search_view::{Props, Search};
use validators::search_dto::SearchDTO;

use crate::operations::search_tmdb_movies::search_tmdb_movies;

#[get("/search")]
async fn get(
    params: Query<SearchDTO>,
    http_client: Data<reqwest::Client>,
) -> Result<HttpResponse, ActixError> {

    let movie_search_results = match search_tmdb_movies(
        params.clone().into_inner(),
        Some(http_client.as_ref().clone()),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            println!("[ERROR -- /search GET]: {}", e);
            return Ok(HttpResponse::InternalServerError().json(
                &json!({"message": "Search is down at the moment; please try again later"}),
            ));
        }
    };

    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<Search>::with_props(|| Props { 
				dto: Some(params.into_inner()), 
				movie_search_results: Some(movie_search_results) 
			})
			.render()
			.await
        })
    })
    .await
    .expect("[ERROR -- /search GET]: Thread allocation error");

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
					<title>Search | mockbuster</title>
					<script defer src="/assets/searchView.js"></script>
				</head>
				<body>
					{}
				</body>
			</html>
		"#,
            content
        )))
}
