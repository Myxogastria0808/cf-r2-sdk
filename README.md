# Cloudflare R2 Operation SDK

This is the Cloudflare R2 Operation SDK.

This crates can upload, download, and delete binary data or files to Cloudflare R2.

## Documentation

https://docs.rs/cf-r2-sdk/latest/cf_r2_sdk/

## How to use

### 1. Create a aws_sdk_s3::Client object

Set the "bucket name", "access key id", "secret access key", "endpoint url", and "region".

Default value of region is "auto" (region is option field).

```rust
// create a client object
let object = Builder::new()
    .set_bucket_name("bucket_name")
    .set_access_key_id("access_key_id")
    .set_secret_access_key("secret_access_key")
    .set_endpoint("endpoint_url")
    .set_region("region")
    .create_client();
```

### 2. Operate R2 object strage

#### upload binary data

```rust
let _ = object
    .upload_binary("<file name (key)> as &str", "<mime type> as &str", "<binary data> as &[u8]")
    .await;
```

#### upload file

```rust
let _ = object
    .upload_file("<file name (key)> as &str", "<mime type> as &str", "<file path> as &str")
    .await;
```

#### download binary data

```rust
let binany: Vec<u8> = object.download("<file name (key)> as &str").await;
```

#### delete file

```rust
let _ = object.delete("<file name (key)> as &str").await;
```

## Sample

Sample repository is

https://github.com/Myxogastria0808/cf-r2-sdk-sample .

```rust
use cf_r2_sdk::builder::Builder;
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

    let object: cf_r2_sdk::operator::Operator = Builder::new()
        .set_bucket_name(bucket_name)
        .set_access_key_id(access_key_id)
        .set_secret_access_key(secret_access_key)
        .set_endpoint(endpoint_url)
        .set_region(region)
        .create_client();

    let _ = object
        .upload_binary("text.txt", "text/plain", b"Hello, World!")
        .await;

    let bin: Result<Vec<u8>, cf_r2_sdk::error::OperationError> = object.download("text.txt").await;

    println!("{:?}", bin);
}
```
