use models::tmdb::movie_id_result::MovieIdResult;

pub async fn get_tmdb_movie(
    id: u32,
    http_client: Option<reqwest::Client>,
) -> Result<MovieIdResult, String> {
    let api_key = std::env::var("TMDB_API_KEY").expect("NO_TMDB_API_KEY_IN_ENV");
    let five_hundo = "This movie is unavailable at the moment; please try again later.".to_string();
	let url = format!(
		"https://api.themoviedb.org/3/movie/{}?language=en-US&api_key={}",
		id, api_key
	);

	println!("{}", url);

    match http_client
        .unwrap_or(reqwest::Client::new())
        .get(url.to_string())
        .send()
        .await
    {
        Ok(res) => {
			println!("{:?}", res); 
			match res.json::<MovieIdResult>().await {
				Ok(v) => {
					println!("[SUCCESS -- get_tmdb_movie]: {:?}", v);
					return Ok(v);
				},
				Err(e) => {
					println!("[ERROR -- get_tmdb_movie res.json::<MovieIdResult>]: {:?}", e);
					return Err(five_hundo);
				}
			}
		},
        Err(e) => {
            println!("[ERROR -- get_tmdb_movie http_client.get({})]: {:?}", url, e);
            return Err(five_hundo);
        }
	}
}
