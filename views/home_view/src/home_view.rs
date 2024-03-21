use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::{frame::Frame, header::Header, page_title::PageTitle, sidebar::CurrentRoute};

#[function_component(Home)]
pub fn home_view() -> Html {
    html! {
        <>
            <Header />
            <Frame current_route={CurrentRoute::Home}>
                <PageTitle
                    h1={"mockbuster"}
                    h2={"Find your next movie"}
                />
            </Frame>
        </>
    }
}

#[wasm_bindgen]
pub fn hydrate_home_view() -> Result<(), JsValue> {
    yew::Renderer::<Home>::new().hydrate();
    Ok(())
}
