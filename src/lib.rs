//! # Unofficial Cloudflare R2 SDK
//!
//! This is the "Unofficial Cloudflare R2 SDK".
//!
//! It can upload, download, and delete binary data or files to Cloudflare R2.
//!
//! This crate is based on [cloudflare-r2-rs](https://crates.io/crates/cloudflare-r2-rs) (License: [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/), Owner: [milen-denev](https://github.com/milen-denev)) and [r2sync](https://crates.io/crates/r2sync) (License: [MIT](https://github.com/Songmu/r2sync/blob/main/LICENSE), Owner: [Songmu](https://github.com/Songmu)).
//!
//! <div class="warning">
//!
//! date: 2025-01-18
//!
//! This crate is <strong>solved</strong> [this problem](https://www.cloudflarestatus.com/incidents/t5nrjmpxc1cj) by adding the following S3Client config.
//!
//! requestChecksumCalculation: "WHEN_REQUIRED",
//! responseChecksumValidation: "WHEN_REQUIRED",
//!
//! Reference: [https://developers.cloudflare.com/r2/examples/aws/aws-sdk-js-v3/](https://developers.cloudflare.com/r2/examples/aws/aws-sdk-js-v3/)
//!
//! </div>
//!
//! # Struct
//!
//! [builder::Builder](https://docs.rs/cf-r2-sdk/latest/cf_r2_sdk/builder/struct.Builder.html)
//!
//! [operator::Operator](https://docs.rs/cf-r2-sdk/latest/cf_r2_sdk/operator/struct.Operator.html)
//!
//! # How to use
//!
//! ### 1. Create a aws_sdk_s3::Client object
//!
//! Set the "bucket name", "access key id", "secret access key", "endpoint url", and "region".
//!
//! Default value of region is "auto" (region is option field).
//!
//! ```
//! use cf_r2_sdk::builder::Builder;
//! use dotenvy::dotenv;
//! use std::env;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() {
//!     // load .env file
//!     dotenv().expect(".env file not found.");
//!     // insert a environment variable
//!     let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
//!     let endpoint_url: String =
//!         env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
//!     let access_key_id: String =
//!         env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
//!     let secret_access_key: String =
//!         env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");
//!     let region: String = env::var("REGION").expect("REGION not found in .env file.");
//!
//!     let object: cf_r2_sdk::operator::Operator = Builder::new()
//!         .set_bucket_name(bucket_name)
//!         .set_access_key_id(access_key_id)
//!         .set_secret_access_key(secret_access_key)
//!         .set_endpoint(endpoint_url)
//!         .set_region(region)
//!         .create_client();
//! }
//! ```
//!
//! ### 2. Operate R2 object strage
//!
//! #### Upload binary data
//!
//!
//! ```
//! use cf_r2_sdk::builder::Builder;
//! use dotenvy::dotenv;
//! use std::env;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), cf_r2_sdk::error::OperationError> {
//!    // load .env file
//!    dotenv().expect(".env file not found.");
//!    // insert a environment variable
//!    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
//!    let endpoint_url: String =
//!        env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
//!    let access_key_id: String =
//!        env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
//!    let secret_access_key: String =
//!       env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");
//!    let region: String = env::var("REGION").expect("REGION not found in .env file.");
//!
//!    let object: cf_r2_sdk::operator::Operator = Builder::new()
//!        .set_bucket_name(bucket_name)
//!        .set_access_key_id(access_key_id)
//!        .set_secret_access_key(secret_access_key)
//!        .set_endpoint(endpoint_url)
//!        .set_region(region)
//!        .create_client();
//!
//!    // upload binary data
//!    object
//!        .upload_binary("sample.txt", "test/plain", b"Hello, World!")
//!        .await?;
//!    Ok(())
//! }
//! ```
//!
//! #### upload file
//!
//! ```
//! use cf_r2_sdk::builder::Builder;
//! use dotenvy::dotenv;
//! use std::env;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), cf_r2_sdk::error::OperationError> {
//!    // load .env file
//!    dotenv().expect(".env file not found.");
//!    // insert a environment variable
//!    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
//!    let endpoint_url: String =
//!        env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
//!    let access_key_id: String =
//!        env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
//!    let secret_access_key: String =
//!       env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");
//!    let region: String = env::var("REGION").expect("REGION not found in .env file.");
//!
//!    let object: cf_r2_sdk::operator::Operator = Builder::new()
//!        .set_bucket_name(bucket_name)
//!        .set_access_key_id(access_key_id)
//!        .set_secret_access_key(secret_access_key)
//!        .set_endpoint(endpoint_url)
//!        .set_region(region)
//!        .create_client();
//!
//!    // upload file
//!    object
//!        .upload_file("sample.jpg", "image/jpeg", "data/sample.jpg")
//!        .await?;
//!    Ok(())
//! }
//! ```
//!
//! #### download binary data
//!
//! ```
//! use cf_r2_sdk::builder::Builder;
//! use dotenvy::dotenv;
//! use std::env;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), cf_r2_sdk::error::OperationError> {
//!    // load .env file
//!    dotenv().expect(".env file not found.");
//!    // insert a environment variable
//!    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
//!    let endpoint_url: String =
//!        env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
//!    let access_key_id: String =
//!        env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
//!    let secret_access_key: String =
//!       env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");
//!    let region: String = env::var("REGION").expect("REGION not found in .env file.");
//!
//!    let object: cf_r2_sdk::operator::Operator = Builder::new()
//!        .set_bucket_name(bucket_name)
//!        .set_access_key_id(access_key_id)
//!        .set_secret_access_key(secret_access_key)
//!        .set_endpoint(endpoint_url)
//!        .set_region(region)
//!        .create_client();
//!
//!     object
//!        .upload_binary("sample.txt", "test/plain", b"Hello, World!")
//!        .await;
//!
//!    // download binary data
//!    let bin: Vec<u8> = object.download("sample.txt").await?;
//!
//!    println!("{:?}", bin);
//!    Ok(())
//! }
//! ```
//!
//! #### delete file
//!
//! ```
//! use cf_r2_sdk::builder::Builder;
//! use dotenvy::dotenv;
//! use std::env;
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), cf_r2_sdk::error::OperationError> {
//!    // load .env file
//!    dotenv().expect(".env file not found.");
//!    // insert a environment variable
//!    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
//!    let endpoint_url: String =
//!        env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
//!    let access_key_id: String =
//!        env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
//!    let secret_access_key: String =
//!       env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");
//!    let region: String = env::var("REGION").expect("REGION not found in .env file.");
//!
//!    let object: cf_r2_sdk::operator::Operator = Builder::new()
//!        .set_bucket_name(bucket_name)
//!        .set_access_key_id(access_key_id)
//!        .set_secret_access_key(secret_access_key)
//!        .set_endpoint(endpoint_url)
//!        .set_region(region)
//!        .create_client();
//!
//!    object
//!        .upload_binary("sample.txt", "test/plain", b"Hello, World!")
//!        .await;
//!
//!    // delete file
//!    object
//!        .delete("sample.txt")
//!        .await?;
//!
//!    Ok(())
//! }
//! ```

pub mod builder;
pub mod error;
pub mod operator;

#[cfg(test)]
mod tests {
    use super::*;
    use builder::Builder;
    use dotenvy::dotenv;
    use std::env;
    use tokio::{fs::File, io::AsyncReadExt};

    #[tokio::test]
    async fn local_test_upload_and_download_binary() -> Result<(), error::OperationError> {
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

        let object = Builder::new()
            .set_bucket_name(bucket_name)
            .set_access_key_id(access_key_id)
            .set_secret_access_key(secret_access_key)
            .set_endpoint(endpoint_url)
            .set_region(region)
            .create_client();

        object
            .upload_binary("test.txt", "text/plain", b"Hello, World!")
            .await?;

        let bin = object.download("test.txt").await;
        match bin {
            Ok(bin) => assert_eq!(bin, b"Hello, World!"),
            Err(e) => panic!("Error: {:?}", e),
        }
        Ok(())
    }

    #[tokio::test]
    async fn local_test_upload_and_download_file() -> Result<(), error::OperationError> {
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

        let object = Builder::new()
            .set_bucket_name(bucket_name)
            .set_access_key_id(access_key_id)
            .set_secret_access_key(secret_access_key)
            .set_endpoint(endpoint_url)
            .set_region(region)
            .create_client();

        let file_path = "./data/sample.jpg";
        object
            .upload_file("sample.jpg", "image/jpeg", file_path)
            .await?;

        let bin = object.download("sample.jpg").await;

        //read file from local
        let mut file = File::open(file_path).await.expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .expect("Failed to close file");

        match bin {
            Ok(bin) => assert_eq!(bin, buffer),
            Err(e) => panic!("Error: {:?}", e),
        }
        Ok(())
    }

    #[tokio::test]
    async fn local_test_upload_and_delete() -> Result<(), error::OperationError> {
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

        let object = Builder::new()
            .set_bucket_name(bucket_name)
            .set_access_key_id(access_key_id)
            .set_secret_access_key(secret_access_key)
            .set_endpoint(endpoint_url)
            .set_region(region)
            .create_client();

        object
            .upload_binary("text.txt", "text/plain", b"Hello, World!")
            .await?;

        object.delete("text.txt").await?;

        let bin: Result<Vec<u8>, error::OperationError> = object.download("text.txt").await;
        if bin.is_ok() {
            panic!("Error: {:?}", bin)
        }
        Ok(())
    }

    //* Actions Test *//
    #[tokio::test]
    async fn actions_test_upload_and_download_binary() -> Result<(), error::OperationError> {
        // dotenv().expect(".env file not found.");
        // insert a environment variable
        let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
        let endpoint_url: String =
            env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
        let access_key_id: String =
            env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
        let secret_access_key: String =
            env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");
        let region: String = env::var("REGION").expect("REGION not found in .env file.");

        let object = Builder::new()
            .set_bucket_name(bucket_name)
            .set_access_key_id(access_key_id)
            .set_secret_access_key(secret_access_key)
            .set_endpoint(endpoint_url)
            .set_region(region)
            .create_client();

        object
            .upload_binary("test.txt", "text/plain", b"Hello, World!")
            .await?;

        let bin = object.download("test.txt").await;
        match bin {
            Ok(bin) => assert_eq!(bin, b"Hello, World!"),
            Err(e) => panic!("Error: {:?}", e),
        }
        Ok(())
    }

    #[tokio::test]
    async fn actions_test_upload_and_download_file() -> Result<(), error::OperationError> {
        // dotenv().expect(".env file not found.");
        // insert a environment variable
        let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
        let endpoint_url: String =
            env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
        let access_key_id: String =
            env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
        let secret_access_key: String =
            env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");
        let region: String = env::var("REGION").expect("REGION not found in .env file.");

        let object = Builder::new()
            .set_bucket_name(bucket_name)
            .set_access_key_id(access_key_id)
            .set_secret_access_key(secret_access_key)
            .set_endpoint(endpoint_url)
            .set_region(region)
            .create_client();

        let file_path = "./data/sample.jpg";
        object
            .upload_file("sample.jpg", "image/jpeg", file_path)
            .await?;

        let bin = object.download("sample.jpg").await;

        //read file from local
        let mut file = File::open(file_path).await.expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .expect("Failed to close file");

        match bin {
            Ok(bin) => assert_eq!(bin, buffer),
            Err(e) => panic!("Error: {:?}", e),
        }
        Ok(())
    }

    #[tokio::test]
    async fn actions_test_upload_and_delete() -> Result<(), error::OperationError> {
        // dotenv().expect(".env file not found.");
        // insert a environment variable
        let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
        let endpoint_url: String =
            env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
        let access_key_id: String =
            env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
        let secret_access_key: String =
            env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");
        let region: String = env::var("REGION").expect("REGION not found in .env file.");

        let object = Builder::new()
            .set_bucket_name(bucket_name)
            .set_access_key_id(access_key_id)
            .set_secret_access_key(secret_access_key)
            .set_endpoint(endpoint_url)
            .set_region(region)
            .create_client();

        object
            .upload_binary("text.txt", "text/plain", b"Hello, World!")
            .await?;

        object.delete("text.txt").await?;

        let bin: Result<Vec<u8>, error::OperationError> = object.download("text.txt").await;
        if bin.is_ok() {
            panic!("Error: {:?}", bin)
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_list_objects() -> Result<(), error::OperationError> {
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

        let object = Builder::new()
            .set_bucket_name(bucket_name)
            .set_access_key_id(access_key_id)
            .set_secret_access_key(secret_access_key)
            .set_endpoint(endpoint_url)
            .set_region(region)
            .create_client();

        object
            .list_objects()
            .await?;

        Ok(())
    }

}
