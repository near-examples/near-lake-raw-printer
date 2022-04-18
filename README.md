# near-lake-raw-printer

This is a small indexer project built on top of [NEAR Lake Framework](https://github.com/near/near-lake-framework). This indexer is created to provide minimal working example of how to use the framework.

You may use it as a starting point in creating your own indexer based on NEAR Lake Framework.

## How it works

`near-lake-raw-printer` indexer connects to the NEAR Lake data source (AWS S3) and starts the block stream from the specified block height. You can adjust it for your needs in `LakeConfig` instantiating

```rust
let config = LakeConfig {
    s3_endpoint: None,
    s3_bucket_name: "near-lake-data-testnet".to_string(),
    s3_region_name: "eu-central-1".to_string(),
    start_block_height: 82800242, // want to start from the freshest
};
```

The main logic of what happens is defined in the function `handle_streamer_message`. In this indexer is just prints the observed block height and the number of shards in that block (number of included chunks).

You can write your logic in this function:

```rust
async fn handle_streamer_message(
    streamer_message: near_lake_framework::near_indexer_primitives::StreamerMessage,
) {
    eprintln!(
        "{} / shards {}",
        streamer_message.block.header.height,
        streamer_message.shards.len()
    );
}
```

Don't forget you need to provide valid AWS credentials in order to be charged by AWS for the S3 usage.

### AWS S3 Credentials
In order to be able to get objects from the AWS S3 bucket you need to provide the AWS credentials.

AWS default profile configuration with aws configure looks similar to the following:

```
~/.aws/credentials
```

```
[default]
aws_access_key_id=AKIAIOSFODNN7EXAMPLE
aws_secret_access_key=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
```

[AWS docs: Configuration and credential file settings](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html)
