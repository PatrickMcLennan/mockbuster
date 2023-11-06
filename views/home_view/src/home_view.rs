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
pub fn hydrate_home_view() -> Result<(), JsValue> {
    yew::Renderer::<Home>::new().hydrate();
    Ok(())
}
