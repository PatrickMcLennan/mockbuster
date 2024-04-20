use models::generated::ratings;
use sea_orm::{prelude::*, DatabaseConnection, QueryOrder, QuerySelect};

const LOG_KEY: &str = "[Operations::Ratings::Fetch::ByUser]: ";

pub async fn execute(user_id: u32, db: DatabaseConnection) -> Result<Vec<ratings::Model>, DbErr> {
    match ratings::Entity::find()
        .order_by_desc(ratings::Column::CreatedAt)
        .filter(ratings::Column::UserId.eq(user_id))
        .limit(10)
        .all(&db)
        .await
    {
        Ok(r) => Ok(r),
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
