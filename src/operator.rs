use aws_sdk_s3::primitives::ByteStream;
use tokio::{fs::File, io::AsyncReadExt};

/// Operator for uploading, downloading, and deleting files to a S3 bucket.
///
/// # Example
///
/// ```
/// use cf_r2_sdk::builder::Builder;
/// use dotenvy::dotenv;
/// use std::env;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() {
///    // load .env file
///    dotenv().expect(".env file not found.");
///    // insert a environment variable
///    let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
///    let endpoint_url: String =
///        env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
///    let access_key_id: String =
///        env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
///    let secret_access_key: String =
///       env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");
///    let region: String = env::var("REGION").expect("REGION not found in .env file.");
///
///    let object: cf_r2_sdk::operator::Operator = Builder::new()
///        .set_bucket_name(bucket_name)
///        .set_access_key_id(access_key_id)
///        .set_secret_access_key(secret_access_key)
///        .set_endpoint(endpoint_url)
///        .set_region(region)
///        .create_client();
///
///    let _ = object
///        .upload_binary("sample.txt", "test/plain", b"Hello, World!")
///        .await;
/// }
/// ```
#[derive(Debug)]
pub struct Operator {
    pub bucket_name: String,
    pub client: aws_sdk_s3::Client,
}

impl Operator {
    pub async fn upload_file(
        &self,
        file_name: &str,
        mime_type: &str,
        file_path: &str,
    ) -> Result<(), crate::error::OperationError> {
        //! Upload a file to the S3 bucket.
        //!
        //! # Example
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
        //!        .upload_file("sample.jpg", "image/jpeg", "./data/sample.jpg")
        //!        .await?;
        //!   Ok(())
        //! }
        //! ```
        let mut file = File::open(file_path).await.expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        let _ = &self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .content_type(mime_type)
            .body(ByteStream::from(buffer))
            .send()
            .await?;
        Ok(())
    }

    pub async fn upload_binary(
        &self,
        file_name: &str,
        mime_type: &str,
        binary: &[u8],
    ) -> Result<(), crate::error::OperationError> {
        //! Upload binary data to the S3 bucket.
        //!
        //! # Example
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
        let _ = &self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .content_type(mime_type)
            .body(ByteStream::from(binary.to_vec()))
            .send()
            .await?;
        Ok(())
    }

    pub async fn download(
        &self, file_name: &str
    ) -> Result<Vec<u8>, crate::error::OperationError> {
        //! Download a file as binary data from the S3 bucket.
        //!
        //! # Example
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
        //!        .await?;
        //!
        //!    // download binary data
        //!    object
        //!        .download("sample.txt")
        //!        .await?;
        //!   Ok(())
        //! }
        //! ```
        let object = self
            .client
            .clone()
            .get_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .send()
            .await?;
        Ok(object.body.collect().await.unwrap().into_bytes().to_vec())
    }

    pub async fn delete(
        &self, file_name: &str
    ) -> Result<(), crate::error::OperationError> {
        //! Delete a file from the S3 bucket.
        //!
        //! # Example
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
        //!        .await?;
        //!
        //!    // delete file
        //!    let bin: Vec<u8> = object.download("sample.txt").await?;
        //!
        //!    println!("{:?}", bin);
        //!    Ok(())
        //! }
        //! ```
        let _ = &self
            .client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .send()
            .await?;
        Ok(())
    }

    pub async fn list_objects(
        &self
    ) -> Result<Vec<String>, crate::error::OperationError> {
        let response = &mut self
            .client
            .list_objects_v2()
            .bucket(self.bucket_name.to_owned())
            .max_keys(10) 
            .into_paginator()
            .send();
        let mut objects = Vec::new();
        while let Some(result) = response.next().await {
            match result {
                Ok(output) => {
                    for object in output.contents() {
                        objects.push(object.key().unwrap_or("Unknown").to_owned());
                    }
                }
                Err(err) => {
                    eprintln!("{err:?}")
                }
            }
        }    
        Ok(objects)
    }
}
