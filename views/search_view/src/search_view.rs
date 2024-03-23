use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::{
    frame::Frame, header::Header, movie_card::MovieCard, page_title::PageTitle,
    pagination::Pagination,
};
use models::tmdb_movies::movie_search_result::MovieSearchResults;
use serde::{Deserialize, Serialize};
use validators::tmdb_movies::search_dto::SearchDTO;

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
    })?
    .unwrap();

    let show_pagination = &state.movie_search_results.total_pages > &1;
    let current_page = state.dto.page;
    let total_pages = state.movie_search_results.total_pages as i64;

    Ok(html! {
        <>
            <Header search={state.dto.query.to_string()} />
            <Frame current_route={None}>
                <PageTitle
                    h1={"Search".to_string()}
                    h2={format!("Results for: {}", &state.dto.query)}
                />
                <section class="row g-3 container">
                    {
                        state
                            .movie_search_results
                            .clone()
                            .results
                            .into_iter()
                            .map(|result|
                                html! {
                                    <div class="col-sm-12 col-md-6 col-lg-4">
                                        <MovieCard key={result.tmdb.title.to_string()} movie={result.tmdb.clone()} />
                                    </div>
                                }
                            )
                            .collect::<Html>()
                    }
                </section>
                {if show_pagination {
                    html! {
                        <Pagination
                            current_page={current_page}
                            previous_url={format!("/search?query={}&page={}", state.dto.query, current_page - 1)}
                            next_url={format!("/search?query={}&page={}", state.dto.query, current_page + 1)}
                            base_url={format!("/search?query={}&", state.dto.query)}
                            total_pages={total_pages}
                        />
                    }
                } else { html! { <></> } }}
            </Frame>
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
pub fn hydrate_search_view() -> Result<(), JsValue> {
    yew::Renderer::<Search>::with_props(Props {
        dto: None,
        movie_search_results: None,
    })
    .hydrate();
    Ok(())
}
