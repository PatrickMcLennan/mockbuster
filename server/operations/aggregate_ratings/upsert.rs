use models::generated::aggregate_ratings;
use sea_orm::{prelude::*, sea_query::OnConflict, ActiveValue, DatabaseConnection};

pub async fn execute(tmdb_id: u32, score: f64, db: DatabaseConnection) -> Option<i32> {
    let new_record = aggregate_ratings::ActiveModel {
        score: ActiveValue::Set(score as f32),
        tmdb_id: ActiveValue::Set(tmdb_id as i32),
        ..Default::default()
    };

    match aggregate_ratings::Entity::insert(new_record)
        .on_conflict(
            OnConflict::column(aggregate_ratings::Column::TmdbId)
                .update_column(aggregate_ratings::Column::Score)
                .to_owned(),
        )
        .exec(&db)
        .await
    {
        Ok(v) => Some(v.last_insert_id),
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
    }
}
