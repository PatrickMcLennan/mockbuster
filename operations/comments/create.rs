use kafka_producer::{schemas, topics::Topic, KafkaProducer};
use models::generated::comments;
use sea_orm::{prelude::*, ActiveValue::Set, DatabaseConnection};

const LOG_KEY: &str = "[Operations::Comments::Create]: ";

pub async fn execute(
    content: String,
    user_id: i32,
    tmdb_id: u32,
    db: DatabaseConnection,
    kafka_producer: KafkaProducer,
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
            let last_inserted_id = last_inserted.last_insert_id;

            let message = schemas::create_comment::CreateCommentSchema {
                id: last_inserted_id,
                tmdb_id: tmdb_id as i32,
                user_id: user_id as i32,
            };

            kafka_producer
                .send_message(
                    &Topic::COMMENT_CREATE,
                    last_inserted.last_insert_id,
                    message.to_json(),
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
