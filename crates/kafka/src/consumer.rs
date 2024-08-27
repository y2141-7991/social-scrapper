use crate::config::KafkaConnectionConfig;
use rdkafka::{consumer::{CommitMode, Consumer, StreamConsumer}, message, Message};

pub fn create_kafka_consumer(conn_config: &KafkaConnectionConfig, topic: &str, group_id: &str) -> StreamConsumer {
    let mut client_config = KafkaConnectionConfig::to_client_config(conn_config);

    let consumer: StreamConsumer = client_config
        .set("auto.offset.reset", "earliest")
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

pub async fn consume_message(consumer: StreamConsumer) {
    loop {
        match consumer.recv().await {
            Err(e) => println!("{:?}", e),
            Ok(message) => {
                match message.payload_view::<str>() {
                    None => println!("No messages"),
                    Some(Ok(msg)) => println!("Message consumed: {}", msg),
                    Some(Err(e)) => println!("Message Error: {}", e),
                }
                consumer.commit_message(&message, CommitMode::Async).unwrap();
            }
        }
    }
}