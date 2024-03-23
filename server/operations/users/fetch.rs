use models::generated::{ratings, users};
use sea_orm::{prelude::*, DatabaseConnection};

const LOG_KEY: &str = "[Operations::Users::Fetch]: ";

pub async fn execute(
    id: i32,
    db: DatabaseConnection,
) -> Result<(users::Model, Vec<ratings::Model>), DbErr> {
    match users::Entity::find_by_id(id)
        .find_with_related(ratings::Entity)
        .all(&db)
        .await
    {
        Ok(r) => Ok(r[0].clone()),
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
