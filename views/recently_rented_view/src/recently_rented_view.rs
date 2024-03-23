#[cfg(feature = "ssr")]
use models::generated::{ratings, users};

use components::{
    frame::Frame, header::Header, page_title::PageTitle, pagination::Pagination,
    rating_bar::RatingBar, sidebar::CurrentRoute,
};
use models::{
    stubs::{rating::Rating as RatingStub, user::User as UserStub},
    tmdb_movies::movie_id_result::MovieIdResult,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsValue};

// use crono
use yew::prelude::*;

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct Props {
    pub results: Option<Vec<(RatingStub, Option<UserStub>, Option<MovieIdResult>)>>,
    pub total_pages: Option<u64>,
    pub current_page: Option<u64>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub results: Option<Vec<(ratings::Model, Option<users::Model>, Option<MovieIdResult>)>>,
    pub total_pages: Option<u64>,
    pub current_page: Option<u64>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct State {
    pub results: Vec<(ratings::Model, Option<users::Model>, Option<MovieIdResult>)>,
    pub total_pages: u64,
    pub current_page: u64,
}

#[cfg(not(feature = "ssr"))]
#[derive(Clone, Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct State {
    results: Vec<(RatingStub, Option<UserStub>, Option<MovieIdResult>)>,
    pub total_pages: u64,
    pub current_page: u64,
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {
    let state = use_prepared_state!((), move |_| -> State {
        State {
            results: props.results.as_ref().unwrap().clone(),
            total_pages: match props.total_pages {
                Some(v) => match v {
                    1..=10 => v,
                    _ => 1,
                },
                None => 1,
            },
            current_page: match props.current_page {
                Some(v) => v,
                None => 1,
            },
        }
    })?
    .unwrap();

    let show_pagination = &state.total_pages > &1;
    let current_page = state.current_page;
    let total_pages = state.total_pages;

    Ok(html! {
        <>
            <Header />
            <Frame current_route={CurrentRoute::RecentlyRented}>
                <PageTitle
                    h1={"Recently Rented".to_string()}
                    h2={"See what your friends are watching".to_string()}
                />
                <section class="row g-3 container">
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
                {if show_pagination {
                    html! {
                        <Pagination
                            current_page={current_page as i64}
                            previous_url={format!("/recently-rented?page={}", current_page - 1)}
                            next_url={format!("/recently-rented?page={}", current_page + 1)}
                            base_url={format!("/recently-rented?")}
                            total_pages={total_pages as i64}
                        />
                    }
                } else { html! { <></> } }}
            </Frame>
        </>
    })
}

#[function_component(RecentlyRented)]
pub fn recently_rented_view(props: &Props) -> Html {
    let props_clone = props.clone();
    html! {
        <Suspense fallback={ html! { <div>{"Loading..."}</div> } }>
            <Content current_page={props_clone.current_page} results={props_clone.results.clone()} total_pages={props_clone.total_pages} />
        </Suspense>
    }
}

#[wasm_bindgen]
pub fn hydrate_recently_rented_view() -> Result<(), JsValue> {
    yew::Renderer::<RecentlyRented>::with_props(Props {
        current_page: None,
        results: None,
        total_pages: None,
    })
    .hydrate();
    Ok(())
}
