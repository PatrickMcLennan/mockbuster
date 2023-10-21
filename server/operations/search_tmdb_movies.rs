use models::tmdb::movie_search_result::MovieResult;
use serde::{Deserialize, Serialize};
use validators::search_dto::SearchDTO;

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieSearchResults {
    pub page: i32,
    pub results: Vec<MovieResult>,
    pub total_pages: i32,
    pub total_results: i32,
}

pub async fn search_tmdb_movies(
    dto: SearchDTO,
    http_client: Option<reqwest::Client>,
) -> Result<MovieSearchResults, String> {
    let api_key = std::env::var("TMDB_API_KEY").expect("NO_TMDB_API_KEY_IN_ENV");

    let five_hundo =
        "Searching movies is unavailable at the moment; please try again later.".to_string();

    let client = match http_client {
        Some(v) => v,
        None => reqwest::Client::new(),
    };

    match client
        .get(format!(
            "https://api.themoviedb.org/3/search/movie?page={}&api_key={}&query={}",
            dto.page, api_key, dto.query
        ))
        .send()
        .await
    {
        Ok(res) => match res.json::<MovieSearchResults>().await {
            Ok(v) => Ok(v),
            Err(e) => {
                println!("[ERROR -- search_tmdb_movies]: {:?}", e);
                return Err(five_hundo);
            }
        },
        Err(e) => {
            println!("[ERROR -- search_tmdb_movies]: {:?}", e);
            return Err(five_hundo);
        }
    }
}
