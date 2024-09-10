use kafka::config::KafkaConnectionConfig;
use kafka::consumer::{consume_message, create_kafka_consumer};
use kafka::producer::{KafkaMessage, KafkaProducer};

#[tokio::main]
async fn main() {
    let conn_config = KafkaConnectionConfig {
        bootstrap_servers: String::from("0.0.0.0:9092"),
    };
    // let producer = KafkaProducer::new(&conn_config, String::from("test-redpanda"));
    // let _ = producer.produce(KafkaMessage {key: "username".to_string(), value: "tarik2".to_string()}).await;
    let consumer = create_kafka_consumer(&conn_config, "test-redpanda", "group_test");
    let _ = consume_message(consumer).await;
}
