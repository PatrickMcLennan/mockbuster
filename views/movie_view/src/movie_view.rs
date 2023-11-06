use models::tmdb::movie_id_result::MovieIdResult;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::Header::Header;

#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub movie: Option<MovieIdResult>,
}

#[derive(Properties, PartialEq, Deserialize, Serialize)]
pub struct State {
    pub movie: MovieIdResult,
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {
    let state = use_prepared_state!((), move |_| -> State {
        State {
            movie: props.movie.as_ref().unwrap().clone(),
        }
    })?
    .unwrap();

    Ok(html! {
        <>
            <Header />
            <main class="container">
                <h1>{state.movie.title.to_string()}</h1>
            </main>
        </>
    })
}

#[function_component(Movie)]
pub fn movie_view(props: &Props) -> Html {
    html! {
        <Suspense fallback={ html! { <div>{"Loading..."}</div> } }>
            <Content movie={props.movie.clone()} />
        </Suspense>
    }
}

#[wasm_bindgen]
pub fn hydrate_movie_view() -> Result<(), JsValue> {
    yew::Renderer::<Movie>::with_props(Props { movie: None }).hydrate();
    Ok(())
}
