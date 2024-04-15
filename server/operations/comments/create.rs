use crate::utils::producer;
use actix_web::web::Data;
use models::generated::comments;
use sea_orm::{prelude::*, ActiveValue::Set, DatabaseConnection};
use serde_json::json;

const LOG_KEY: &str = "[Operations::Comments::Create]: ";

pub async fn execute(
    content: String,
    user_id: i32,
    tmdb_id: u32,
    db: DatabaseConnection,
    kafka_producer: Data<producer::KafkaProducer>,
) -> Result<comments::ActiveModel, DbErr> {
    if content.len() > 250 {
        return Err(DbErr::RecordNotInserted);
    }

    let comment = comments::ActiveModel {
        content: Set(content.to_owned()),
        user_id: Set(user_id.to_owned()),
        tmdb_id: Set((tmdb_id as i32).to_owned()),
        ..Default::default()
    };

    match comments::Entity::insert(comment.clone()).exec(&db).await {
        Ok(last_inserted) => {
            kafka_producer
                .send_message(
                    "comments",
                    "COMMENT_CREATE".to_string(),
                    json!({
                        "id": last_inserted.last_insert_id
                    }),
                )
                .await;
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
