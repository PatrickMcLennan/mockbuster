use models::tmdb_movies::movie_search_result::MovieResult;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub movie: MovieResult,
}

#[function_component(MovieCard)]
pub fn movie_card(props: &Props) -> Html {
    let movie = &props.movie;

    let image = match &movie.poster_path {
        Some(v) => format!("https://image.tmdb.org/t/p/w300{}", v),
        None => "https://www.google.com".to_string(),
    };

    html! {
        <div class="card border-dark">
            <div class="row g-0">
                <div class="col-4">
                    <a class="block" href={format!("/movie/{}", movie.id)}>
                        <img
                            alt={format!("{} poster", movie.title)}
                            class="img-fluid rounded-start"
                            src={image}
                            style="aspect-ratio: 2/3; width: 100%; height: auto;"
                        />
                    </a>
                </div>
                <div class="col-8">
                    <div class="card-body">
                        <a class="block" href={format!("/movie/{}", movie.id)} style="-webkit-line-clamp: 3;">
                            <h5 class="card-title h6" style="display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 3; overflow: hidden;">{movie.title.to_string()}</h5>
                        </a>
                    </div>
                </div>
            </div>
        </div>
        // <div class="card">
        //     <div class="card-header">
        //         <h3 class="card-title h5 mb-0">
        //             <a class="block" href={format!("/movie/{}", movie.id)}>
        //                 {&movie.title}
        //             </a>
        //         </h3>
        //         </div>
        //     <div class="card-body">
        //         <div class="row g-2">
        //             <div class="col-4">
        //                 <img class="img-fluid w-100" src={image} alt={format!("{} poster", movie.title)} />
        //             </div>
        //         <div class="col-8">
        //             <p>{overview}</p>
        //         </div>
        //     </div>
        //     </div>
        //     <div class="card-footer">
        //         <div class="row g-2">
        //             <div class="col-6">
        //                 {"Rating"}
        //             </div>
        //             <div class="col-6">
        //                 {"raters"}
        //             </div>
        //         </div>
        //     </div>
        // </div>
    }
}
