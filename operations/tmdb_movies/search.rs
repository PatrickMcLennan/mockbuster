use models::tmdb_movies::movie_search_result::TmdbSearchResults;
use validators::tmdb_movies::search_dto::SearchDTO;

use reqwest_middleware::Error;

const LOG_KEY: &str = "[Operations::TmdbMovies::Search]: ";

pub async fn execute(
    dto: SearchDTO,
    http_client: Option<reqwest_middleware::ClientWithMiddleware>,
) -> Result<TmdbSearchResults, Error> {
    let api_key = std::env::var("TMDB_API_KEY").expect("NO_TMDB_API_KEY_IN_ENV");

    match http_client
        .unwrap_or(reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build())
        .get(format!(
            "https://api.themoviedb.org/3/search/movie?page={}&api_key={}&query={}",
            dto.page, api_key, dto.query
        ))
        .send()
        .await
    {
        Ok(res) => match res.json::<TmdbSearchResults>().await {
            Ok(v) => Ok(v),
            Err(e) => {
                println!("{}{:?}", LOG_KEY, e);
                Err(Error::Reqwest(e))
            }
        },
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
