use actix_web::{
    get,
    web::{Data, Query},
    Error as ActixError, HttpResponse,
};
use models::{
    generated::{ratings as ratings_model, users as users_model},
    tmdb_movies::movie_id_result::MovieIdResult,
};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};
use std::collections::HashMap;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

use crate::operations::{ratings, tmdb_movies};
use crate::utils::document::{Document, DocumentProps};
use recently_rented_view::recently_rented_view::{Props, RecentlyRented};
use validators::ratings::recently_rented_dto::RecentlyRentedDTO;

const PAGE_SIZE: u64 = 10;

#[get("/recently-rented")]
async fn get(
    db: Data<DatabaseConnection>,
    params: Query<RecentlyRentedDTO>,
    http_client: Data<reqwest_middleware::ClientWithMiddleware>,
) -> Result<HttpResponse, ActixError> {
    let page = match params.page {
        Some(v) => match v.to_string().parse::<u64>() {
            Ok(page) => page,
            Err(_) => 1,
        },
        None => 1,
    };

    println!("{:?}", params);

    let start_cursor = (page - 1) * PAGE_SIZE;
    let end_cursor = page * PAGE_SIZE;

    let recently_rented_op =
        match ratings::list::execute(start_cursor, end_cursor, db.get_ref().clone()).await {
            Ok(v) => v,
            Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
        };

    let total_pages = match ratings::count::execute(db.get_ref().clone()).await {
        Ok(v) => {
            let truncated = v / PAGE_SIZE;
            if truncated > 10 {
                10
            } else {
                truncated
            }
        }
        Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
    };

    let mut recently_rented_hashmap: HashMap<i32, Option<MovieIdResult>> = HashMap::new();
    for rating in &recently_rented_op {
        let id = rating.0.tmdb_id;
        recently_rented_hashmap.insert(id, None);
    }

    let mut tmdb_futures = vec![];
    for tmdb_id in recently_rented_hashmap.clone().into_keys() {
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
                recently_rented_hashmap.insert(v.id, Some(v));
                ()
            }
            Err(_) => (),
        };
    }

    let recently_rented = recently_rented_op
        .into_iter()
        .map(|r_r| {
            let id = r_r.0.tmdb_id.clone();
            (
                r_r.0.clone(),
                r_r.1.clone(),
                recently_rented_hashmap.get(&id).unwrap().clone(),
            )
        })
        .collect::<Vec<(
            ratings_model::Model,
            Option<users_model::Model>,
            Option<MovieIdResult>,
        )>>();

    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<RecentlyRented>::with_props(move || Props {
                results: Some(recently_rented),
                total_pages: Some(total_pages),
                current_page: Some(page),
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
            description: "See what people are watching".to_string(),
            wasm_assets: "recentlyRentedView.js".to_string(),
            title: "Recently Rented".to_string(),
            content,
        })))
}
