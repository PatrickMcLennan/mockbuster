use models::generated::ratings as ratings_model;
use sea_orm::{prelude::*, ActiveValue::Set, DatabaseConnection};

use crate::operations::{aggregate_ratings, ratings as ratings_operations};

pub async fn execute(score: f32, user_id: i32, tmdb_id: u32, db: DatabaseConnection) -> Option<()> {
    let rating = ratings_model::ActiveModel {
        score: Set(score.to_owned()),
        user_id: Set(user_id.to_owned()),
        tmdb_id: Set((tmdb_id as i32).to_owned()),
        ..Default::default()
    };

    match ratings_model::Entity::insert(rating).exec(&db).await {
        Ok(_) => (),
        Err(e) => {
            return {
                println!("Error: {:?}", e);
                None
            }
        }
    };

    let new_summed_score = ratings_operations::summed::execute(tmdb_id as i32, db.clone())
        .await
        .unwrap();

    match aggregate_ratings::upsert::execute(tmdb_id, new_summed_score.weighted_average as f64, db)
        .await
    {
        Some(_) => Some(()),
        None => None,
    }
}
