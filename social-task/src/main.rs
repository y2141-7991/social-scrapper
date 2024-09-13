use kafka::config::KafkaConnectionConfig;
use kafka::consumer::{consume_message, consumer_mess, create_kafka_consumer, run_concurrently};
use kafka::producer::{KafkaMessage, KafkaProducer};

use social::services::*;
use social_store::store::Store;

// #[tokio::main]
// async fn main() {
// let conn_config = KafkaConnectionConfig {
//     let consumer = create_kafka_consumer(&conn_config, "test-redpanda", "group_test");
//     let _ = consumer_mess(consumer).await;

// let _ = run_concurrently("test-redpanda".to_string(), "group_test".to_string(), 5).await;
// }

macro_rules! produce_message {
    ($func_name:ident, $k:expr, $v:expr) => {
            {
            let conn_config = KafkaConnectionConfig {
                bootstrap_servers: String::from("0.0.0.0:9092"),
            };
            let producer = KafkaProducer::new(&conn_config, String::from(stringify!($func_name)));
            let _ = producer.produce(KafkaMessage { key: $k, value: $v }).await;
        }
    };
}


#[tokio::main]
async fn main() {
    let store = Store::new("postgres://postgres:password@localhost:5432/postgres".to_string());
    let service = SocialAccountCrawlingComponents{ store };
    // let acc = service.crawl_twitch_account_by_username("tarik").await;
    produce_message!(crawl_twitch_account_by_username, "username", "tarik");
    println!()
}
