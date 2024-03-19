use crate::sidebar::Sidebar;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(Frame)]
pub fn frame(props: &Props) -> Html {
    html! {
        <div class="d-flex justify-content-start align-items-stretch">
            <Sidebar />
            <main style="width: 100%; flex: 3;">
                {props.children.clone()}
            </main>
        </div>
    }
}
