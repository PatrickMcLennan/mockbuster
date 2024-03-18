use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::header::Header;

#[function_component(Home)]
pub fn home_view() -> Html {
    html! {
        <>
            <Header />
            <main class="container">
                <header class="border-bottom mb-4 pt-2 pb-4">
                    <h1>{"mockbuster"}</h1>
                    <h2 class="mb-0">{"Find your next movie"}</h2>
                </header>
            </main>
        </>
    }
}

#[wasm_bindgen]
pub fn hydrate_home_view() -> Result<(), JsValue> {
    yew::Renderer::<Home>::new().hydrate();
    Ok(())
}
