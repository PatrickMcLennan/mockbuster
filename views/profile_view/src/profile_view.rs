use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::Header::Header;

#[function_component(Profile)]
pub fn profile_view() -> Html {
    html! {
        <>
            <Header />
            <div class="container row">
                <h1>{"Profile"}</h1>
            </div>
        </>
    }
}

#[wasm_bindgen]
pub fn run_profile_view_wasm() -> Result<(), JsValue> {
    yew::Renderer::<Profile>::new().hydrate();
    Ok(())
}