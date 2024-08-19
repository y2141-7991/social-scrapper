use crate::config::KafkaConnectionConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use std::time::Duration;

pub struct KafkaMessage<K: Serialize, V: Serialize> {
    pub key: K,
    pub value: V,
}

pub struct KafkaProducer {
    pub producer: FutureProducer,
    pub topic: String,
}

impl KafkaProducer {
    pub fn new(conn_config: &KafkaConnectionConfig, topic: String) -> Self {
        let mut producer_config = conn_config.to_client_config();
        let producer: FutureProducer = producer_config
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Could not create producer");
        Self { producer, topic }
    }

    pub async fn produce<K: Serialize, V: Serialize>(
        &self,
        message: KafkaMessage<K, V>,
    ) -> anyhow::Result<(i32, i64)> {
        let key_str = serde_json::to_string(&message.key).expect("Could not serialize key");
        let val_str = serde_json::to_string(&message.value).expect("Could not serialize value");
        let record = FutureRecord::to(self.topic.as_str())
            .payload(val_str.as_str())
            .key(key_str.as_str());
        self.producer
            .send(record, Duration::from_secs(30))
            .await
            .map_err(|(error, _)| {
                log::error!("Failed to publish records with key={key_str}, value={val_str}");
                anyhow::Error::new(error)
            })
    }
}
