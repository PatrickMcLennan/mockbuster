use kafka_producer::schemas::create_comment::CreateCommentSchema;
use rdkafka::{message::OwnedMessage, Message};
use sea_orm::DatabaseConnection;

const LOG_KEY: &str = "[Consumers::COMMENT_CREATE]";

pub async fn execute(db: &DatabaseConnection, message: OwnedMessage) -> () {
    let mut payload_opt: Option<CreateCommentSchema> = None;

    match message.payload_view::<str>() {
        Some(Ok(json_str)) => {
            match kafka_producer::schemas::create_comment::CreateCommentSchema::from_json(json_str)
            {
                Ok(v) => {
                    println!("{}: {:?}", LOG_KEY, v);
                    payload_opt = Some(v)
                }
                Err(_) => println!("{}: Consumer could not consume message", LOG_KEY),
            }
        }
        Some(Err(e)) => println!("{}: {}", LOG_KEY, e),
        None => println!("{}: Received empty message", LOG_KEY),
    };

    let payload = match payload_opt {
        Some(p) => p,
        None => return (),
    };

    let users_to_notify = match operations::users::list::by_new_comment_notification::execute(
        db.clone(),
        payload.user_id,
        payload.tmdb_id,
    )
    .await
    {
        Ok(v) => v,
        Err(_e) => {
            println!("{}: ", LOG_KEY);
            return ();
        }
    };

    return ();
}
