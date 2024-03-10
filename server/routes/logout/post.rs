use crate::operations::users;
use actix_session::Session;
use actix_web::{http::StatusCode, post, Error as ActixError, HttpResponse};

#[post("/logout")]
async fn post(session: Session) -> Result<HttpResponse, ActixError> {
    users::logout::execute(session);

    Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
        .append_header(("Location", "/login"))
        .finish())
}
