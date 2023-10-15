use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::Header::Header;

#[function_component(TopTen)]
pub fn top_ten_view() -> Html {
    html! {
        <>
            <Header />
            <div class="container row">
                <div class="col-6 offset-3">
                    <h1>{"This is the Top 10"}</h1>
                </div>
            </div>
        </>
    }
}

#[wasm_bindgen]
pub fn run_top_ten_view() -> Result<(), JsValue> {
    yew::Renderer::<TopTen>::new().hydrate();
    Ok(())
}
