use kafka::config::KafkaConnectionConfig;
use kafka::consumer::{consume_message, consumer_mess, create_kafka_consumer, run_concurrently};
use kafka::producer::{KafkaMessage, KafkaProducer};

#[tokio::main]
async fn main() {
    let conn_config = KafkaConnectionConfig {
        bootstrap_servers: String::from("0.0.0.0:9092"),
    };
    let producer = KafkaProducer::new(&conn_config, String::from("test-redpanda"));
    for i in 0..10 {
        let _ = producer.produce(KafkaMessage {key: "username".to_string(), value: format!("tarik{}", i)}).await;
    };
    
    let consumer = create_kafka_consumer(&conn_config, "test-redpanda", "group_test");
    let _ = consumer_mess(consumer).await;

    // let _ = run_concurrently("test-redpanda".to_string(), "group_test".to_string(), 5).await;
}
