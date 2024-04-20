use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    pub header: String,
    pub copy: String,
}

#[function_component(StatsCard)]
pub fn stats_card(props: &Props) -> Html {
    html! {
        <div class="col-sm-6 col-md-4">
            <section class="card">
                <header class="card-header">
                    <strong>{&props.header}</strong>
                </header>
                <div class="card-body">
                    <p class="card-text">{&props.copy}</p>
                </div>
            </section>
        </div>
    }
}
