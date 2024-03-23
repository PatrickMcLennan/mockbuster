use models::generated::{ratings, users};
use sea_orm::{prelude::*, DatabaseConnection, QueryOrder, QuerySelect};

const LOG_KEY: &str = "[Operations::Ratings::List]: ";

pub async fn execute(
    start_cursor: u64,
    end_cursor: u64,
    db: DatabaseConnection,
) -> Result<Vec<(ratings::Model, Option<users::Model>)>, DbErr> {
    match ratings::Entity::find()
        .order_by_desc(ratings::Column::CreatedAt)
        .find_also_related(users::Entity)
        .offset(start_cursor)
        .limit(end_cursor - start_cursor)
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
