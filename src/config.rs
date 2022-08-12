use std::collections::HashMap;

use derive_builder::Builder;
use rdkafka::ClientConfig;

#[derive(Builder, Default, Debug)]
#[builder(pattern = "owned")]
pub struct StreamerConfig {
    pub kafka_config: HashMap<String, String>,

    pub topics: Vec<String>,

    #[builder(default = "32")]
    pub preload_pool_size: usize,
}

impl StreamerConfig {
    pub fn build_kafka_config(&self) -> ClientConfig {
        let mut kafka_conf = ClientConfig::new();
        self.kafka_config.iter().for_each(|(k, v)| {
            kafka_conf.set(k, v);
        });

        kafka_conf
    }
}
