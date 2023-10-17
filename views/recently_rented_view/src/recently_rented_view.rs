use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::Header::Header;

#[function_component(RecentlyRented)]
pub fn recently_rented_view() -> Html {
    html! {
        <>
            <Header />
            <div class="container">
                <h1>{"Recently Rented"}</h1>
            </div>
        </>
    }
}

#[wasm_bindgen]
pub fn run_recently_rented_view_wasm() -> Result<(), JsValue> {
    yew::Renderer::<RecentlyRented>::new().hydrate();
    Ok(())
}
