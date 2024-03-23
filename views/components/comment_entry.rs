use wasm_bindgen::JsCast;
use yew::prelude::*;

use web_sys::{EventTarget, HtmlTextAreaElement};
#[derive(Properties, PartialEq)]
pub struct Props {
    pub comment: UseStateHandle<String>,
}

#[function_component(CommentEntry)]
pub fn comment_entry(props: &Props) -> Html {
    let oninput = {
        let comment_clone = props.comment.clone();
        Callback::from(move |e: InputEvent| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());

            if let Some(input) = input {
                let mut new_comment = input.value();
                new_comment.truncate(250);

                comment_clone.set(new_comment);
            }
        })
    };

    html! {
        <fieldset>
            <textarea
                aria-describedby="comment-limit"
                class="form-control"
                maxlength="250"
                minlength="0"
                name="comment"
                rows="4"
                cols="50"
                placeholder="Optional"
                required=true
                oninput={oninput}
                value={props.comment.to_string()}
            >
            </textarea>
            <small id="comment-limit" class="d-block mt-3 text-right">{format!("{}/250", props.comment.to_string().len())}</small>
        </fieldset>
    }
}
