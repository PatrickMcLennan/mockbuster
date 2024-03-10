use models::generated::{ratings, users};
use sea_orm::{prelude::*, DatabaseConnection, QueryOrder, QuerySelect};

pub async fn execute(
    start_cursor: u64,
    finish_cursor: u64,
    db: DatabaseConnection,
) -> Vec<(ratings::Model, Option<users::Model>)> {
    match ratings::Entity::find()
        .order_by_desc(ratings::Column::CreatedAt)
        .find_also_related(users::Entity)
        .limit(finish_cursor - start_cursor)
        .all(&db)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
        }
    }
}
