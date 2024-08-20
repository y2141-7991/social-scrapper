use crate::config::KafkaConnectionConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};

pub fn create_kafka_consumer(conn_config: &KafkaConnectionConfig, topic: &str, group_id: &str) -> StreamConsumer {
    let mut client_config = KafkaConnectionConfig::to_client_config(conn_config);

    let consumer: StreamConsumer = client_config
        .set("key", "earliest")
        .set("group.id", group_id)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Failed to create Kafka consumer");
    consumer.subscribe(&[topic]).expect("Failed to subcribe topic");
    log::info!("Kafka consumer created and subcribed to topic '{}' with group id '{}'", topic, group_id);
    consumer
}
