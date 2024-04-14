use crate::operations::subscriptions;
use actix_session::Session;
use actix_web::{get, web, web::Data, Error as ActixError, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Deserialize)]
struct SubscribeRequest {
    endpoint: String,
    p256: String,
    auth: String,
}

#[get("/subscribe")]
async fn post(
    req: web::Json<SubscribeRequest>,
    db: Data<DatabaseConnection>,
    session: Session,
) -> Result<HttpResponse, ActixError> {
    let user_id = match session.get::<i32>("id") {
        Ok(Some(id)) => id,
        _ => return Ok(HttpResponse::Unauthorized().finish()),
    };

    match subscriptions::create::execute(
        req.endpoint.to_string(),
        req.p256.to_string(),
        req.auth.to_string(),
        user_id,
        db.get_ref().clone(),
    )
    .await
    {
        Ok(_subscription) => Ok(HttpResponse::Ok().finish()),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
