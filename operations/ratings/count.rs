use models::generated::ratings as ratings_model;
use sea_orm::{prelude::*, DatabaseConnection};

const LOG_KEY: &str = "[Operations::Ratings::Count]: ";

pub async fn execute(db: DatabaseConnection) -> Result<u64, DbErr> {
    match ratings_model::Entity::find().count(&db).await {
        Ok(v) => Ok(v),
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
