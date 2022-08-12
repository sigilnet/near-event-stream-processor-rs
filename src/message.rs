use std::{sync::Arc, time::Duration};

use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    error::KafkaResult,
    message::OwnedMessage,
    types::RDKafkaErrorCode,
    Message, Offset, TopicPartitionList,
};
use tracing::warn;

pub struct StreamerMessage {
    pub message: OwnedMessage,
    consumer: Arc<StreamConsumer>,
}

impl StreamerMessage {
    pub fn new(message: OwnedMessage, consumer: Arc<StreamConsumer>) -> Self {
        Self { message, consumer }
    }

    pub async fn commit(&self) -> KafkaResult<()> {
        let mut tpl = TopicPartitionList::new();
        tpl.add_partition_offset(
            self.message.topic(),
            self.message.partition(),
            Offset::Offset(self.message.offset() + 1),
        )?;

        loop {
            let result = self.consumer.commit(&tpl, CommitMode::Sync);
            match result {
                Ok(()) => {
                    return Ok(());
                }
                Err(err) => {
                    if err.rdkafka_error_code() == Some(RDKafkaErrorCode::RebalanceInProgress) {
                        warn!("RebalanceInProgress, retry after 5s");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                    return Err(err);
                }
            }
        }
    }

    pub fn fetch_all_topics(&self) -> anyhow::Result<Vec<String>> {
        let metadata = self.consumer.fetch_metadata(None, Duration::from_secs(1))?;

        let topics: Vec<String> = metadata
            .topics()
            .iter()
            .map(|t| t.name().to_string())
            .collect();

        Ok(topics)
    }
}
