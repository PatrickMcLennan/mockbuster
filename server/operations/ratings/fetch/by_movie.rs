use models::generated::{aggregate_ratings, ratings, users};
use sea_orm::{prelude::*, DatabaseConnection};

pub async fn execute(
    tmdb_id: u32,
    db: DatabaseConnection,
) -> Vec<(ratings::Model, Option<users::Model>)> {
    match ratings::Entity::find()
        .filter(ratings::Column::TmdbId.eq(tmdb_id))
        .find_also_related(users::Entity)
        .all(&db)
        .await
    {
        Ok(vec) => vec,
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
        }
    }
}
