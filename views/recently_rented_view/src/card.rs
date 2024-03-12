use models::tmdb_movies::movie_search_result::MovieResult;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub movie: MovieResult,
}

#[function_component(Card)]
pub fn card(props: &Props) -> Html {
    let movie = &props.movie;
    let image = match &movie.poster_path {
        Some(v) => format!("https://image.tmdb.org/t/p/w300{}", v),
        None => "https://www.google.com".to_string(),
    };

    let overview = {
        let overview = &movie.overview.to_string();
        let string = overview.chars().take(150).collect::<String>();
        if overview.len() > string.len() {
            format!("{}...", string)
        } else {
            string
        }
    };

    html! {
        <div class="card">
            <div class="card-header">
                <h3 class="card-title h5 mb-0">
                    <a class="block" href={format!("/movie/{}", movie.id)}>
                        {&movie.title}
                    </a>
                </h3>
                </div>
            <div class="card-body">
                <div class="row g-2">
                    <div class="col-4">
                        <img class="img-fluid w-100" src={image} alt={format!("{} poster", movie.title)} />
                    </div>
                <div class="col-8">
                    <p>{overview}</p>
                </div>
            </div>
            </div>
            <div class="card-footer">
                <div class="row g-2">
                    <div class="col-6">
                        {"Our rating"}
                    </div>
                    <div class="col-6">
                        {"tmdb rating"}
                    </div>
                </div>
            </div>
        </div>
    }
}
