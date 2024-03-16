use models::tmdb_movies::movie_search_result::TmdbSearchResults;
use validators::tmdb_movies::search_dto::SearchDTO;

pub async fn execute(
    dto: SearchDTO,
    http_client: Option<reqwest_middleware::ClientWithMiddleware>,
) -> Result<TmdbSearchResults, String> {
    let api_key = std::env::var("TMDB_API_KEY").expect("NO_TMDB_API_KEY_IN_ENV");

    let five_hundo =
        "Searching movies is unavailable at the moment; please try again later.".to_string();

    let tmdb_results = match http_client
        .unwrap_or(reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build())
        .get(format!(
            "https://api.themoviedb.org/3/search/movie?page={}&api_key={}&query={}",
            dto.page, api_key, dto.query
        ))
        .send()
        .await
    {
        Ok(res) => match res.json::<TmdbSearchResults>().await {
            Ok(v) => v,
            Err(e) => {
                println!("[ERROR -- search_tmdb_movies]: {:?}", e);
                return Err(five_hundo);
            }
        },
        Err(e) => {
            println!("[ERROR -- search_tmdb_movies]: {:?}", e);
            return Err(five_hundo);
        }
    };

    Ok(tmdb_results)
}
