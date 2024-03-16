use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct VoteModalProps {
    pub title: String,
    pub id: i32,
    pub open: bool,
}

#[function_component(VoteModal)]
pub fn vote_modal(props: &VoteModalProps) -> Html {
    let score = use_state(|| 5.0);

    let score_color = match *score as f32 {
        0.0..=2.5 => Some("bg-danger"),
        2.6..=5.0 => Some("bg-info"),
        5.1..=7.5 => Some("bg-success"),
        _ => None,
    };

    let oninput = {
        let score_clone = score.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                let value = input.value();
                let mut new_score = if value.len() == 0 {
                    0.0
                } else {
                    value.parse::<f64>().unwrap()
                };

                if new_score > 10.0 {
                    new_score = 10.0
                } else if new_score < 0.0 {
                    new_score = 0.0
                } else if new_score % 0.5 != 0.0 {
                    new_score = (new_score * 2.0).round() / 2.0
                }

                score_clone.set(new_score);
            }
        })
    };

    return html! {
        <div aria-hidden={(!props.open).to_string()} class="modal fade" id={format!("vote-modal-{}", props.id)} tabindex="-1">
            <form method="POST" class="modal-dialog modal-dialog-centered">
                <div class="modal-content">
                    <header class="modal-header">
                        <h5 class="modal-title">{format!("Rate {}", props.title)}</h5>
                        <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                    </header>
                    <fieldset class="modal-body">
                        <legend style="font-size: 1rem;">
                            {"Select a score from 0 - 10 in 0.5 increments.  "}<strong>{"This cannot be undone, and you cannot change your vote later."}</strong>
                        </legend>
                        <div class="input-group mb-3">
                            <input
                                type="number"
                                name="score"
                                class="form-control"
                                aria-label="Score"
                                min="0"
                                max="10"
                                step="0.5"
                                oninput={oninput}
                                value={format!("{}", *score)}
                            />
                            <div class="input-group-append">
                                <span class="input-group-text">{" / 10"}</span>
                            </div>
                        </div>
                        <div class="progress-stacked mt-1" role="progressbar" aria-label={format!("{} / 10", *score)} aria-valuenow={format!("{}", *score)} aria-valuemin="0" aria-valuemax="10">
                            <div class={classes!("progress-bar", if score_color.is_some() { score_color } else { None })} style={format!("width: {}%", *score * 10.0)}></div>
                        </div>
                    </fieldset>
                    <fieldset class="modal-footer">
                        <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                        <button type="submit" class="btn btn-primary">{"Submit Rating"}</button>
                    </fieldset>
                </div>
            </form>
        </div>
    };
}
