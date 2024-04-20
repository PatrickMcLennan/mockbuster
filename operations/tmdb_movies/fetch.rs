use models::tmdb_movies::movie_id_result::MovieIdResult;
use reqwest::Client;
use reqwest_middleware::Error;

const LOG_KEY: &str = "[Operations::TmdbMovies::Fetch]: ";

pub async fn execute(
    id: u32,
    http_client: Option<reqwest_middleware::ClientWithMiddleware>,
) -> Result<MovieIdResult, Error> {
    let api_key = std::env::var("TMDB_API_KEY").expect("NO_TMDB_API_KEY_IN_ENV");
    let url = format!(
        "https://api.themoviedb.org/3/movie/{}?language=en-US&api_key={}",
        id, api_key
    );

    match http_client
        .unwrap_or(reqwest_middleware::ClientBuilder::new(Client::new()).build())
        .get(url.to_string())
        .send()
        .await
    {
        Ok(res) => match res.json::<MovieIdResult>().await {
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
