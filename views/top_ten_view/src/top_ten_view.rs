use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::Header::Header;

#[function_component(TopTen)]
pub fn top_ten_view() -> Html {
    html! {
        <>
            <Header />
            <main class="container">
                <header class="border-bottom mb-4 pt-2 pb-4">
                    <h1>{"Top 10"}</h1>
                    <h2 class="mb-0">{"Our favourite movies"}</h2>
                </header>
            </main>
        </>
    }
}

#[wasm_bindgen]
pub fn hydrate_top_ten_view() -> Result<(), JsValue> {
    yew::Renderer::<TopTen>::new().hydrate();
    Ok(())
}
