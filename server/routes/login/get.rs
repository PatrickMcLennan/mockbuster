use crate::utils::document::{Document, DocumentProps};
use actix_web::{get, Error as ActixError, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use login_view::login_view::{Login, Props};
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

#[get("/login")]
async fn get(messages: IncomingFlashMessages) -> Result<HttpResponse, ActixError> {
    let mut iterated_messages = messages.iter();
    let mut alert_message: Option<String> = None;
    let mut alert_styles: Option<String> = None;
    match iterated_messages.len() {
        1 => {
            let first = iterated_messages.nth(0).unwrap();
            alert_message = Some(first.content().to_string());
            alert_styles = Some(first.level().to_string());
            ()
        }
        _ => (),
    };

    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<Login>::with_props(|| Props {
                alert_message,
                alert_styles,
            })
            .render()
            .await
        })
    })
    .await
    .expect("the thread has failed.");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(Document::new(DocumentProps {
            wasm_assets: "loginView.js".to_string(),
            title: "Log in".to_string(),
            description: "Log in to mockbuster".to_string(),
            content,
        })))
}
