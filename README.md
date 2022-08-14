# NEAR Event Stream Processor

## Usage

```toml
[dependencies]
near-event-stream-processor = "0.0.1"

```

```rust
#[derive(Deserialize, Debug)]
pub struct EmitInfo {
    pub receipt_id: String,
    pub block_timestamp: u64,
    pub block_height: u64,
    pub shard_id: u64,
    pub contract_account_id: String,
}

#[derive(Deserialize, Debug)]
pub struct GenericEvent {
    pub standard: String,
    pub version: String,
    pub event: String,
    pub data: serde_json::Value,
    pub emit_info: Option<EmitInfo>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut kafka_config: HashMap<String, String> = HashMap::new();
    kafka_config.insert(
        "bootstrap.servers".to_string(),
        "localhost:29092".to_string(),
    );

    let streamer_config = StreamerConfigBuilder::default()
        .kafka_config(kafka_config)
        .group_id("example-group".to_string())
        .auto_offset_reset("earliest".to_string())
        .topics(vec!["localnet.events.all".to_string()])
        .build()?;

    let (_, mut stream) = streamer(&streamer_config)?;

    while let Some(streamer_message) = stream.recv().await {
        let event = streamer_message.event::<GenericEvent>();
        match event {
            Ok(event) => println!("Received event: {:?}", event),
            Err(err) => println!("Error: {:?}", err),
        }
        streamer_message.commit().await?;
    }

    Ok(())
}
```
