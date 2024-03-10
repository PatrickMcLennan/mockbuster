#[cfg(feature = "ssr")]
use models::{generated::{ratings, users}};

use models::stubs::{rating::Rating as RatingStub, user::User as UserStub};
use components::Header::Header;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::console;
use yew::prelude::*;

#[cfg(not(feature = "ssr"))]
#[derive(Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct Props {
    pub results: Option<Vec<(RatingStub, Option<UserStub>)>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub results: Option<Vec<(ratings::Model, Option<users::Model>)>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct State {
    pub results: Vec<(ratings::Model, Option<users::Model>)>,
}

#[cfg(not(feature = "ssr"))]
#[derive(Clone, Debug, Deserialize, PartialEq, Properties, Serialize)]
pub struct State {
    results: Vec<(RatingStub, Option<UserStub>)>,
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {

    #[cfg(not(feature = "ssr"))]
    console::log_1(&format!("Props: {:?}", props).into());

    let state = use_prepared_state!((), move |_| -> State {
        State {
            results: props.results.as_ref().unwrap().clone()
        }
    })?.unwrap();

    Ok(html! {
        <>
            <Header />
            <div class="container">
                <h1>{"Recently Rented"}</h1>
                {
                    state
                        .results
                        .clone()
                        .into_iter()
                        .map(|result| html! {
                            <p>{result.0.score}</p>
                        })
                        .collect::<Html>()
                }
            </div>
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
