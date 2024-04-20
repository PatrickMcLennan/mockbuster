use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::OwnedMessage;
use rdkafka::Message;
use sea_orm::{Database, DatabaseConnection};
use std::sync::Mutex;

mod topics;

const KAFKA_BROKERS: &str = "kafka:9092";
const LOG_KEY: &str = "[Consumers]";

pub struct KafkaConsumer {
    consumer: StreamConsumer,
    pool: Mutex<DatabaseConnection>,
    isahc_client: Mutex<web_push::IsahcWebPushClient>,
}

impl KafkaConsumer {
    pub async fn new(brokers: &str, group_id: &str) -> KafkaConsumer {
        let mut kafka_config = ClientConfig::new();
        kafka_config.set("bootstrap.servers", brokers);
        kafka_config.set("group.id", group_id);

        let consumer: StreamConsumer = kafka_config.create().expect("Consumer creation error");

        KafkaConsumer {
            consumer,
            pool: Mutex::new(
                Database::connect(std::env::var("DATABASE_URL").expect("NO_POSTGRES_URL_IN_ENV"))
                    .await
                    .unwrap(),
            ),
            isahc_client: Mutex::new(web_push::IsahcWebPushClient::new().unwrap()),
        }
    }

    pub async fn start(&self, topics: &[&str]) {
        self.consumer.subscribe(topics).unwrap();

        loop {
            let pool = self.pool.lock().unwrap();
            let mut message: Option<OwnedMessage> = None;
            match self.consumer.recv().await {
                Ok(v) => message = Some(v.detach()),
                Err(e) => {
                    println!("{}: {}", LOG_KEY, e);
                    println!("Consumer Configuration: {:?}", self.consumer.context());
                    println!("Subscribed Topics: {:?}", self.consumer.subscription());
                }
            };

            match message {
                Some(_message) => match _message.topic() {
                    "COMMENT_CREATE" => topics::comment_create::execute(&*pool, _message).await,
                    _ => topics::comment_create::execute(&*pool, _message).await,
                },
                None => (),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let consumer = KafkaConsumer::new(KAFKA_BROKERS, "mockbuster-1").await;

    consumer.start(&["COMMENT_CREATE"]).await;
    ()
}
