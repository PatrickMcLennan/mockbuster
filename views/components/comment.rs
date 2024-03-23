use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub comment: String,
    pub user_id: i32,
    pub user_name: String,
    pub created_at: String,
}

#[function_component(Comment)]
pub fn comment(props: &Props) -> Html {
    html! {
        <blockquote class="blockquote mb-0">
            <p>{&props.comment}</p>
            <footer class="blockquote-footer">
                <a class="link-primary" href={format!("/profile/{}", props.user_id)}>
                    <cite title={props.user_name.to_string()}>
                        {&props.user_name}
                    </cite>
                </a>
                <small>
                    {" on "}
                    <date time={props.created_at.to_string()}>
                        {&props.created_at}
                    </date>
                </small>
            </footer>
        </blockquote>
    }
}
