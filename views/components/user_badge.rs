use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub user_name: String,
    pub user_id: i32,
    pub image_url: String,
}

#[function_component(UserBadge)]
pub fn user_badge(props: &Props) -> Html {
    html! {
        <a href={format!("/profile/{}", props.user_id)}>
            <img
                alt={props.user_name.to_string()}
                src={props.image_url.to_string()}
                style="display: inline-block; margin-right: 0.5rem; width: 2rem; height: auto; border-radius: 100%;"
            />
            {props.user_name.to_string()}
        </a>
    }
}
