use models::generated::{ratings, users};
use sea_orm::{prelude::*, DatabaseConnection};

const LOG_KEY: &str = "[Operations::Ratings::Fetch::ByMovie]: ";

pub async fn execute(
    tmdb_id: u32,
    db: DatabaseConnection,
) -> Result<Vec<(ratings::Model, Option<users::Model>)>, DbErr> {
    match ratings::Entity::find()
        .filter(ratings::Column::TmdbId.eq(tmdb_id))
        .find_also_related(users::Entity)
        .all(&db)
        .await
    {
        Ok(vec) => Ok(vec),
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
