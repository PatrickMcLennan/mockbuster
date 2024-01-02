use crate::Logo::Logo;
use reqwasm::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{console, window, HtmlInputElement};
use yew::prelude::*;

#[derive(Debug, Properties, PartialEq, Deserialize, Serialize, Clone)]
pub struct Props {
    #[prop_or(String::new())]
    pub search: String,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    let timeout = use_mut_ref(|| 0);
    let timeout_clone = timeout.clone();
    let props_clone = props.clone();

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

    let oninput = Callback::from(move |event: InputEvent| {
        let target: HtmlInputElement = event.target_unchecked_into();
        let value: String = target.value().into();

        window()
            .unwrap()
            .clear_timeout_with_handle(*timeout.borrow_mut());

        if &value != &props_clone.search {
            *timeout.borrow_mut() = window()
                .unwrap()
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    Closure::once_into_js(Box::new(move || {
                        window()
                            .unwrap()
                            .location()
                            .set_href(&format!("/search?page=1&query={}", value))
                            .unwrap()
                    }) as Box<dyn FnMut()>)
                    .as_ref()
                    .unchecked_ref(),
                    250,
                )
                .expect("Failed to set timeout");
        } else {
            *timeout.borrow_mut() = 0;
        }
        ()
    });

    let submit_stub = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        ()
    });

    use_effect(move || {
        move || {
            window()
                .unwrap()
                .clear_timeout_with_handle(*timeout_clone.borrow_mut())
        }
    });

    html! {
        <header class="sticky-top bg-white">
            <nav class="navbar navbar-expand-md">
                <div class="container-fluid">
                    <div class="navbar-brand" style="max-height: 80px;">
                        <a href="/">
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
                    <div class="collapse navbar-collapse ml-auto" id="navbarNavDropdown">
                        <ul class="navbar-nav">
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
                            <li>
                                <form class="input-group mb-0" novalidate={true} onsubmit={submit_stub}>
                                    <span class="input-group-text">{"🔎"}</span>
                                    <input
                                        autofocus={ if props.search.to_string().len() >= 1 { true }  else { false } }
                                        type="search"
                                        class="form-control"
                                        placeholder="Search for movies"
                                        value={props.search.to_string()}
                                        oninput={oninput}
                                    />
                                </form>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
        </header>
    }
}
