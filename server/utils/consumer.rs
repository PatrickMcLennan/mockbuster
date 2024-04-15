use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};

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
