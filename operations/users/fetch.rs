use models::generated::users;
use sea_orm::{prelude::*, DatabaseConnection};

const LOG_KEY: &str = "[Operations::Users::Fetch]: ";

pub async fn execute(id: i32, db: DatabaseConnection) -> Result<users::Model, DbErr> {
    match users::Entity::find_by_id(id).one(&db).await {
        Ok(r) => match r {
            Some(user) => Ok(user),
            None => Err(DbErr::RecordNotFound(id.to_string())),
        },
        Err(e) => {
            println!("{}{:?}", LOG_KEY, e);
            Err(e)
        }
    }
}
