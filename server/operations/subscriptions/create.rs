use models::generated::subscriptions;
use sea_orm::{prelude::*, ActiveValue::Set, DatabaseConnection};

const LOG_KEY: &str = "[Operations::Subscriptions::Create]: ";

pub async fn execute(
    endpoint: String,
    p256: String,
    auth: String,
    user_id: i32,
    db: DatabaseConnection,
) -> Result<subscriptions::ActiveModel, DbErr> {
    let subscription = subscriptions::ActiveModel {
        endpoint: Set(endpoint.to_string()),
        p256: Set(p256.to_string()),
        auth: Set(auth.to_string()),
        user_id: Set(user_id.to_owned()),
        ..Default::default()
    };

    match subscriptions::Entity::insert(subscription.clone())
        .exec(&db)
        .await
    {
        Ok(_) => Ok(subscription),
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
