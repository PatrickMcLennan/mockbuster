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
pub fn run_top_ten_view_wasm() -> Result<(), JsValue> {
    yew::Renderer::<TopTen>::new().hydrate();
    Ok(())
}
