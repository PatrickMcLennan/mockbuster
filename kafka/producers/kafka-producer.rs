use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::Value;
use std::time::Duration;

pub mod schemas;
pub mod topics;

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> KafkaProducer {
        let mut kafka_config = ClientConfig::new();
        kafka_config.set("bootstrap.servers", brokers);
        let producer: FutureProducer = kafka_config.create().expect("Producer creation error");

        KafkaProducer { producer }
    }

    pub async fn send_message(&self, topic: &topics::Topic, key: i32, value: Value) {
        let value_bytes = serde_json::to_vec(&value).expect("Error serializing JSON value");
        let key_string = key.to_string();

        let record = FutureRecord::to(match topic {
            topics::Topic::COMMENT_CREATE => topics::Topic::COMMENT_CREATE.as_str(),
            _ => "no_topic",
        })
        .key(&key_string)
        .payload(&value_bytes);

        match self.producer.send(record, Duration::from_millis(0)).await {
            Ok(_) => (),
            Err(e) => {
                println!("Error!: {:?}", e);
                ()
            }
        }
    }
}
