use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub score: f32,
    #[prop_or_default]
    pub date: Option<String>,
}

const TEXT_BACKGROUND: &str = "background-color: white; max-width: max-content; padding: 0.5rem;";

#[function_component(RatingBar)]
pub fn rating_bar(props: &Props) -> Html {
    let score_f32 = props.score as f32;
    let score_color = if score_f32 >= 0.0 && score_f32 <= 2.5 {
        Some("bg-danger")
    } else if score_f32 >= 2.6 && score_f32 <= 5.0 {
        Some("bg-info")
    } else {
        Some("bg-success")
    };

    let score_100 = props.score as i32 * 10;

    html! {
        <div>
            <div class="d-flex flex-md-row flex-sm-column justify-content-between align-items-end">
                <span class="display-6" style={TEXT_BACKGROUND}>
                    {props.score}
                    <small class="h5" style={TEXT_BACKGROUND}>{" / 10"}</small>
                </span>
                {
                    match &props.date {
                        Some(v) => html! {
                            <small class="text-muted">
                                <time datetime={v.to_string()}>
                                    {v}
                                </time>
                            </small>
                        },
                        None => html! { <></> }
                    }
            }
            </div>
            <div class="progress-stacked mt-1" role="progressbar" aria-label={format!("{} / 10", props.score)} aria-valuenow={props.score.to_string()} aria-valuemin="0" aria-valuemax="10">
                <div class={classes!("progress-bar", if score_color.is_some() { score_color } else { None })} style={format!("width: {}%", score_100)}></div>
            </div>
        </div>
    }
}
