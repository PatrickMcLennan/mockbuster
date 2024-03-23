use actix_session::Session;
use actix_web::{
    http::StatusCode, post, web::Data, web::Json, Error as ActixError, HttpResponse, Responder,
};
use sea_orm::DatabaseConnection;
use validators::users::login_form::LoginFormSchema;

use crate::operations::users;

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

    match users::login::execute(session, db.get_ref().clone(), form).await {
        Some(v) => match v {
            Ok(_) => Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
                .append_header(("Location", "/"))
                .finish()),
            Err(_) => Ok(HttpResponse::InternalServerError().finish()),
        },
        None => Ok(HttpResponse::InternalServerError().finish()),
    }
}
