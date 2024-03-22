use models::generated::{ratings, users};
use sea_orm::{prelude::*, DatabaseConnection};

const LOG_KEY: &str = "[Operations::Users::Fetch]: ";

pub async fn execute(
    id: i32,
    db: DatabaseConnection,
) -> Result<Vec<(users::Model, Vec<ratings::Model>)>, DbErr> {
    match users::Entity::find_by_id(id)
        .find_with_related(ratings::Entity)
        .all(&db)
        .await
    {
        Ok(r) => Ok(r),
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
