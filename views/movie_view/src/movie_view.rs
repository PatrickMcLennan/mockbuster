#[cfg(feature = "ssr")]
use models::generated::{aggregate_ratings, comments, ratings, users};

use crate::_components::{scores_card::ScoresCard, stats_card::StatsCard};
use components::{
    comment::Comment,
    comment_entry::CommentEntry,
    frame::Frame,
    header::Header,
    page_title::PageTitle,
    rating_bar::{Props as RatingBarProps, RatingBar},
    vote_modal::VoteModal,
};
use models::{
    stubs::{
        aggregate_ratings::AggregateRating as AggregateRatingsStub,
        comment::Comment as CommentStub, rating::Rating as RatingStub, user::User as UserStub,
    },
    tmdb_movies::movie_id_result::MovieIdResult,
};
use num_format::{Locale, ToFormattedString};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub aggregate_rating: Option<AggregateRatingsStub>,
    pub comments: Option<Vec<(CommentStub, Option<UserStub>)>>,
    pub movie: Option<MovieIdResult>,
    pub alert_styles: Option<String>,
    pub alert_copy: Option<String>,
    pub user_score: Option<f32>,
    pub user_rated_date: Option<String>,
    pub ratings: Option<Vec<(RatingStub, Option<UserStub>)>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub aggregate_rating: Option<aggregate_ratings::Model>,
    pub comments: Option<Vec<(comments::Model, Option<users::Model>)>>,
    pub movie: Option<MovieIdResult>,
    pub alert_styles: Option<String>,
    pub alert_copy: Option<String>,
    pub user_score: Option<f32>,
    pub user_rated_date: Option<String>,
    pub ratings: Option<Vec<(ratings::Model, Option<users::Model>)>>,
}

#[cfg(not(feature = "ssr"))]
#[derive(Properties, PartialEq, Deserialize, Serialize)]
pub struct State {
    pub aggregate_rating: Option<AggregateRatingsStub>,
    pub comments: Vec<(CommentStub, Option<UserStub>)>,
    pub movie: MovieIdResult,
    pub alert_styles: Option<String>,
    pub alert_copy: Option<String>,
    pub user_score: Option<f32>,
    pub user_rated_date: Option<String>,
    pub ratings: Vec<(RatingStub, Option<UserStub>)>,
}

