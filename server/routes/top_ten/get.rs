use crate::operations::{aggregate_ratings as aggregate_ratings_operations, tmdb_movies};
use crate::utils::document::{Document, DocumentProps};
use actix_web::{get, web::Data, Error as ActixError, HttpResponse};
use models::{
    generated::aggregate_ratings as aggregate_ratings_model,
    tmdb_movies::movie_id_result::MovieIdResult,
};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

use top_ten_view::top_ten_view::{Props, TopTen};

#[get("/top-10")]
async fn get(
    db: Data<DatabaseConnection>,
    http_client: Data<reqwest_middleware::ClientWithMiddleware>,
) -> Result<HttpResponse, ActixError> {
    let top_movies = match aggregate_ratings_operations::list::execute(db.get_ref().clone()).await {
        Ok(v) => v,
        Err(e) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let mut unique_movies = HashMap::new();
    let movie_ids = top_movies
        .clone()
        .into_iter()
        .filter_map(|model| {
            let tmdb_id = model.tmdb_id;
            if unique_movies.contains_key(&tmdb_id) {
                None
            } else {
                unique_movies.insert(tmdb_id, None);
                Some(tmdb_id)
            }
        })
        .collect::<Vec<i32>>();

    let mut tmdb_futures = vec![];
    for tmdb_id in movie_ids {
        let clone = http_client.clone();
        let future = async move {
            tmdb_movies::fetch::execute(tmdb_id as u32, Some(clone.as_ref().clone())).await
        };
        tmdb_futures.push(future)
    }

    let tmdb_results = tokio::join!(async {
        let mut result_vec = Vec::new();
        for future in tmdb_futures {
            result_vec.push(future.await);
        }
        result_vec
    })
    .0;

    for resolved_future in tmdb_results {
        match resolved_future {
            Ok(v) => {
                unique_movies.insert(v.id, Some(v.clone()));
                // unique_movies.(v.id, Some(v));
                ()
            }
            Err(_) => (),
        };
    }

    let movies = top_movies
        .into_iter()
        .map(|model| {
            (
                model.clone(),
                unique_movies.get(&model.tmdb_id.clone()).unwrap().clone(),
            )
        })
        .collect::<Vec<(aggregate_ratings_model::Model, Option<MovieIdResult>)>>();

    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<TopTen>::with_props(move || Props {
                movies: Some(movies),
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
            wasm_assets: "topTenView.js".to_string(),
            title: "Top rated".to_string(),
            content,
        })))
}
