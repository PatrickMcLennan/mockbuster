use crate::utils::document::{Document, DocumentProps};
use actix_web::{get, Error as ActixError, HttpResponse};
use home_view::home_view::Home;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

#[get("/")]
async fn get() -> Result<HttpResponse, ActixError> {
    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<Home>::new().render().await
        })
    })
    .await
    .expect("the thread has failed.");

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(Document::new(DocumentProps {
            wasm_assets: "homeView.js".to_string(),
            title: "Home".to_string(),
            content,
        })))
}
