use models::generated::ratings as ratings_model;
use sea_orm::{prelude::*, ActiveValue::Set, DatabaseConnection};

use crate::{aggregate_ratings, ratings as ratings_operations};

const LOG_KEY: &str = "[Operations::Ratings::Create]: ";

pub async fn execute(
    score: f32,
    user_id: i32,
    tmdb_id: u32,
    db: DatabaseConnection,
) -> Result<ratings_model::ActiveModel, DbErr> {
    let rating = ratings_model::ActiveModel {
        score: Set(score.to_owned()),
        user_id: Set(user_id.to_owned()),
        tmdb_id: Set((tmdb_id as i32).to_owned()),
        created_at: Set(chrono::Utc::now().fixed_offset()),
        updated_at: Set(chrono::Utc::now().fixed_offset()),
        ..Default::default()
    };

    match ratings_model::Entity::insert(rating.clone())
        .exec(&db)
        .await
    {
        Ok(_) => (),
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            return Err(e);
        }
    };

    // Calculating + writing the new aggregate_score should happen in a downstream consumer.  However
    // with so few users the UX will seem delayed when the aggregate score doesn't happen syncronously
    // with your rating.  With thousands of users, an aggregate score not updating immediately makes sense.
    // With < 20, it will seem like a bug.

    // Having this side effect in here is less than ideal, this would be a bottleneck in a larger app.
    // But it's not a large app, and updating the aggregate here makes more sense than standing up a downstream
    // consumer that acts on this async, and trying to monkey-patch that UX for a small handful of users.

    let new_summed_score = ratings_operations::summed::execute(tmdb_id as i32, db.clone())
        .await
        .unwrap();

    match aggregate_ratings::upsert::execute(tmdb_id, new_summed_score.weighted_average as f64, db)
        .await
    {
        Ok(_) => Ok(rating),
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Ok(rating.clone())
        }
    }
}
