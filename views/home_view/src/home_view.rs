#[cfg(feature = "ssr")]
use models::events_list_result::EventsListResult;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use models::stubs::events_list_result::EventsListResult as EventsListResultStub;

use components::{
    comment::{self, Comment},
    frame::Frame,
    header::Header,
    page_title::PageTitle,
    rating_bar::RatingBar,
    sidebar::CurrentRoute,
    user_badge::UserBadge,
};

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub events: Option<Vec<EventsListResultStub>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub events: Option<Vec<EventsListResult>>,
}

#[cfg(not(feature = "ssr"))]
#[derive(Properties, PartialEq, Deserialize, Serialize)]
pub struct State {
    pub events: Vec<EventsListResultStub>,
}

#[cfg(feature = "ssr")]
#[derive(Properties, PartialEq, Deserialize, Serialize)]
pub struct State {
    pub events: Vec<EventsListResult>,
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {
    let state = use_prepared_state!((), move |_| -> State {
        let props_clone = props.clone();
        State {
            events: match props_clone.events {
                Some(v) => v,
                None => vec![],
            },
        }
    })?
    .unwrap();

    Ok(html! {
        <>
            <Header />
            <Frame current_route={CurrentRoute::Home}>
                <PageTitle
                    h1={"mockbuster"}
                    h2={"Find your next movie"}
                />
                {if *&state.events.len() <= 0 {
                    html! {
                        <h3>{"No recent events were found; be the first to rate or comment on a movie!"}</h3>
                    }
                } else {
                    html! {
                        <section class="row g-3 container">
                            {
                                state
                                    .events
                                    .clone()
                                    .into_iter()
                                    .map(|event| {
                                        let has_comment = event.comment.is_some();
                                        let has_rating = event.rating.is_some();
                                        let has_movie = event.tmdb_movie.is_some();

                                        let tmdb_id = if has_movie {
                                            event.tmdb_movie.clone().unwrap().id
                                        } else if has_comment {
                                            event.comment.clone().unwrap().tmdb_id
                                        } else {
                                            event.rating.clone().unwrap().tmdb_id
                                        };

                                        let first_name = event.user.first_name;
                                        let user_id = event.user.id;

                                        let image = match event.tmdb_movie.clone() {
                                            Some(movie) => format!("https://image.tmdb.org/t/p/w300{}", movie.poster_path),
                                            None => "https://www.google.com".to_string(),
                                        };

                                        let movie_link = match event.tmdb_movie {
                                            Some(movie) => html!{
                                                <a href={format!("/movie/{}", movie.id)}>
                                                    {movie.title}
                                                </a>
                                             },
                                             None => html! {
                                                <a href={format!("/movie/{}", tmdb_id)}>
                                                    {"REDACTED"}
                                                </a>
                                             }
                                        };

                                        let comment_markup = if event.comment.is_some() {
                                            let comment = event.comment.unwrap();
                                            html! {
                                                <div class="container">
                                                    <Comment comment={comment.content} user_id={user_id} user_name={first_name.to_string()} created_at={comment.created_at.to_string()} />
                                                </div>
                                            }
                                        } else {
                                            html!{ <></> }
                                        };

                                        html! {
                                            <>
                                                <div class="col-1"></div>
                                                <div class="col-10">
                                                    <div class="card border-dark">
                                                        <figure class="row g-0 mb-0">
                                                            <div class="col-2">
                                                                <a class="block" href={format!("/movie/{}", tmdb_id)}>
                                                                    <img
                                                                        alt={format!("{} poster", image)}
                                                                        class="img-fluid rounded-start"
                                                                        src={image}
                                                                        style="aspect-ratio: 2/3; width: 100%; height: auto;"
                                                                    />
                                                                </a>
                                                            </div>
                                                            <figcaption class="col-9">
                                                                <div class="card-body d-flex flex-column justify-content-stretch align-items-start h-100">

                                                                    <h4 class="card-title h6 mb-0" style="display: -webkit-box; -webkit-box-orient: vertical; -webkit-line-clamp: 2; overflow: hidden;">
                                                                        <UserBadge user_name={first_name.to_string()} user_id={user_id} image_url={String::new()} />
                                                                        {if has_comment && has_rating {
                                                                            {" commented and rated "}
                                                                        } else if has_comment {
                                                                            {" commented on "}
                                                                        } else if has_rating {
                                                                            {" rated "}
                                                                        } else {
                                                                            {""}
                                                                        }}
                                                                        {movie_link}
                                                                    </h4>

                                                                    <div class="mt-auto mb-auto w-100">
                                                                        {if has_rating {
                                                                            let rating = event.rating.unwrap();
                                                                            html! {
                                                                                <>
                                                                                    <RatingBar score={rating.score} date={rating.created_at.to_string()} />
                                                                                    {comment_markup}
                                                                                </>
                                                                            }
                                                                        } else { comment_markup }}
                                                                    </div>

                                                                </div>
                                                            </figcaption>
                                                        </figure>
                                                    </div>
                                                </div>
                                                <div class="col-1"></div>
                                            </>
                                        }
                                    })
                                    .collect::<Html>()
                            }
                        </section>
                    }
                }}
            </Frame>
        </>
    })
}

#[function_component(Home)]
pub fn home_view(props: &Props) -> Html {
    html! {
        <Suspense fallback={ html! { <div>{"Loading..."}</div> } }>
            <Content events={props.events.clone()} />
        </Suspense>
    }
}

#[wasm_bindgen]
pub fn hydrate_home_view() -> Result<(), JsValue> {
    yew::Renderer::<Home>::with_props(Props { events: None }).hydrate();
    Ok(())
}
