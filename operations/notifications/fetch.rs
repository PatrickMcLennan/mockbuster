use models::generated::notifications;
use sea_orm::{prelude::*, DatabaseConnection, QueryOrder, QuerySelect};

const LOG_KEY: &str = "[Operations::Notifications::Fetch]: ";

pub async fn execute(
    user_id: i32,
    db: DatabaseConnection,
) -> Result<Vec<notifications::Model>, DbErr> {
    match notifications::Entity::find()
        .order_by_desc(notifications::Column::CreatedAt)
        .filter(notifications::Column::UserId.eq(user_id))
        .limit(10)
        .all(&db)
        .await
    {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
