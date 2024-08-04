#[cfg(feature = "ssr")]
use models::{
    events_list_result::EventsListResult,
    generated::{comments, ratings, users},
};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use models::{
    stubs::{
        comment::Comment as CommentStub,
        events_list_result::EventsListResult as EventsListResultStub, rating::Rating as RatingStub,
        user::User as UserStub,
    },
    tmdb_movies::movie_id_result::MovieIdResult,
};

use components::{frame::Frame, header::Header, page_title::PageTitle, sidebar::CurrentRoute};

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
