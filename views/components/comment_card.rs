use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub comment: f32,
    // pub created_at: chrono::
    pub user_name: String,
}

#[function_component(CommentCard)]
pub fn comment_card(props: &Props) -> Html {
    html! {
        <div>
            <blockquote class="blockquote">
                <p class="mb-0">{props.comment}</p>
                <footer class="blockquote-footer"><cite title={props.user_name.to_string()}>{&props.user_name}</cite></footer>
            </blockquote>
        </div>
    }
}
