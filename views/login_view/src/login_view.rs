use reqwasm::http::{Headers, Request};
use serde::{Deserialize, Serialize};
use validators::users::login_form::LoginFormSchema;
use wasm_bindgen::prelude::*;
use web_sys::{console, window, HtmlInputElement};
use yew::prelude::*;

static EMAIL_ERROR_ID: &str = "email-error-message";
static PASSWORD_ERROR_ID: &str = "password-error-message";

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub alert_message: Option<String>,
    pub alert_styles: Option<String>,
}

#[derive(Properties, PartialEq, Clone, Deserialize, Serialize)]
pub struct State {
    pub alert_message: Option<String>,
    pub alert_styles: Option<String>,
}

#[function_component]
pub fn Content(props: &Props) -> HtmlResult {
    let state = use_prepared_state!((), |_| -> State {
        let props_clone = props.clone();
        State {
            alert_message: props_clone.alert_message,
            alert_styles: props_clone.alert_styles,
        }
    })?
    .unwrap();
    let loading = use_state(|| false);
    let email_error = use_state(|| String::new());
    let password_error = use_state(|| String::new());
    let email_ref = use_node_ref();
    let password_ref = use_node_ref();

    let email_ref_clone = email_ref.clone();
    let email_error_clone = email_error.clone();
    let loading_clone = loading.clone();
    let password_ref_clone = password_ref.clone();
    let password_error_clone = password_error.clone();

    let has_email_errors = &email_error.chars().count() >= &1;
    let has_password_errors = &password_error.chars().count() >= &1;

    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();

        let (email_value, password_value) = (
            email_ref_clone
                .clone()
                .cast::<HtmlInputElement>()
                .expect("No email ref")
                .value(),
            password_ref_clone
                .clone()
                .cast::<HtmlInputElement>()
                .expect("No password ref")
                .value(),
        );

        let form_data = LoginFormSchema {
            email: email_value.to_string(),
            password: password_value.to_string(),
        };

        match &form_data.get_errors() {
            Some(errors) => {
                email_error_clone.set(errors.email.to_string());
                password_error_clone.set(errors.password.to_string());
                return ();
            }
            None => (),
        };

        let new_loading_clone = loading_clone.clone();
        new_loading_clone.set(true);
        let body = form_data.to_json();

        let headers = Headers::new();
        headers.set("Content-Type", "application/json");

        wasm_bindgen_futures::spawn_local(async move {
            match Request::post("/login")
                .headers(headers)
                .body(body)
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
                    new_loading_clone.set(false);
                    ()
                }
            }
        });

        ()
    });

    Ok(html! {
        <div class="container row">
            <div class="col-6 offset-3">
                <form novalidate={true} onsubmit={onsubmit}>
                    <fieldset>
                        <div class="form-group">
                            <label for="email">{"Email"}</label>
                            <input
                                aria-errormessage={
                                    if has_email_errors { Some(EMAIL_ERROR_ID) } else { None }
                                }
                                class={classes!("form-control", if has_email_errors { "is-invalid" } else { "" })}
                                value="king@theking.com"
                                type="email"
                                id="email"
                                ref={email_ref}
                            />
                            {if has_email_errors {
                                html! {
                                    <small id={EMAIL_ERROR_ID} class="form-text invalid-feedback">{email_error.to_string()}</small>
                                }
                            } else {
                                html! {
                                    <></>
                                }
                            }}
                        </div>
                        <div class="form-group">
                            <label for="password">{"Password"}</label>
                            <input
                                aria-errormessage={
                                    if has_password_errors { Some(PASSWORD_ERROR_ID) } else { None }
                                }
                                class={classes!("form-control", if has_password_errors { "is-invalid" } else { "" })}
                                type="password"
                                value="!Testing2"
                                id="password"
                                ref={password_ref}
                            />
                            {if has_password_errors {
                                html! {
                                    <small id={PASSWORD_ERROR_ID} class="form-text invalid-feedback">{password_error.to_string()}</small>
                                }
                            } else {
                                html! {
                                    <></>
                                }
                            }}
                        </div>
                    </fieldset>
                    <fieldset>
                        <button class="btn btn-primary" disabled={*loading} type="submit">
                            {if *loading {
                                html! {
                                    <div class="spinner-border text-warning" role="status">
                                        <span class="visually-hidden">{"Loading..."}</span>
                                    </div>
                                }
                            } else {
                                html! { {"Log In"} }
                            }}
                        </button>
                    </fieldset>
                </form>
            </div>
        </div>
    })
}

#[function_component(Login)]
pub fn login(props: &Props) -> Html {
    let props_clone = props.clone();
    html! {
        <Suspense fallback={ html! { <div>{"Loading..."}</div> }}>
            <Content alert_styles={props_clone.alert_styles} alert_message={props_clone.alert_message} />
        </Suspense>
    }
}

#[wasm_bindgen]
pub fn hydrate_login_view() -> Result<(), JsValue> {
    yew::Renderer::<Login>::with_props(Props {
        alert_message: None,
        alert_styles: None,
    })
    .hydrate();
    Ok(())
}
