use actix_web::{http::StatusCode, post, web::Json, Error as ActixError, HttpResponse, Responder};

use validators::login_form::LoginFormSchema;

#[post("/login")]
async fn post(Json(form): Json<LoginFormSchema>) -> Result<impl Responder, ActixError> {
    match form.get_errors() {
        Some(e) => Ok(HttpResponse::BadRequest().body(e.to_json())),
        None => Ok(HttpResponse::build(StatusCode::MOVED_PERMANENTLY)
            .append_header(("Location", "/"))
            .finish()),
    }
}
