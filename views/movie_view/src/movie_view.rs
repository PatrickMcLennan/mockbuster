use models::tmdb_movies::movie_id_result::MovieIdResult;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use components::{vote_modal::VoteModal, Header::Header};

#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub movie: Option<MovieIdResult>,
}

#[derive(Properties, PartialEq, Deserialize, Serialize)]
pub struct State {
    pub movie: MovieIdResult,
}

#[function_component]
fn Content(props: &Props) -> HtmlResult {
    let state = use_prepared_state!((), move |_| -> State {
        State {
            movie: props.movie.as_ref().unwrap().clone(),
        }
    })?
    .unwrap();
    let vote_modal_open = use_state(|| false);
    let vote_modal_open_clone = vote_modal_open.clone();

    let backdrop = format!(
        "https://image.tmdb.org/t/p/w500{}",
        match &state.movie.backdrop_path {
            Some(v) => v.to_string(),
            None => String::new(),
        }
    );

    let toggle_modal = {
        let clone = vote_modal_open.clone();
        Callback::from(move |_: MouseEvent| vote_modal_open.set(!*clone))
    };

    let poster = format!("https://image.tmdb.org/t/p/w500{}", state.movie.poster_path);
    let title = &state.movie.title;
    let tagline = &state.movie.tagline;

    Ok(html! {
        <>
            <Header />
            <main>
                <header class="card" style={format!("background-image: url({}); background-size: cover; background-repeat: no-repeat;", backdrop)}>
                    <div class="container">
                        <div class="row g-0">
                            <div class="col-md-4 d-flex align-items-center">
                                <img src={poster} class="img-fluid rounded-start h-75 d-block mx-auto" alt={format!("Poster for {}", title)} />
                            </div>
                            <div class="col-md-8 my-auto">
                                <div class="card-body text-bg-dark rounded">
                                    <h1 class="card-title text-end">{title}</h1>
                                    <h2 class="card-subtitle my-2 text-end"><i>{tagline}</i></h2>
                                    <p class="card-text" style="text-indent: 2rem;">{&state.movie.overview}</p>
                                    <button
                                        onclick={toggle_modal}
                                        class="btn btn-primary"
                                        data-bs-toggle="modal"
                                        data-bs-target={format!("#vote-modal-{}", state.movie.id)}
                                        type="button"
                                    >
                                        {"Rate"}
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </header>
                <section class="container">

                </section>
                <VoteModal
                    title={state.movie.title.to_string()}
                    id={state.movie.id}
                    open={*vote_modal_open_clone}
                />
            </main>
        </>
    })
}

#[function_component(Movie)]
pub fn movie_view(props: &Props) -> Html {
    html! {
        <Suspense fallback={ html! { <div>{"Loading..."}</div> } }>
            <Content movie={props.movie.clone()} />
        </Suspense>
    }
}

#[wasm_bindgen]
pub fn hydrate_movie_view() -> Result<(), JsValue> {
    yew::Renderer::<Movie>::with_props(Props { movie: None }).hydrate();
    Ok(())
}
