use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde_json::Value;
use std::time::Duration;

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

    pub async fn send_message(&self, topic: &str, key: String, value: Value) {
        let value_bytes = serde_json::to_vec(&value).expect("Error serializing JSON value");

        let record = FutureRecord::to(topic).key(&key).payload(&value_bytes);

        let _delivery_status = self
            .producer
            .send(record, Duration::from_millis(5000))
            .await;
    }
}
