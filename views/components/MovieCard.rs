use yew::prelude::*;
use models::tmdb::movie_search_result::MovieResult;
use serde::{Deserialize, Serialize};

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
        <figure class="card">
			<img class="card-img-top" src={image} alt={format!("{} poster", movie.title)} />
			<figcaption class="card-body">
				<h4 class="card-title">{movie.title.to_string()}</h4>
				<p class="card-text">{movie.overview.to_string().truncate(150)}</p>
			</figcaption>
		</figure>
    }
}
