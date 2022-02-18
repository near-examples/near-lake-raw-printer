use futures::StreamExt;
use serde_json::json;

use near_lake_framework::LakeConfig;

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    let config = LakeConfig {
        s3_bucket_name: "near-lake-testnet".to_string(),
        start_block_height: 82422587,
        s3_region_name: "eu-central-1".to_string(),
    };
    let stream = near_lake_framework::streamer(config);

    let mut handlers = tokio_stream::wrappers::ReceiverStream::new(stream)
        .map(|streamer_message| handle_json_value(json!(streamer_message)))
        .buffer_unordered(1usize);

    while let Some(_handle_message) = handlers.next().await {}

    Ok(())
}

async fn handle_json_value(json_value: serde_json::Value) {
    eprintln!(
        "{} / shards {}",
        json_value["block"]["header"]["height"],
        json_value["shards"].as_array().unwrap().len()
    );
}
