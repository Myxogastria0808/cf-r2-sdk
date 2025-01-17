# Cloudflare R2 Operation SDK

## ⚠ This crate is under development ⚠

This is the Cloudflare R2 Operation SDK.

## Documentation

https://github.com/Myxogastria0808/cf-r2-sdk-sample

## Usage

Usage sample repository is https://github.com/Myxogastria0808/cf-r2-sdk-sample .

```rust
use cf_r2_sdk::utils::builder::Builder;
use dotenvy::dotenv;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // load .env file
    dotenv().expect(".env file not found.");
    // insert a environment variable
    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
    let endpoint_url: String =
        env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
    let access_key_id: String =
        env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
    let secret_access_key: String =
        env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");
    let region: String = env::var("REGION").expect("REGION not found in .env file.");

    // create a client object
    let object = Builder::new()
        .set_bucket_name(bucket_name)
        .set_access_key_id(access_key_id)
        .set_secret_access_key(secret_access_key)
        .set_endpoint(endpoint_url)
        .set_region(region)
        .create_client();

    // upload a binary data
    object
        .upload_binary("text.txt", "text/plain", b"Hello, World!")
        .await;

    // download data as binary
    let bin = object.download("text.txt").await;

    println!("{:?}", bin);
}
```
