use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Map};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::Header::Header;

#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub results: Option<Vec<Value>>,
}

#[derive(Properties, PartialEq, Deserialize, Serialize, Debug)]
pub struct State {
    pub results: Vec<Value>,
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {
    let state = use_prepared_state!((), move |_| -> State {

		let first = props
			.results
			.as_ref()
			.unwrap();
	
		// let second = match first.as_array() {
		// 	Some(v) => v.clone(),
		// 	None => {
		// 		println!("second");
		// 		Map::new().clone()
		// 	}
		// };

		let second = first.into_iter().map(|json| { json.as_object().unwrap() }).clone();

		// let second = match second.get("results") {
		// 	Some(v) => v.clone(),
		// 	None => {
		// 		println!("third");
		// 		println!("first: {:?}", first);
		// 		println!("second: {:?}", second);
		// 		json!({})
		// 	}
		// };

		// let fourth = match third.as_array() {
		// 	Some(v) => v.clone(),
		// 	None => {
		// 		println!("fourth");
		// 		vec![]
		// 	}
		// };

        State {
            results: second
            // results: props
			// 	.results
            //     .as_ref()
            //     .unwrap()
			// 	.as_object()
			// 	.unwrap()
			// 	.get("results")
			// 	.unwrap()
			// 	.as_array()
			// 	.unwrap()
			// 	.clone()
        }
    })?
    .unwrap();

    println!("{:?}", state);

    Ok(html! {
        <>
            <Header />
            <div class="container">
                <h1>{"Recently Rented"}</h1>
            </div>
        </>
    })
}

#[function_component(RecentlyRented)]
pub fn recently_rented_view(props: &Props) -> Html {
    html! {
        <Suspense fallback={ html! { <div>{"Loading..."}</div> } }>
            <Content results={props.results.clone()} />
        </Suspense>
    }
}

#[wasm_bindgen]
pub fn hydrate_recently_rented_view() -> Result<(), JsValue> {
    yew::Renderer::<RecentlyRented>::with_props(Props { results: None }).hydrate();
    Ok(())
}
