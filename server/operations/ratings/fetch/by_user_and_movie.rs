use models::generated::ratings;
use sea_orm::{prelude::*, DatabaseConnection};

pub async fn execute(tmdb_id: u32, user_id: i32, db: DatabaseConnection) -> Option<ratings::Model> {
    match ratings::Entity::find()
        .filter(ratings::Column::TmdbId.eq(tmdb_id))
        .filter(ratings::Column::UserId.eq(user_id))
        .one(&db)
        .await
    {
        Ok(opt) => opt,
        Err(e) => {
            println!("Error: {:?}", e);
            None
        }
    }
}
