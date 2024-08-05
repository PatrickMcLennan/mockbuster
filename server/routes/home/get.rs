use crate::utils::document::{Document, DocumentProps};
use actix_web::{get, web::Data, Error as ActixError, HttpResponse};
use home_view::home_view::{Home, Props};
use operations::events;
use sea_orm::DatabaseConnection;
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

#[get("/")]
async fn get(
    db: Data<DatabaseConnection>,
    http_client: Data<reqwest_middleware::ClientWithMiddleware>,
) -> Result<HttpResponse, ActixError> {
    let events =
        match events::list::execute(db.get_ref().clone(), http_client.get_ref().clone()).await {
            Ok(v) => v,
            Err(_) => return Ok(HttpResponse::InternalServerError().finish()),
        };

    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<Home>::with_props(|| Props {
                events: Some(events),
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
            wasm_assets: "homeView.js".to_string(),
            title: "Home".to_string(),
            description: "Share movies with your friends on mockbuster".to_string(),
            content,
        })))
}
