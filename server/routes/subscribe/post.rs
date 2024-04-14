use crate::operations::subscriptions;
use actix_session::Session;
use actix_web::{post, web, web::Data, Error as ActixError, HttpResponse};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SubscribeRequest {
    endpoint: String,
    p256: String,
    auth: String,
}

#[post("/subscribe")]
async fn post(
    req: web::Json<SubscribeRequest>,
    db: Data<DatabaseConnection>,
    session: Session,
) -> Result<HttpResponse, ActixError> {
    println!("{:?}", req);
    let user_id = match session.get::<i32>("id") {
        Ok(Some(id)) => id,
        _ => return Ok(HttpResponse::Unauthorized().finish()),
    };

    println!("{:?}", user_id);

    match subscriptions::upsert::execute(
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
