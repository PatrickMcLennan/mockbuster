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
        <header class="sticky-top bg-white border border-top-none border-left-none border-right-none">
            <nav class="px-4 container">
                <div class="row gx-2">
                    <div class="col-4" style="max-height: 80px;">
                        <a href="/" aria-label="Home">
                            <Logo />
                        </a>
                    </div>
                    <div class="col-8 d-flex align-items-center">
                        <form class="input-group mb-0" novalidate={true} onsubmit={submit_stub} role="search">
                            <input
                                aria-label="Search"
                                autofocus={ if props.search.to_string().len() >= 1 { true }  else { false } }
                                type="search"
                                class="form-control"
                                placeholder="Search..."
                                value={props.search.to_string()}
                                oninput={oninput}
                                style="max-width: 300px;"
                            />
                            <button class="btn btn-outline-primary mr-auto" type="submit">{"🔎"}</button>
                        </form>
                    </div>
                </div>
            </nav>
        </header>
    }
}
