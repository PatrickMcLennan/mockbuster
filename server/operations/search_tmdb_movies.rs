use models::tmdb::movie_search_result::MovieSearchResults;
use validators::search_dto::SearchDTO;

pub async fn search_tmdb_movies(
    dto: SearchDTO,
    http_client: Option<reqwest::Client>,
) -> Result<MovieSearchResults, String> {
    let api_key = std::env::var("TMDB_API_KEY").expect("NO_TMDB_API_KEY_IN_ENV");

    let five_hundo =
        "Searching movies is unavailable at the moment; please try again later.".to_string();

    match http_client
        .unwrap_or(reqwest::Client::new())
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
