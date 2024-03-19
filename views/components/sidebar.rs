use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <nav class="border border-right border-top-none fixed" style="flex: 0.5; min-width: 170px;">
            <ul class="nav nav-pills flex-column mb-auto">
                <li class="nav-item">
                    <a href="/" class="nav-link active">
                        <span class="nav-name" style="word-wrap: normal;">{"Home"}</span>
                    </a>
                </li>
                <li class="nav-item">
                    <a href="/recently-rented" class="nav-link">
                        <span class="nav-name" style="word-wrap: normal;">{"Recently Rented"}</span>
                    </a>
                </li>
                <li class="nav-item">
                    <a href="top-10" class="nav-link">
                        <span class="nav-name" style="word-wrap: normal;">{"Top Rated"}</span>
                    </a>
                </li>
            </ul>
        </nav>
    }
}
