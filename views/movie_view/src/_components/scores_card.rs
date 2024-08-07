use yew::prelude::*;

use components::rating_bar::{Props as RatingBarProps, RatingBar};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub scores: Vec<RatingBarProps>,
}

#[function_component(ScoresCard)]
pub fn scores_card(props: &Props) -> Html {
    html! {
        <aside class="card px-0 col-sm-12 col-lg-4" style="height: max-content;">
            <div class="card-header">
                <strong>{"Scores"}</strong>
            </div>
            <div class="card-body">
                {
                    props
                        .scores
                        .clone()
                        .into_iter()
                        .map(|score| {
                            html! {
                                <div class="mb-2">
                                    <RatingBar score={score.score} date={score.date} />
                                </div>
                            }
                        })
                        .collect::<Html>()
                }
            </div>
        </aside>
    }
}
