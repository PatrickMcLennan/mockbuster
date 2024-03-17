#[cfg(feature = "ssr")]
use models::generated::{ratings, users};

use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use components::{rating_bar::RatingBar, Header::Header};
use models::{
    stubs::{rating::Rating as RatingStub, user::User as UserStub},
    tmdb_movies::movie_id_result::MovieIdResult,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsValue};
use web_sys::console;

// use crono
use yew::prelude::*;

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct Props {
    pub results: Option<Vec<(RatingStub, Option<UserStub>, Option<MovieIdResult>)>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub results: Option<Vec<(ratings::Model, Option<users::Model>, Option<MovieIdResult>)>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct State {
    pub results: Vec<(ratings::Model, Option<users::Model>, Option<MovieIdResult>)>,
}

#[cfg(not(feature = "ssr"))]
#[derive(Clone, Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct State {
    results: Vec<(RatingStub, Option<UserStub>, Option<MovieIdResult>)>,
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {
    let state = use_prepared_state!((), move |_| -> State {
        State {
            results: props.results.as_ref().unwrap().clone(),
        }
    })?
    .unwrap();

    Ok(html! {
        <>
            <Header />
            <main class="container">
                <header class="border-bottom mb-4 pt-2 pb-4">
                    <h1>{"Recently Rented"}</h1>
                    <h2 class="mb-0">{"See what your friends are watching"}</h2>
                </header>
                <section class="row g-3">
                    {
                        state
                            .results
                            .clone()
                            .into_iter()
                            .map(|result| {
                                let model = result.0;
                                let user = result.1.unwrap();
                                let tmdb = result.2.unwrap();

                                let image = format!("https://image.tmdb.org/t/p/w300{}", tmdb.poster_path);
                                let watched_at = model.created_at.format("%d-%m-%Y");

                                html! {
                                    <div class="col-sm-12 col-md-6 col-lg-4">
                                        <div class="card border-dark">
                                            <figure class="row g-0 mb-0">
                                                <div class="col-4">
                                                    <a class="block" href={format!("/movie/{}", tmdb.id)}>
                                                        <img
                                                            alt={format!("{} poster", tmdb.title)}
                                                            class="img-fluid rounded-start"
                                                            src={image}
                                                            style="aspect-ratio: 2/3; width: 100%; height: auto; max-height: 168px;"
                                                        />
                                                    </a>
                                                </div>
                                                <figcaption class="col-8">
                                                    <div class="card-body">
                                                        <a href={format!("/movie/{}", tmdb.id)} style="-webkit-line-clamp: 3;">
                                                            <h5 class="card-title h6 mb-0" style="display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 2; overflow: hidden;">{tmdb.title.to_string()}</h5>
                                                        </a>
                                                        <div class="mb-0">
                                                            <div>
                                                                <RatingBar score={model.score} date={watched_at.to_string()} />
                                                            </div>
                                                            <div class="mt-2 d-flex justify-content-between">
                                                                <span class="badge rounded-pill text-bg-secondary">
                                                                    {user.first_name.chars().nth(0).unwrap()}{user.last_name.chars().nth(0).unwrap()}
                                                                </span>
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
            </main>
        </>
    })
}

#[function_component(RecentlyRented)]
pub fn recently_rented_view(props: &Props) -> Html {
    html! {
        <Suspense fallback={ html! { <div>{"Loading..."}</div> } }>
            <Content results={props.results.clone()} />
        </Suspense>
    }
}

#[wasm_bindgen]
pub fn hydrate_recently_rented_view() -> Result<(), JsValue> {
    yew::Renderer::<RecentlyRented>::with_props(Props { results: None }).hydrate();
    Ok(())
}
