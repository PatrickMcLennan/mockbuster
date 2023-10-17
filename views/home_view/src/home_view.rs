use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::Header::Header;

#[function_component(Home)]
pub fn home_view() -> Html {
    html! {
        <>
            <Header />
            <div class="container">
                <h1>{"What's trending"}</h1>
            </div>
        </>
    }
}

#[wasm_bindgen]
pub fn run_home_view_wasm() -> Result<(), JsValue> {
    yew::Renderer::<Home>::new().hydrate();
    Ok(())
}
