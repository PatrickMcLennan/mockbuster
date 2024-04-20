use models::generated::aggregate_ratings;
use sea_orm::{prelude::*, sea_query::OnConflict, ActiveValue, DatabaseConnection};

const LOG_KEY: &str = "[Operations::AggregateRatings::Upsert]: ";

pub async fn execute(
    tmdb_id: u32,
    score: f64,
    db: DatabaseConnection,
) -> Result<aggregate_ratings::ActiveModel, DbErr> {
    let new_ratings = aggregate_ratings::ActiveModel {
        score: ActiveValue::Set(score as f32),
        tmdb_id: ActiveValue::Set(tmdb_id as i32),
        ..Default::default()
    };

    match aggregate_ratings::Entity::insert(new_ratings.clone())
        .on_conflict(
            OnConflict::column(aggregate_ratings::Column::TmdbId)
                .update_column(aggregate_ratings::Column::Score)
                .to_owned(),
        )
        .exec(&db)
        .await
    {
        Ok(_) => Ok(new_ratings),
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
