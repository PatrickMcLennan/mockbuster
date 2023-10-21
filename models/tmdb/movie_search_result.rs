use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieResult {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<i32>,
    pub id: i32,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    pub release_date: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i32,
}
