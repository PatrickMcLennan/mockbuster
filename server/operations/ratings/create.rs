use models::generated::ratings;
use sea_orm::{prelude::*, ActiveValue::Set, DatabaseConnection};

pub async fn execute(
    score: f32,
    user_id: i32,
    tmdb_id: u32,
    db: DatabaseConnection,
) -> Result<(), DbErr> {
    let rating = ratings::ActiveModel {
        score: Set(score.to_owned()),
        user_id: Set(user_id.to_owned()),
        tmdb_id: Set((tmdb_id as i32).to_owned()),
        ..Default::default()
    };

    match ratings::Entity::insert(rating).exec(&db).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
