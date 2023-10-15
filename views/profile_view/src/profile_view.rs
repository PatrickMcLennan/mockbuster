use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::Header::Header;

#[function_component(Profile)]
pub fn profile_view() -> Html {
    html! {
        <>
            <Header />
            <div class="container row">
                <div class="col-6 offset-3">
                    <h1>{"This is the Profile page"}</h1>
                </div>
            </div>
        </>
    }
}

#[wasm_bindgen]
pub fn run_profile_view() -> Result<(), JsValue> {
    yew::Renderer::<Profile>::new().hydrate();
    Ok(())
}
