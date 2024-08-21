use envconfig::Envconfig;
use rdkafka::ClientConfig;

#[derive(Envconfig, Debug, Clone)]
pub struct KafkaConnectionConfig {
    #[envconfig(from = "", default = "localhost:9092")]
    pub bootstrap_servers: String,
}

impl KafkaConnectionConfig {
    pub fn to_client_config(&self) -> ClientConfig {
        let mut client_config = ClientConfig::new();
        client_config.set("bootstrap.servers", &self.bootstrap_servers);
        client_config
    }
}
