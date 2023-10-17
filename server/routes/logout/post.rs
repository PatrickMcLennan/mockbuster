use actix_web::{post, Error as ActixError, HttpResponse, http::StatusCode};
use actix_session::Session;
use crate::operations::logout::logout as logout_operation;

#[post("/logout")]
async fn post(session: Session) -> Result<HttpResponse, ActixError> {
    logout_operation(session);

    Ok(
		HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
			.append_header(("Location", "/login"))
			.finish()
	)
}
