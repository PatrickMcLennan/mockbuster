use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::Header::Header;

#[function_component(Home)]
pub fn home_view() -> Html {
    html! {
		<>
			<Header />
			<div class="container row">
				<div class="col-6 offset-3">
					<h1>{"This is the home page"}</h1>
				</div>
			</div>
		</>
    }
}

#[wasm_bindgen]
pub fn run_home_view() -> Result<(), JsValue> {
    yew::Renderer::<Home>::new().hydrate();
    Ok(())
}
