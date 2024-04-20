use models::generated::ratings;
use sea_orm::{prelude::*, DatabaseConnection};

const LOG_KEY: &str = "[Operations::Ratings::Fetch::ByUserAndMovie]: ";

pub async fn execute(
    tmdb_id: u32,
    user_id: i32,
    db: DatabaseConnection,
) -> Result<ratings::Model, DbErr> {
    match ratings::Entity::find()
        .filter(ratings::Column::TmdbId.eq(tmdb_id))
        .filter(ratings::Column::UserId.eq(user_id))
        .one(&db)
        .await
    {
        Ok(opt) => match opt {
            Some(v) => Ok(v),
            None => Err(DbErr::RecordNotFound(user_id.to_string())),
        },
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
