use serde::{Deserialize, Serialize};
use yew::Properties;

#[derive(Debug, Deserialize, Serialize, PartialEq, Properties, Clone)]
pub struct MovieIdResult {
    pub adult: bool,
    pub backdrop_path: String,
    pub belongs_to_collection: Option<BelongsToCollection>,
    pub budget: i32,
    pub genres: Vec<Genre>,
    pub homepage: String,
    pub id: i32,
    pub imdb_id: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: String,
    pub production_companies: Vec<ProductionCompany>,
    pub production_countries: Vec<ProductionCountry>,
    pub release_date: String,
    pub revenue: i32,
    pub runtime: i32,
    pub spoken_languages: Vec<SpokenLanguage>,
    pub status: String,
    pub tagline: String,
    pub title: String,
    pub video: bool,
    pub vote_average: f64,
    pub vote_count: i32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Properties, Clone)]
pub struct BelongsToCollection {
    pub id: i32,
    pub name: String,
    pub poster_path: String,
    pub backdrop_path: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Properties, Clone)]
pub struct Genre {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Properties, Clone)]
pub struct ProductionCompany {
    pub id: i32,
    pub logo_path: Option<String>,
    pub name: String,
    pub origin_country: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Properties, Clone)]
pub struct ProductionCountry {
    pub iso_3166_1: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Properties, Clone)]
pub struct SpokenLanguage {
    pub english_name: String,
    pub iso_639_1: String,
    pub name: String,
}
