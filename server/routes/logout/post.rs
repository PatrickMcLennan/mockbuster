use crate::operations::logout::logout as logout_operation;
use actix_session::Session;
use actix_web::{http::StatusCode, post, Error as ActixError, HttpResponse};

#[post("/logout")]
async fn post(session: Session) -> Result<HttpResponse, ActixError> {
    logout_operation(session);

    Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
        .append_header(("Location", "/login"))
        .finish())
}
