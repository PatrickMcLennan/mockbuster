use models::generated::comments;
use sea_orm::{prelude::*, ActiveValue::Set, DatabaseConnection};

pub async fn execute(
    content: String,
    user_id: i32,
    tmdb_id: u32,
    db: DatabaseConnection,
) -> Option<()> {
    if content.len() > 250 {
        // TODO: Handle this better
        return None;
    }

    let comment = comments::ActiveModel {
        content: Set(content.to_owned()),
        user_id: Set(user_id.to_owned()),
        tmdb_id: Set((tmdb_id as i32).to_owned()),
        ..Default::default()
    };

    match comments::Entity::insert(comment).exec(&db).await {
        Ok(_) => Some(()),
        Err(e) => {
            return {
                println!("Error: {:?}", e);
                None
            }
        }
    }
}
