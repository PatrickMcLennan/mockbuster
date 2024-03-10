use models::generated::{ratings, users};
use sea_orm::{prelude::*, DatabaseConnection};

pub async fn execute(id: i32, db: DatabaseConnection) -> Vec<(users::Model, Vec<ratings::Model>)> {
    match users::Entity::find_by_id(id)
        .find_with_related(ratings::Entity)
        .all(&db)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
        }
    }
}
