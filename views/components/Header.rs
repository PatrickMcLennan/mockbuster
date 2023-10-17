use crate::Logo::Logo;
use reqwasm::http::{Headers, Request};
use serde_json::json;
use web_sys::{console, window};
use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    let logout = Callback::from(move |_: MouseEvent| {
        wasm_bindgen_futures::spawn_local(async move {
            let headers = Headers::new();
            headers.set("Content-Type", "application/json");

            match Request::post("/logout")
                .headers(headers)
                .body(serde_json::to_string(&json!({})).unwrap())
                .send()
                .await
            {
                Ok(res) => {
                    if res.redirected() {
                        window().unwrap().location().set_href(&res.url()).unwrap();
                    }
                    ()
                }
                Err(e) => {
                    console::log_1(&format!("{:?}", e).into());
                    ()
                }
            }
        });
    });

    html! {
        <header class="sticky-top">
            <div class="container">
                <nav class="navbar navbar-expand-md">
                    <div class="col-2">
                        <a href="/" class="navbar-brand" style="max-height: 100px;">
                            <Logo />
                        </a>
                    </div>
                    <button
                        class="navbar-toggler"
                        type="button"
                        data-bs-toggle="collapse"
                        data-target="#navbarNavDropdown"
                        aria-controls="navbarNavDropdown"
                        aria-expanded="false"
                        aria-label="Toggle navigation"
                    >
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarNavDropdown">
                        <ul class="navbar-nav ml-auto">
                            <li class="nav-item">
                                <a class="nav-link" href="/recently-rented">
                                    {"Recently Rented"}
                                </a>
                            </li>
                            <li class="nav-item">
                                <a class="nav-link" href="/top-10">
                                    {"Top 10"}
                                </a>
                            </li>
                            <li class="nav-item dropdown">
                                <a
                                    class="nav-link dropdown-toggle"
                                    href="#"
                                    id="navbarDropdownMenuLink"
                                    data-bs-toggle="dropdown"
                                    aria-haspopup="true"
                                    aria-expanded="false"
                                >
                                    <svg
                                        aria-label="Profile dropdown"
                                        xmlns="http://www.w3.org/2000/svg"
                                        width="16"
                                        height="16"
                                        fill="currentColor"
                                        class="bi bi-person-circle"
                                        viewBox="0 0 16 16"
                                    >
                                        <path d="M11 6a3 3 0 1 1-6 0 3 3 0 0 1 6 0z"/>
                                        <path fill-rule="evenodd" d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8zm8-7a7 7 0 0 0-5.468 11.37C3.242 11.226 4.805 10 8 10s4.757 1.225 5.468 2.37A7 7 0 0 0 8 1z"/>
                                    </svg>
                                </a>
                                <ul class="dropdown-menu" aria-labelledby="navbarDropdownMenuLink">
                                    <li class="dropdown-item">
                                        <a class="nav-link" href="/profile">{"Profile"}</a>
                                    </li>
                                    <li class="dropdown-item">
                                        <button class="nav-link" onclick={logout}>{"Logout"}</button>
                                    </li>
                                </ul>
                            </li>
                        </ul>
                    </div>
                </nav>
            </div>
        </header>
    }
}
