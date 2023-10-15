use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::Header::Header;

#[function_component(RecentlyRented)]
pub fn recently_rented_view() -> Html {
    html! {
        <>
            <Header />
            <div class="container row">
                <div class="col-6 offset-3">
                    <h1>{"This is the Recently Rented"}</h1>
                </div>
            </div>
        </>
    }
}

#[wasm_bindgen]
pub fn run_recently_rented_view() -> Result<(), JsValue> {
    yew::Renderer::<RecentlyRented>::new().hydrate();
    Ok(())
}
