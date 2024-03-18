use crate::logo::Logo;
use reqwasm::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{console, window, HtmlInputElement};
use yew::prelude::*;

const TIMEOUT_MS: i32 = 450;

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
                    TIMEOUT_MS,
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
                    <div class="collapse navbar-collapse ml-auto" id="navbarNavDropdown">
                        <ul class="navbar-nav">

                        </ul>
                    </div>
                    <form class="input-group mb-0 d-flex justify-content-center" novalidate={true} onsubmit={submit_stub} role="search">
                        <input
                            aria-label="Search"
                            autofocus={ if props.search.to_string().len() >= 1 { true }  else { false } }
                            type="search"
                            class="form-control ml-auto"
                            placeholder="Search..."
                            value={props.search.to_string()}
                            oninput={oninput}
                            style="max-width: 250px;"
                        />
                        <button class="btn btn-outline-primary mr-auto" type="submit">{"🔎"}</button>
                    </form>
                    <div class="nav-item dropdown">
                        <button
                            class="btn dropdown-toggle"
                            id="navbarDropdownMenuButton"
                            data-bs-toggle="dropdown"
                            data-toggle="dropdown"
                            aria-haspopup="true"
                            aria-expanded="false"
                            type="button"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-list" viewBox="0 0 16 16">
                                <path fill-rule="evenodd" d="M2.5 12a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5m0-4a.5.5 0 0 1 .5-.5h10a.5.5 0 0 1 0 1H3a.5.5 0 0 1-.5-.5"/>
                            </svg>
                        </button>
                        <ul class="dropdown-menu dropdown-menu-right" aria-labelledby="navbarDropdownMenuButton">
                            <li class="dropdown-item">
                                    <a class="nav-link text-nowrap" href="/recently-rented">
                                        {"Recently Rented"}
                                    </a>
                                </li>
                            <li class="dropdown-item">
                                <a class="nav-link text-nowrap" href="/top-10">
                                    {"Top 10"}
                                </a>
                            </li>
                            <li class="dropdown-item">
                                <a class="nav-link" href="/profile">{"Profile"}</a>
                            </li>
                            <li class="dropdown-item">
                                <button class="nav-link" onclick={logout}>{"Logout"}</button>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
        </header>
    }
}
