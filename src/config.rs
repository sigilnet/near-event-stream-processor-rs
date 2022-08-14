use std::collections::HashMap;

use derive_builder::Builder;
use rdkafka::ClientConfig;

#[derive(Builder, Default, Debug)]
#[builder(pattern = "owned")]
pub struct StreamerConfig {
    pub kafka_config: HashMap<String, String>,

    pub topics: Vec<String>,

    pub group_id: String,

    #[builder(default = r#""earliest".to_string()"#)]
    pub auto_offset_reset: String,

    #[builder(default = "6000")]
    pub session_timeout: usize,

    #[builder(default = "32")]
    pub preload_pool_size: usize,
}

impl StreamerConfig {
    pub fn build_kafka_config(&self) -> ClientConfig {
        let mut kafka_conf = ClientConfig::new();
        self.kafka_config.iter().for_each(|(k, v)| {
            kafka_conf.set(k, v);
        });

        kafka_conf.set("enable.auto.commit", "false");
        kafka_conf.set("enable.partition.eof", "false");
        kafka_conf.set("session.timeout.ms", self.session_timeout.to_string());
        kafka_conf.set("group.id", self.group_id.to_owned());
        kafka_conf.set("auto.offset.reset", self.auto_offset_reset.to_owned());

        kafka_conf
    }
}
