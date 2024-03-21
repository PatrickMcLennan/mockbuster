use crate::sidebar::{CurrentRoute, Sidebar};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub current_route: Option<CurrentRoute>,
}

#[function_component(Frame)]
pub fn frame(props: &Props) -> Html {
    html! {
        <div class="d-flex justify-content-start align-items-stretch">
            <Sidebar current_route={props.current_route.clone()} />
            <main style="width: 100%; flex: 3; height: 100%;">
                {props.children.clone()}
            </main>
        </div>
    }
}