#[cfg(feature = "ssr")]
#[derive(Properties, PartialEq, Deserialize, Serialize)]
pub struct State {
    pub aggregate_rating: Option<aggregate_ratings::Model>,
    pub comments: Vec<(comments::Model, Option<users::Model>)>,
    pub movie: MovieIdResult,
    pub alert_styles: Option<String>,
    pub alert_copy: Option<String>,
    pub user_score: Option<f32>,
    pub user_rated_date: Option<String>,
    pub ratings: Vec<(ratings::Model, Option<users::Model>)>,
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {
    let comment = use_state(|| String::new());
    let state = use_prepared_state!((), move |_| -> State {
        let props_clone = props.clone();
        State {
            aggregate_rating: props_clone.aggregate_rating,
            comments: match props_clone.comments {
                Some(v) => v,
                None => vec![],
            },
            movie: props_clone.movie.as_ref().unwrap().clone(),
            alert_styles: props_clone.alert_styles,
            alert_copy: props_clone.alert_copy,
            user_score: props_clone.user_score,
            user_rated_date: props_clone.user_rated_date,
            ratings: match props_clone.ratings {
                Some(ratings) => ratings,
                None => vec![],
            },
        }
    })?
    .unwrap();
    let vote_modal_open = use_state(|| false);
    let vote_modal_open_clone = vote_modal_open.clone();

    let backdrop = format!(
        "https://image.tmdb.org/t/p/w500{}",
        match &state.movie.backdrop_path {
            Some(v) => v.to_string(),
            None => String::new(),
        }
    );

    let toggle_modal = {
        let clone = vote_modal_open.clone();
        Callback::from(move |_: MouseEvent| vote_modal_open.set(!*clone))
    };

    let poster = format!("https://image.tmdb.org/t/p/w500{}", state.movie.poster_path);
    let title = &state.movie.title;
    let tagline = &state.movie.tagline;

    let has_not_rated = state.user_score == None;
    let has_user_ratings = state.ratings.len() >= 1;

    let scores_card_props = state
        .ratings
        .clone()
        .into_iter()
        .map(|rating| RatingBarProps {
            score: rating.0.score,
            date: Some(rating.0.created_at.format("%d-%m-%Y").to_string()),
        })
        .collect::<Vec<RatingBarProps>>();

    Ok(html! {
        <>
            <Header />
                <Frame current_route={None}>
                    <PageTitle
                        h1={title.clone()}
                        h2={tagline.clone()}
                        alert={match state.alert_copy.clone() {
                            Some(v) => html! {
                                <div class={format!("alert alert-{} fade show mt-2 mb-4", state.alert_styles.clone().unwrap())} role="alert">
                                    <h5 class="mb-0">{v}</h5>
                                </div>
                            },
                            None => html! { <></> }
                        }}
                        background_path={backdrop}
                        cta={match state.user_score {
                            Some(_) => None,
                            None => {
                                Some(html! {
                                    <button
                                        onclick={toggle_modal}
                                        class="btn btn-primary ml-auto"
                                        data-bs-toggle="modal"
                                        data-bs-target={format!("#vote-modal-{}", state.movie.id)}
                                        type="button"
                                    >
                                        {"Rate"}
                                    </button>
                                })
                            }
                        }}
                        rating={match state.aggregate_rating.clone() {
                            Some(score) => html!{ <RatingBar score={score.score} /> },
                            None => html! { <></> }
                        }}
                        poster_path={poster}
                    />
                    <div class="container">
                        <div class="row g-3 mt-4">
                            <article class={classes!(if has_user_ratings { "col-sm-12 col-lg-8" } else { "col-12" } )}>
                                <div class="row g-2 mb-2">
                                    <StatsCard header={"Released".to_string()} copy={state.movie.release_date.to_string()} />
                                    <StatsCard
                                        header={"Budget".to_string()}
                                        copy={
                                            match &state.movie.budget {
                                                0 => "/".to_string(),
                                                _ => format!("${}", state.movie.budget.to_formatted_string(&Locale::en).to_string())
                                            }
                                        }
                                    />
                                    <StatsCard header={"Runtime".to_string()} copy={format!("{} mins", &state.movie.runtime)} />
                                </div>
                                <div class="row g-2 mb-2">
                                    <div class="col-12">
                                        <section class="card">
                                            <header class="card-header">
                                                <strong>{"Overview"}</strong>
                                            </header>
                                            <div class="card-body">
                                                <p class="card-text" style="text-indent: 2rem; text-wrap: pretty;">{&state.movie.overview}</p>
                                            </div>
                                        </section>
                                    </div>
                                </div>
                                <div class="row g-2 mb-2">
                                    <div class="col-10 mx-auto">
                                        <ol class="list-group list-group-flush">
                                            {
                                                state
                                                    .comments
                                                    .clone()
                                                    .into_iter()
                                                    .map(|comment| {
                                                        let user = comment.1.unwrap();
                                                        let user_name = format!("{} {}", user.first_name, user.last_name);
                                                        let created_at = comment.0.created_at.format("%d-%m-%Y").to_string();
                                                        html! {
                                                            <li class="list-group-item">
                                                                <Comment
                                                                    comment={comment.0.content.to_string()}
                                                                    user_id={user.id}
                                                                    user_name={user_name}
                                                                    created_at={created_at}
                                                                />
                                                            </li>
                                                        }
                                                    })
                                                    .collect::<Html>()
                                            }
                                        </ol>
                                    </div>
                                </div>
                                <div class="row g-2 mb-2">
                                    <div class="col-12">
                                        <form method="POST" class="card">
                                            <legend class="card-header" style="font-size: 1rem;">
                                                <strong>{"Leave a comment"}</strong>
                                            </legend>
                                            <div class="card-body">
                                                <CommentEntry comment={comment} />
                                            </div>
                                            <fieldset class="card-footer">
                                                <button type="submit" class="btn btn-primary">{"Submit Comment"}</button>
                                            </fieldset>
                                        </form>
                                    </div>
                                </div>
                            </article>
                            {if has_user_ratings {
                                html! {
                                    <ScoresCard scores={scores_card_props} />
                                }
                            } else { html! { <></> } }}
                        </div>
                    </div>
                    {if has_not_rated {
                        html! {
                            <VoteModal
                                title={state.movie.title.to_string()}
                                id={state.movie.id}
                                open={*vote_modal_open_clone}
                            />
                        }
                    } else { html! { <></> } }}
            </Frame>
        </>
    })
}

#[function_component(Movie)]
pub fn movie_view(props: &Props) -> Html {
    html! {
        <Suspense fallback={ html! { <div>{"Loading..."}</div> } }>
            <Content
                aggregate_rating={props.aggregate_rating.clone()}
                comments={props.comments.clone()}
                movie={props.movie.clone()}
                alert_copy={props.alert_copy.clone()}
                alert_styles={props.alert_styles.clone()}
                user_score={props.user_score.clone()}
                user_rated_date={props.user_rated_date.clone()}
                ratings={props.ratings.clone()}

            />
        </Suspense>
    }
}

#[wasm_bindgen]
pub fn hydrate_movie_view() -> Result<(), JsValue> {
    yew::Renderer::<Movie>::with_props(Props {
        aggregate_rating: None,
        comments: None,
        movie: None,
        alert_styles: None,
        alert_copy: None,
        user_score: None,
        user_rated_date: None,
        ratings: None,
    })
    .hydrate();
    Ok(())
}
