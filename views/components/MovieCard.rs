use models::tmdb::movie_search_result::MovieResult;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub movie: MovieResult,
}

#[function_component(MovieCard)]
pub fn movie_card(props: &Props) -> Html {
    let movie = &props.movie;
    let image = match movie.poster_path.clone() {
        Some(v) => format!("https://image.tmdb.org/t/p/w300{}", v),
        None => "https://www.google.com".to_string(),
    };

    html! {
        <a class="card position-relative overflow-hidden col-6 col-sm-4 col-lg-2" href={format!("/movie/{}", movie.id)}>
            <img class="card-img-top w-100" src={image} alt={format!("{} poster", movie.title)} />
            <div class="card-body position-absolute bottom-0 w-100 bg-dark">
                <h3 class="card-title h6">
                    <small class="text-white small">{movie.title.to_string()}</small>
                </h3>
            </div>
        </a>
    }
}
