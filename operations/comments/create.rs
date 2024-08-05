use models::generated::comments;
use sea_orm::{prelude::*, ActiveValue::Set, DatabaseConnection};

const LOG_KEY: &str = "[Operations::Comments::Create]: ";

pub async fn execute(
    content: String,
    user_id: i32,
    tmdb_id: u32,
    db: DatabaseConnection,
) -> Result<comments::ActiveModel, DbErr> {
    if content.len() > 250 {
        return Err(DbErr::RecordNotInserted);
    }

    let comment = comments::ActiveModel {
        content: Set(content.to_owned()),
        user_id: Set(user_id.to_owned()),
        tmdb_id: Set((tmdb_id as i32).to_owned()),
        created_at: Set(chrono::Utc::now().fixed_offset()),
        updated_at: Set(chrono::Utc::now().fixed_offset()),
        ..Default::default()
    };

    match comments::Entity::insert(comment.clone()).exec(&db).await {
        Ok(_last_inserted) => {
            // TODO: Send notifications
            Ok(comment)
        }
        Err(e) => {
            return {
                println!("{}{:?}", LOG_KEY, e);
                Err(e)
            }
        }
    }
}
