use crate::operations::{ratings, users};
use crate::utils::document::{Document, DocumentProps};
use actix_session::Session;
use actix_web::{
    get,
    web::{Data, Query},
    Error as ActixError, HttpResponse,
};
use profile_view::profile_view::{Profile, Props};
use sea_orm::{DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;
use tokio::task::LocalSet;

#[derive(Serialize, Deserialize)]
struct Params {
    pub id: Option<i32>,
}

#[get("/profile/{profile_id}")]
async fn get(
    db: Data<DatabaseConnection>,
    params: Query<Params>,
    session: Session,
) -> Result<HttpResponse, ActixError> {
    let unauthed = HttpResponse::Unauthorized().finish();

    let id = match params.id {
        Some(id) => id,
        None => match session.get("id") {
            Ok(v) => match v {
                Some(id) => id,
                None => return Ok(unauthed),
            },
            Err(_) => return Ok(unauthed),
        },
    };

    let profile = match users::fetch::execute(id, db.get_ref().clone()).await {
        Ok(s) => s,
        Err(_) => return Ok(HttpResponse::NotFound().finish()),
    };

    let recent_ratings =
        match ratings::fetch::by_user::execute(id as u32, db.get_ref().clone()).await {
            Ok(v) => Some(v),
            Err(e) => match e {
                DbErr::RecordNotFound(id) if id == id.to_string() => None,
                _ => return Ok(HttpResponse::InternalServerError().finish()),
            },
        };

    let profile_clone = profile.clone();
    let content = spawn_blocking(move || {
        use tokio::runtime::Builder;
        let set = LocalSet::new();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        set.block_on(&rt, async {
            yew::ServerRenderer::<Profile>::with_props(|| Props {
                recent_ratings,
                profile: Some(profile_clone),
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
            description: format!(
                "Profile for {} {}",
                profile.clone().first_name,
                profile.clone().last_name
            ),
            wasm_assets: "profiveView.js".to_string(),
            title: "Profile".to_string(),
            content,
        })))
}
