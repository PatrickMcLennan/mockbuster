use models::generated::aggregate_ratings;
use sea_orm::{prelude::*, DatabaseConnection, QueryOrder, QuerySelect};

const LOG_KEY: &str = "[Operations::AggregateRatings::List]: ";

pub async fn execute(db: DatabaseConnection) -> Result<Vec<aggregate_ratings::Model>, DbErr> {
    match aggregate_ratings::Entity::find()
        .order_by_desc(aggregate_ratings::Column::Score)
        .limit(20)
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
