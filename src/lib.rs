use std::sync::Arc;

use config::StreamerConfig;
use futures::StreamExt;
use message::StreamerMessage;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig,
};
use tokio::sync::mpsc;
use tracing::warn;

pub mod config;
pub mod error;
pub mod message;

#[allow(clippy::type_complexity)]
pub fn streamer(
    config: &StreamerConfig,
) -> anyhow::Result<(
    tokio::task::JoinHandle<Result<(), anyhow::Error>>,
    mpsc::Receiver<StreamerMessage>,
)> {
    let (sender, receiver) = mpsc::channel(config.preload_pool_size);

    let sender = tokio::spawn(start(
        sender,
        config.build_kafka_config(),
        config.topics.clone(),
    ));

    Ok((sender, receiver))
}

async fn start(
    sender: mpsc::Sender<StreamerMessage>,
    kafka_config: ClientConfig,
    topics: Vec<String>,
) -> anyhow::Result<()> {
    let consumer: StreamConsumer = kafka_config.create()?;

    let topics_ref: Vec<&str> = topics.iter().map(|topic| topic.as_str()).collect();
    consumer.subscribe(&topics_ref.to_vec())?;

    let consumer_arc = Arc::new(consumer);
    let mut stream = consumer_arc.stream();

    while let Some(message_result) = stream.next().await {
        match message_result {
            Err(e) => warn!("Kafka error: {}", e),
            Ok(borrowed_message) => {
                let message = borrowed_message.detach();
                if let Err(err) = sender
                    .send(StreamerMessage::new(message, consumer_arc.clone()))
                    .await
                {
                    tracing::error!("Channel closed, exiting with error {}", err);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
