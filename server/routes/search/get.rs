use actix_web::{
    get,
    web::{Data, Query},
    Error as ActixError, HttpResponse,
};
use models::generated::ratings;
use models::tmdb_movies::movie_search_result::{Movie, MovieSearchResults, Rating};
use sea_orm::{prelude::*, DatabaseConnection};
use search_view::search_view::{Props, Search};
use serde_json::json;
use std::collections::HashMap;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;
use validators::tmdb_movies::search_dto::SearchDTO;

use crate::operations::tmdb_movies;
use crate::utils::document::{Document, DocumentProps};

#[get("/search")]
async fn get(
    params: Query<SearchDTO>,
    http_client: Data<reqwest_middleware::ClientWithMiddleware>,
    db: Data<DatabaseConnection>,
) -> Result<HttpResponse, ActixError> {
    // Pass params to tmdb search, get postgres entries for all tmdb results

    let tmdb_search_results = match tmdb_movies::search::execute(
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
        .filter(ratings::Column::TmdbId.is_in(ids.clone()))
        .all(&db.get_ref().clone())
        .await
    {
        Ok(v) => {
            let mut hash_map: HashMap<i32, Vec<Rating>> = HashMap::new();
            for rating in v {
                hash_map
                    .entry(rating.tmdb_id)
                    .or_insert_with(Vec::new)
                    .push(Rating {
                        id: rating.id.clone(),
                        user_id: rating.user_id.clone(),
                        score: rating.score.clone(),
                        tmdb_id: rating.tmdb_id.clone(),
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
        .body(
            Document::new(DocumentProps {
                wasm_assets: "searchView.js".to_string(),
                title: "Search".to_string(),
                content,
            })
        )
    )
}
