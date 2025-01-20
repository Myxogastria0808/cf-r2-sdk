# Unofficial Cloudflare R2 SDK

[![test](https://github.com/Myxogastria0808/cf-r2-sdk/actions/workflows/test.yaml/badge.svg)](https://github.com/Myxogastria0808/cf-r2-sdk/actions/workflows/test.yaml)
![GitHub License](https://img.shields.io/github/license/Myxogastria0808/cf-r2-sdk)
![GitHub Release](https://img.shields.io/github/v/release/Myxogastria0808/cf-r2-sdk)
![GitHub Tag](https://img.shields.io/github/v/tag/Myxogastria0808/cf-r2-sdk)

This is the "Unofficial Cloudflare R2 SDK".

It can upload, download, and delete binary data or files to Cloudflare R2.

This crate is based on [cloudflare-r2-rs](https://crates.io/crates/cloudflare-r2-rs) (License: [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/), Owner: [milen-denev](https://github.com/milen-denev)) and [r2sync](https://crates.io/crates/r2sync) (License: [MIT](https://github.com/Songmu/r2sync/blob/main/LICENSE), Owner: [Songmu](https://github.com/Songmu)).

> [!TIP]
> date: 2025-01-18
>
> This crates is solved [this problem](https://www.cloudflarestatus.com/incidents/t5nrjmpxc1cj) by adding the following S3Client config.
>
> ```
> requestChecksumCalculation: "WHEN_REQUIRED",
> responseChecksumValidation: "WHEN_REQUIRED",
> ```
>
> Reference: https://developers.cloudflare.com/r2/examples/aws/aws-sdk-js-v3/

## Documentation

https://docs.rs/cf-r2-sdk/latest/cf_r2_sdk/

## Article (Japanese)

https://qiita.com/Yuuki-Osada/items/10734e3d701a519b3d5f

## How to use

### 1. Create a aws_sdk_s3::Client object

Set the "bucket name", "access key id", "secret access key", "endpoint url", and "region".

Default value of region is "auto" (region is option field).

```rust
// create a client object
let object: cf_r2_sdk::operator::Operator = Builder::new()
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
    .await.unwrap();
```

#### upload file

```rust
let _ = object
    .upload_file("<file name (key)> as &str", "<mime type> as &str", "<file path> as &str")
    .await.unwrap();
```

#### download binary data

```rust
let binany: Vec<u8> = object.download("<file name (key)> as &str").await.unwrap();
```

#### delete file

```rust
let _ = object.delete("<file name (key)> as &str").await.unwrap();
```

#### get file names vector (max get file names is 10)

```rust
let file_names_list:Vec<String> = object.list_object().await.unwrap();
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
