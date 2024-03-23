#[cfg(feature = "ssr")]
use models::generated::{ratings, users};

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::{frame::Frame, header::Header, page_title::PageTitle};
use serde::{Deserialize, Serialize};

use models::{
    stubs::{
        aggregate_ratings::AggregateRating as AggregateRatingsStub,
        comment::Comment as CommentStub, rating::Rating as RatingStub, user::User as UserStub,
    },
    tmdb_movies::movie_id_result::MovieIdResult,
};

#[cfg(not(feature = "ssr"))]
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub recent_ratings: Option<Vec<RatingStub>>,
    pub profile: Option<UserStub>,
}

#[cfg(feature = "ssr")]
#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub recent_ratings: Option<Vec<ratings::Model>>,
    pub profile: Option<users::Model>,
}

#[cfg(not(feature = "ssr"))]
#[derive(Properties, PartialEq, Clone, Deserialize, Serialize)]
pub struct State {
    pub recent_ratings: Option<Vec<RatingStub>>,
    pub profile: Option<UserStub>,
}

#[cfg(feature = "ssr")]
#[derive(Properties, PartialEq, Clone, Deserialize, Serialize)]
pub struct State {
    pub recent_ratings: Option<Vec<ratings::Model>>,
    pub profile: Option<users::Model>,
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {
    let state = use_prepared_state!((), move |_| -> State {
        let props_clone = props.clone();
        State {
            recent_ratings: props_clone.recent_ratings,
            profile: props_clone.profile,
        }
    })?
    .unwrap();

    let profile = &state.profile.clone().unwrap();

    Ok(html! {
        <>
            <Header />
            <Frame current_route={None}>
                <PageTitle
                    h1={"Profile".to_string()}
                    h2={format!("{} {}", &profile.first_name, &profile.last_name)}
                />
                <div class="container">

                </div>
            </Frame>
        </>
    })
}

#[function_component(Profile)]
pub fn profile_view(props: &Props) -> Html {
    let props_clone = props.clone();
    html! {
        <Suspense fallback={ html! { <div>{"Loading..."}</div> } }>
            <Content recent_ratings={props_clone.recent_ratings} profile={props_clone.profile} />
        </Suspense>
    }
}

#[wasm_bindgen]
pub fn hydrate_profile_view() -> Result<(), JsValue> {
    yew::Renderer::<Profile>::with_props(Props {
        recent_ratings: None,
        profile: None,
    })
    .hydrate();
    Ok(())
}
