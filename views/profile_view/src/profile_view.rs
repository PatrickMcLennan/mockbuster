use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::header::Header;

#[function_component(Profile)]
pub fn profile_view() -> Html {
    html! {
        <>
            <Header />
            <div class="container">
                <h1>{"Profile"}</h1>
            </div>
        </>
    }
}

#[wasm_bindgen]
pub fn hydrate_profile_view() -> Result<(), JsValue> {
    yew::Renderer::<Profile>::new().hydrate();
    Ok(())
}
