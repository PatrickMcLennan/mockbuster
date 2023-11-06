use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::Header::Header;

#[function_component(TopTen)]
pub fn top_ten_view() -> Html {
    html! {
        <>
            <Header />
            <div class="container">
                <h1>{"Top 10"}</h1>
            </div>
        </>
    }
}

#[wasm_bindgen]
pub fn hydrate_top_ten_view() -> Result<(), JsValue> {
    yew::Renderer::<TopTen>::new().hydrate();
    Ok(())
}
