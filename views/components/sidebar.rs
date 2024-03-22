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
}

#[function_component(Sidebar)]
pub fn sidebar(props: &Props) -> Html {
    let current_route = match props.current_route.clone() {
        Some(v) => v,
        None => CurrentRoute::Default,
    };

    html! {
        <nav class="border border-right border-top-none fixed" style="flex: 0.5; min-width: 170px;">
            <ul class="nav nav-pills flex-column mb-auto">
                <li class="nav-item">
                    <a href="/" class={classes!("nav-link", if current_route == CurrentRoute::Home { "active" } else { "" })}>
                        <span class="nav-name" style="word-wrap: normal;">{"Home"}</span>
                    </a>
                </li>
                <li class="nav-item">
                    <a href="/recently-rented" class={classes!("nav-link", if current_route == CurrentRoute::RecentlyRented { "active" } else { "" })}>
                        <span class="nav-name" style="word-wrap: normal;">{"Recently Rented"}</span>
                    </a>
                </li>
                <li class="nav-item">
                    <a href="top-10" class={classes!("nav-link", if current_route == CurrentRoute::TopRated { "active" } else { "" })}>
                        <span class="nav-name" style="word-wrap: normal;">{"Top Rated"}</span>
                    </a>
                </li>
            </ul>
        </nav>
    }
}
