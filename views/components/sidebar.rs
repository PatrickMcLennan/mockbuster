use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum CurrentRoute {
    TopRated,
    RecentlyRented,
    Home,
    Default,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub current_route: Option<CurrentRoute>,
    // pub user_initials: String,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &Props) -> Html {
    let current_route = match props.current_route.clone() {
        Some(v) => v,
        None => CurrentRoute::Default,
    };

    html! {
        <nav class="border border-right border-top-none sticky-top" style="flex: 0.5; min-width: 170px; height: calc(100vh - 78px); top: 78px;">
            <ul class="nav nav-pills flex-column mb-auto" style="height: 100%;">
                <li class="nav-item my-3">
                    <a class="rounded-circle border d-flex justify-content-center align-items-center mx-auto" style="width:100px;height:100px" href="/profile">
                        // {&props.user_initials}
                        {"PM"}
                    </a>
                </li>
                <li class="nav-item">
                    <a href="/" class={classes!("nav-link", if current_route == CurrentRoute::Home { "active" } else { "" })} style="border-radius: 0;">
                        <span class="nav-name" style="word-wrap: normal;">{"Home"}</span>
                    </a>
                </li>
                <li class="nav-item">
                    <a href="/recently-rented" class={classes!("nav-link", if current_route == CurrentRoute::RecentlyRented { "active" } else { "" })} style="border-radius: 0;">
                        <span class="nav-name" style="word-wrap: normal;">{"Recently Rented"}</span>
                    </a>
                </li>
                <li class="nav-item">
                    <a href="/top-10" class={classes!("nav-link", if current_route == CurrentRoute::TopRated { "active" } else { "" })} style="border-radius: 0;">
                        <span class="nav-name" style="word-wrap: normal;">{"Top Rated"}</span>
                    </a>
                </li>
                <li class="nav-item mt-auto">
                    <form class="p-0" method="POST" action="/logout">
                        <button class="btn btn-secondary" type="submit" style="width: 100%; border-radius: 0; text-align: left;">{"Logout"}</button>
                    </form>
                </li>
            </ul>
        </nav>
    }
}
