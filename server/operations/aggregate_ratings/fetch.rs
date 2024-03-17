use models::generated::aggregate_ratings;
use sea_orm::{prelude::*, DatabaseConnection};

pub async fn execute(tmdb_id: u32, db: DatabaseConnection) -> Option<aggregate_ratings::Model> {
    match aggregate_ratings::Entity::find()
        .filter(aggregate_ratings::Column::TmdbId.eq(tmdb_id))
        .one(&db)
        .await
    {
        Ok(v) => Some(v?),
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
    }
}
