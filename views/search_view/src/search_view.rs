use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::{Header::Header, MovieCard::MovieCard};
use models::tmdb::movie_search_result::MovieSearchResults;
use serde::{Deserialize, Serialize};
use validators::search_dto::SearchDTO;

#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub movie_search_results: Option<MovieSearchResults>,
    pub dto: Option<SearchDTO>,
}

#[derive(Properties, PartialEq, Deserialize, Serialize)]
pub struct State {
    pub movie_search_results: MovieSearchResults,
    pub dto: SearchDTO,
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {
	let state = use_prepared_state!((), move |_| -> State {
		State {
			movie_search_results: props.movie_search_results.as_ref().unwrap().clone(),
			dto: props.dto.as_ref().unwrap().clone(),
		}
	})?.unwrap();

	Ok(html! {
		<>
			<Header />
			<div class="container">
				<h1>{"Search"}</h1>
				<h2>{format!("Results for: {}", state.dto.query)}</h2>
				<div class="row">
					{
						state
							.movie_search_results
							.clone()
							.results
							.into_iter()
							.map(|result| 
								html! {
									<div class="col-12 col-sm-6 col-lg-3 col-4">
										<MovieCard key={result.title.to_string()} movie={result.clone()} />
									</div>
								}
							)
							.collect::<Html>()
					}
				</div>
			</div>
		</>
	})
}

#[function_component(Search)]
pub fn search_view(props: &Props) -> Html {
    html! {
        <Suspense fallback={ html! { <div>{"Loading..."}</div> } }>
			<Content movie_search_results={props.movie_search_results.clone()} dto={props.dto.clone()} />
        </Suspense>
    }
}

#[wasm_bindgen]
pub fn run_search_view_wasm() -> Result<(), JsValue> {
    yew::Renderer::<Search>::with_props(Props {
        dto: None,
        movie_search_results: None,
    })
    .hydrate();
    Ok(())
}
