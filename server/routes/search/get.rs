use actix_web::{
    get,
    web::{Data, Query},
    Error as ActixError, HttpResponse,
};
use db_models::generated::ratings;
use models::tmdb::movie_search_result::{Movie, MovieSearchResults, Rating};
use sea_orm::{prelude::*, DatabaseConnection};
use search_view::search_view::{Props, Search};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;
use validators::search_dto::SearchDTO;

use crate::operations::search_tmdb_movies::search_tmdb_movies;

#[get("/search")]
async fn get(
    params: Query<SearchDTO>,
    http_client: Data<reqwest::Client>,
    db: Data<DatabaseConnection>,
) -> Result<HttpResponse, ActixError> {
    // Pass params to tmdb search, get postgres entries for all tmdb results

    let tmdb_search_results = match search_tmdb_movies(
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

    let ids = &tmdb_search_results
        .results
        .iter()
        .map(|movie| movie.id.clone().into())
        .collect::<Vec<sea_orm::Value>>();

    let postgres_search_results: HashMap<i32, Vec<Rating>> = match ratings::Entity::find()
        .filter(ratings::Column::MediaId.is_in(ids.clone()))
        .all(&db.get_ref().clone())
        .await
    {
        Ok(v) => {
            let mut hash_map: HashMap<i32, Vec<Rating>> = HashMap::new();
            for rating in v {
                hash_map
                    .entry(rating.media_id)
                    .or_insert_with(Vec::new)
                    .push(Rating {
                        id: rating.id.clone(),
                        user_id: rating.user_id.clone(),
                        score: rating.score.clone(),
                        media_id: rating.media_id.clone(),
                        created_at: rating.created_at.to_string(),
                        updated_at: rating.updated_at.to_string(),
                    });
            }
            hash_map
        }
        Err(e) => {
            println!("[ERROR -- /search GET]: {}", e);
            return Ok(HttpResponse::InternalServerError().json(
                &json!({"message": "Search is down at the moment; please try again later"}),
            ));
        }
    };

    let movie_search_results: Vec<Movie> = tmdb_search_results
        .results
        .iter()
        .map(|result| {
            let ratings: Vec<Rating> = match postgres_search_results.get(&result.id) {
                Some(v) => v.clone(),
                None => vec![],
            };
            Movie {
                tmdb: result.clone(),
                postgres: ratings.clone(),
            }
        })
        .collect::<Vec<Movie>>()
        .to_vec();

    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        let results_clone = tmdb_search_results.clone();

        set.block_on(&rt, async {
            let clone = tmdb_search_results.clone();
            yew::ServerRenderer::<Search>::with_props(move || Props {
                dto: Some(params.into_inner()),
                movie_search_results: Some(MovieSearchResults {
                    page: clone.page.clone(),
                    results: movie_search_results,
                    total_pages: clone.total_pages.clone(),
                    total_results: clone.total_results.clone(),
                }),
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
