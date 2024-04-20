use models::generated::aggregate_ratings;
use sea_orm::{prelude::*, DatabaseConnection};

const LOG_KEY: &str = "[Operations::AggregateRatings::Fetch]: ";

pub async fn execute(
    tmdb_id: u32,
    db: DatabaseConnection,
) -> Result<aggregate_ratings::Model, DbErr> {
    match aggregate_ratings::Entity::find()
        .filter(aggregate_ratings::Column::TmdbId.eq(tmdb_id))
        .one(&db)
        .await
    {
        Ok(v) => match v {
            Some(v) => Ok(v),
            None => Err(DbErr::RecordNotFound(tmdb_id.to_string())),
        },
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
