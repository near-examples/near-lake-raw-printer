use futures::StreamExt;

use near_lake_framework::LakeConfig;

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    let config = LakeConfig {
        bucket: "near-lake-testnet".to_string(),
        region: "eu-central-1".to_string(),
        start_block_height: None, // want to start from the freshest
        tracked_shards: vec![],
    };
    let stream = near_lake_framework::streamer(config);

    let mut handlers = tokio_stream::wrappers::ReceiverStream::new(stream)
        .map(|json_value| handle_json_value(json_value))
        .buffer_unordered(1usize);

    while let Some(_handle_message) = handlers.next().await {}

    Ok(())
}


async fn handle_json_value(json_value: serde_json::Value) {
    eprintln!("{} / shards {}", json_value["block"]["header"]["height"], json_value["shards"].as_array().unwrap().len());
}
