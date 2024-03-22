use crate::utils::document::{Document, DocumentProps};
use actix_web::{get, Error as ActixError, HttpResponse};
use login_view::login_view::Login;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

#[get("/login")]
async fn get() -> Result<HttpResponse, ActixError> {
    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();
        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<Login>::new().render().await
        })
    })
    .await
    .expect("the thread has failed.");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(Document::new(DocumentProps {
            wasm_assets: "loginView.js".to_string(),
            title: "Log in".to_string(),
            content,
        })))
}
