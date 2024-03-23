use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub comment: String,
    pub oninput: Callback<InputEvent>,
}

#[function_component(CommentEntry)]
pub fn comment_entry(props: &Props) -> Html {
    html! {
        <fieldset class="mt-3">
            <textarea
                class="form-control"
                name="comment"
                rows="4"
                cols="50"
                placeholder="Leave an optional comment.  You can comment on a movie at any time."
                oninput={props.oninput.clone()}
                value={props.comment.to_string()}
            >
            </textarea>
            <small class="d-block mt-3 text-right">{format!("{}/250", props.comment.to_string().len())}</small>
        </fieldset>
    }
}
