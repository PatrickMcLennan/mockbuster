use crate::logo::Logo;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{window, HtmlInputElement};
use yew::prelude::*;

const TIMEOUT_MS: i32 = 450;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[prop_or(String::new())]
    pub search: String,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    let timeout = use_mut_ref(|| 0);
    let timeout_clone = timeout.clone();
    let props_clone = props.clone();

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
            <nav class="navbar">
                <div class="container-fluid">
                    <a class="navbar-brand" href="/" aria-label="Home">
                        <Logo />
                    </a>
                    <form class="d-flex mb-0" novalidate={true} onsubmit={submit_stub} role="search">
                        <div class="input-group">
                            <input
                                aria-label="Search"
                                autofocus={ if props.search.to_string().len() >= 1 { true }  else { false } }
                                type="search"
                                class="form-control"
                                placeholder="Search..."
                                value={props.search.to_string()}
                                oninput={oninput}
                                style="width: 275px;"
                            />
                            <button class="btn btn-outline-primary" type="submit">{"🔎"}</button>
                        </div>
                    </form>
                </div>
            </nav>
        </header>
    }
}
