use actix_session::Session;
use actix_web::{
    http::StatusCode, post, web::Data, web::Json, Error as ActixError, HttpResponse, Responder,
};
use sea_orm::DatabaseConnection;
use serde_json::json;
use validators::login_form::LoginFormSchema;

use crate::operations::login::login as login_operation;

#[post("/login")]
async fn post(
    Json(form): Json<LoginFormSchema>,
    db: Data<DatabaseConnection>,
    session: Session,
) -> Result<impl Responder, ActixError> {
    match form.get_errors() {
        Some(e) => return Ok(HttpResponse::BadRequest().body(e.to_json())),
        None => (),
    };

    match login_operation(session, db.get_ref().clone(), form).await {
        Ok(_) => Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
            .append_header(("Location", "/"))
            .finish()),
        Err(e) => {
            Ok(HttpResponse::BadRequest()
                .body(serde_json::to_string(&json!({"message": e})).unwrap()))
        }
    }
}
