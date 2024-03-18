use models::generated::aggregate_ratings;
use sea_orm::{prelude::*, DatabaseConnection, QueryOrder, QuerySelect};

pub async fn execute(db: DatabaseConnection) -> Vec<aggregate_ratings::Model> {
    match aggregate_ratings::Entity::find()
        .order_by_desc(aggregate_ratings::Column::Score)
        .limit(20)
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
