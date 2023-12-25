use db_models::generated::ratings;
use models::tmdb::movie_search_result::TmdbSearchResults;
use sea_orm::{prelude::*, DatabaseBackend, DatabaseConnection, Statement};
use serde::{Deserialize, Serialize};
use validator::Validate;
use validators::search_dto::SearchDTO;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Validate, PartialEq)]
pub struct Pagination {
    #[validate(range(min = 1, message = "Page must be a value > 1"))]
    pub page: i64,
}

pub async fn get_recently_rented_movies(
    pagination: SearchDTO,
    db: DatabaseConnection,
) -> Result<(), String> {
    // ratings::Entity::find()
    // 	.order_by_asc(ratings::Column::CreatedAt);

    return Ok(());

    // match http_client
    //     .unwrap_or(reqwest::Client::new())
    //     .get(format!(
    //         "https://api.themoviedb.org/3/search/movie?page={}&api_key={}&query={}",
    //         dto.page, api_key, dto.query
    //     ))
    //     .send()
    //     .await
    // {
    //     Ok(res) => match res.json::<TmdbSearchResults>().await {
    //         Ok(v) => Ok(v),
    //         Err(e) => {
    //             println!("[ERROR -- search_tmdb_movies]: {:?}", e);
    //             return Err(five_hundo);
    //         }
    //     },
    //     Err(e) => {
    //         println!("[ERROR -- search_tmdb_movies]: {:?}", e);
    //         return Err(five_hundo);
    //     }
    // }
}
