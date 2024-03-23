#[cfg(feature = "ssr")]
use models::generated::aggregate_ratings;

use components::{
    frame::Frame, header::Header, page_title::PageTitle, rating_bar::RatingBar,
    sidebar::CurrentRoute,
};
use models::{
    stubs::{
        aggregate_ratings::AggregateRating as AggregateRatingsStub, rating::Rating as RatingStub,
    },
    tmdb_movies::movie_id_result::MovieIdResult,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub movies: Option<Vec<(AggregateRatingsStub, Option<MovieIdResult>)>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub movies: Option<Vec<(aggregate_ratings::Model, Option<MovieIdResult>)>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct State {
    pub movies: Vec<(aggregate_ratings::Model, Option<MovieIdResult>)>,
}

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct State {
    pub movies: Vec<(AggregateRatingsStub, Option<MovieIdResult>)>,
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {
    let state = use_prepared_state!((), move |_| -> State {
        let props_clone = props.clone();
        State {
            movies: match props_clone.movies {
                Some(v) => v,
                None => vec![],
            },
        }
    })?
    .unwrap();

    Ok(html! {
        <>
            <Header />
            <Frame current_route={CurrentRoute::TopRated}>
                <PageTitle
                    h1={"Top Rated"}
                    h2={"Our favourite movies"}
                />
                <section class="row g-3 container">
                    {
                        state
                            .movies
                            .clone()
                            .into_iter()
                            .map(|result| {
                                let aggregate_rating = result.0;
                                let tmdb_result = result.1.unwrap();
                                let image = format!("https://image.tmdb.org/t/p/w300{}", tmdb_result.poster_path);
                                html! {
                                    <div class="col-sm-12 col-md-6 col-lg-4">
                                        <div class="card border-dark">
                                            <figure class="row g-0 mb-0">
                                                <div class="col-4">
                                                    <a class="block" href={format!("/movie/{}", tmdb_result.id)}>
                                                        <img
                                                            alt={format!("{} poster", tmdb_result.title)}
                                                            class="img-fluid rounded-start"
                                                            src={image}
                                                            style="aspect-ratio: 2/3; width: 100%; height: auto; max-height: 168px;"
                                                        />
                                                    </a>
                                                </div>
                                                <figcaption class="col-8">
                                                    <div class="card-body">
                                                        <a href={format!("/movie/{}", tmdb_result.id)} style="-webkit-line-clamp: 3;">
                                                            <h5 class="card-title h6 mb-0" style="display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 2; overflow: hidden;">{tmdb_result.title.to_string()}</h5>
                                                        </a>
                                                        <div class="mb-0">
                                                            <div>
                                                                <RatingBar score={aggregate_rating.score} />
                                                            </div>
                                                        </div>
                                                    </div>
                                                </figcaption>
                                            </figure>
                                        </div>
                                    </div>
                                }
                            })
                            .collect::<Html>()
                    }
                </section>
            </Frame>
        </>
    })
}

#[function_component(TopTen)]
pub fn top_ten_view(props: &Props) -> Html {
    let props_clone = props.clone();
    html! {
        <Suspense fallback={ html! { <div>{"Loading..."}</div> } }>
            <Content movies={props_clone.movies} />
        </Suspense>
    }
}

#[wasm_bindgen]
pub fn hydrate_top_ten_view() -> Result<(), JsValue> {
    yew::Renderer::<TopTen>::with_props(Props { movies: None }).hydrate();
    Ok(())
}
