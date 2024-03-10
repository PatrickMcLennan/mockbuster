use actix_web::{post, web::Data, web::Json, Error as ActixError, HttpResponse, Responder};
use serde_json::json;
use validators::tmdb_movies::search_dto::SearchDTO;

use crate::operations::tmdb_movies;

#[post("/search")]
async fn post(
    Json(dto): Json<SearchDTO>,
    http_client: Data<reqwest::Client>,
) -> Result<impl Responder, ActixError> {
    match dto.get_errors() {
        Some(e) => return Ok(HttpResponse::BadRequest().body(e.to_json())),
        None => (),
    };

    match tmdb_movies::search::execute(dto, Some(http_client.as_ref().clone())).await {
        Ok(v) => Ok(HttpResponse::Ok().json(v)),
        Err(e) => Ok(HttpResponse::ServiceUnavailable().json(&json!({"message": e}))),
    }
}
