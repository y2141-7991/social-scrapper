use kafka::producer::{KafkaMessage, KafkaProducer};
use kafka::consumer::{create_kafka_consumer, consume};
use kafka::config::KafkaConnectionConfig;


#[tokio::main]
async fn main() {
    let conn_config = KafkaConnectionConfig {bootstrap_servers: String::from("0.0.0.0:9092")};
    let producer = KafkaProducer::new(&conn_config, String::from("test-redpanda"));
    let _ = producer.produce(KafkaMessage {key: "username".to_string(), value: "tarik".to_string()}).await;
    let consumer = create_kafka_consumer(&conn_config, "test-redpanda", "group_test");
    let _ = consume(consumer).await;
}

