use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use sea_orm::{Database, DatabaseConnection};

const KAFKA_BROKERS: &str = "kafka:9092";

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new(brokers: &str, group_id: &str) -> KafkaConsumer {
        let mut kafka_config = ClientConfig::new();
        kafka_config.set("bootstrap.servers", brokers);
        kafka_config.set("group.id", group_id);

        let consumer: StreamConsumer = kafka_config.create().expect("Consumer creation error");

        KafkaConsumer { consumer }
    }

    pub async fn start(&self, topics: &[&str]) {
        self.consumer.subscribe(topics).unwrap();

        loop {
            let message = self.consumer.recv().await.expect("Error receiving message");
            println!("Topic: {}", message.topic());
            println!("Partition: {}", message.partition());
            println!("Offset: {}", message.offset());
            println!("Key: {:?}", message.key());

            let key_str = match message.key() {
                Some(key_bytes) => String::from_utf8_lossy(key_bytes).to_string(),
                None => "No Key".to_string(), // Provide a default value for when key is missing
            };
            println!("Key: {}", key_str);

            match message.payload_view::<str>() {
                Some(Ok(payload)) => {
                    println!("Received message:");
                    println!("Payload: {}", payload);
                }
                Some(Err(e)) => println!("Error decoding message payload: {}", e),
                None => println!("Received empty message"),
            }

            println!("{:?}", message);
            // match message {
            //     Ok(m) => {
            //         println!("Received message: {:?}", m);
            //         // Process the message as needed (e.g., store in database, send to clients, etc.)
            //     }
            //     Err(e) => println!("Error receiving message: {:?}", e),
            // }
        }
    }
}

#[tokio::main]
async fn main() {
    let pool: DatabaseConnection =
        Database::connect(std::env::var("DATABASE_URL").expect("NO_POSTGRES_URL_IN_ENV"))
            .await
            .unwrap();

    let consumer = KafkaConsumer::new(KAFKA_BROKERS, "mockbuster-1");

    consumer.start(&["comments"]).await;
    ()
}
