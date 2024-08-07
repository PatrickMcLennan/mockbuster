use actix_web::{post, web::Data, web::Json, Error as ActixError, HttpResponse, Responder};
use validators::tmdb_movies::search_dto::SearchDTO;

use operations::tmdb_movies;

#[post("/search")]
async fn post(
    Json(dto): Json<SearchDTO>,
    http_client: Data<reqwest_middleware::ClientWithMiddleware>,
) -> Result<impl Responder, ActixError> {
    match dto.get_errors() {
        Some(e) => return Ok(HttpResponse::BadRequest().body(e.to_json())),
        None => (),
    };

    match tmdb_movies::search::execute(dto, Some(http_client.as_ref().clone())).await {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(_) => Ok(HttpResponse::ServiceUnavailable().finish()),
    }
}
