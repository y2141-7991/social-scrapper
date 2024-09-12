use crate::config::KafkaConnectionConfig;
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer}, Message
};
use futures::stream::FuturesOrdered;
use futures::StreamExt;

pub fn create_kafka_consumer(
    conn_config: &KafkaConnectionConfig,
    topic: &str,
    group_id: &str,
) -> StreamConsumer {
    let mut client_config = KafkaConnectionConfig::to_client_config(conn_config);

    let consumer: StreamConsumer = client_config
        .set("auto.offset.reset", "earliest")
        .set("group.id", group_id)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Failed to create Kafka consumer");
    consumer
        .subscribe(&[topic])
        .expect("Failed to subcribe topic");
    log::info!(
        "Kafka consumer created and subcribed to topic '{}' with group id '{}'",
        topic,
        group_id
    );
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
                consumer
                    .commit_message(&message, CommitMode::Async)
                    .unwrap();
            }
        }
    }
}

pub async fn consumer_mess(consumer: StreamConsumer) {
    while let Some(Ok(msg)) = consumer.stream().next().await {
        let _ = consumer.commit_message(&msg, CommitMode::Async);
        if let Some(Ok(msg)) = msg.payload_view::<str>() {
            println!("Consumed: {}", msg);
        }
    }
}

pub async fn run_concurrently(topic: String, group_id: String, number_workers: i32) {
    (0..number_workers).map(|_| {
        tokio::spawn(create_kafka_consumer_concurrently(topic.to_owned(), group_id.to_owned()
        ))
    }).collect::<FuturesOrdered<_>>().for_each(|_| async {}).await
}


pub async fn create_kafka_consumer_concurrently(topic: String, group_id: String) {
    let conn_config = KafkaConnectionConfig { bootstrap_servers: String::from("0.0.0.0:9092") };
    let mut client_config = KafkaConnectionConfig::to_client_config(&conn_config);
    let consumer: StreamConsumer = client_config
        .set("auto.offset.reset", "earliest")
        .set("group.id", &group_id)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Failed to create Kafka consumer");
    consumer
        .subscribe(&[&topic])
        .expect("Failed to subcribe topic");
    log::info!(
        "Kafka consumer created and subcribed to topic '{}' with group id '{}'",
        topic,
        &group_id
    );

    while let Some(Ok(msg)) = consumer.stream().next().await {
        if let Some(Ok(msg)) = msg.payload_view::<str>() {
            println!("Consumed: {}", msg)
        };
        let _ = consumer.commit_message(&msg, CommitMode::Async);
    };
}