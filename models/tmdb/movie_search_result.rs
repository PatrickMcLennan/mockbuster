use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Debug, Deserialize, Serialize, PartialEq, Properties, Clone)]
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

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct MovieSearchResults {
    pub page: i32,
    pub results: Vec<MovieResult>,
    pub total_pages: i32,
    pub total_results: i32,
}
