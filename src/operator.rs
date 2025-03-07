use aws_sdk_s3::primitives::ByteStream;
use tokio::{fs::File, io::AsyncReadExt};

/// Operator for uploading, downloading, and deleting files to a R2 bucket.
///
/// # Example
///
/// ```
/// use cf_r2_sdk::builder::Builder;
/// use cf_r2_sdk::error::Error;
/// use dotenvy::dotenv;
/// use std::env;
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), Error> {
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
///        .create_client_result()?;
///
///    let _ = object
///        .upload_binary("sample.txt", "test/plain", b"Hello, World!", None)
///        .await?;
///    Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Operator {
    bucket_name: String,
    client: aws_sdk_s3::Client,
}

impl Operator {
    pub fn new(bucket_name: String, client: aws_sdk_s3::Client) -> Self {
        //! Create a new [Operator] instance.
        Self {
            bucket_name,
            client,
        }
    }

    pub async fn upload_file(
        &self,
        file_name: &str,
        mime_type: &str,
        file_path: &str,
        cache_control: Option<&str>,
    ) -> Result<(), crate::error::OperationError> {
        //! Upload a file to the R2 bucket.
        //!
        //! # Example
        //!
        //! ```
        //! use cf_r2_sdk::builder::Builder;
        //! use cf_r2_sdk::error::Error;
        //! use dotenvy::dotenv;
        //! use std::env;
        //!
        //! #[tokio::main(flavor = "current_thread")]
        //! async fn main() -> Result<(), Error> {
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
        //!        .create_client_result()?;
        //!
        //!    // upload file
        //!    object
        //!        .upload_file("sample.jpg", "image/jpeg", "./data/sample.jpg", None)
        //!        .await?;
        //!   Ok(())
        //! }
        //! ```
        let mut file = File::open(file_path).await?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await?;

        match &self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .content_type(mime_type)
            .cache_control(cache_control.unwrap_or("no-cache"))
            .body(ByteStream::from(buffer))
            .send()
            .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(crate::error::OperationError::AWSSdkS3PutObjectError(
                    err.to_string(),
                ))
            }
        };
        Ok(())
    }

    pub async fn upload_binary(
        &self,
        file_name: &str,
        mime_type: &str,
        binary: &[u8],
        cache_control: Option<&str>,
    ) -> Result<(), crate::error::OperationError> {
        //! Upload binary data to the R2 bucket.
        //!
        //! # Example
        //!
        //! ```
        //! use cf_r2_sdk::builder::Builder;
        //! use cf_r2_sdk::error::Error;
        //! use dotenvy::dotenv;
        //! use std::env;
        //!
        //! #[tokio::main(flavor = "current_thread")]
        //! async fn main() -> Result<(), Error> {
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
        //!        .create_client_result()?;
        //!
        //!    // upload binary data
        //!    object
        //!        .upload_binary("sample.txt", "test/plain", b"Hello, World!", None)
        //!        .await?;
        //!
        //!    Ok(())
        //! }
        //! ```
        match &self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .content_type(mime_type)
            .cache_control(cache_control.unwrap_or("no-cache"))
            .body(ByteStream::from(binary.to_vec()))
            .send()
            .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(crate::error::OperationError::AWSSdkS3PutObjectError(
                    err.to_string(),
                ))
            }
        };
        Ok(())
    }

    pub async fn download(&self, file_name: &str) -> Result<Vec<u8>, crate::error::OperationError> {
        //! Download a file as binary data from the R2 bucket.
        //!
        //! # Example
        //!
        //! ```
        //! use cf_r2_sdk::builder::Builder;
        //! use cf_r2_sdk::error::Error;
        //! use dotenvy::dotenv;
        //! use std::env;
        //!
        //! #[tokio::main(flavor = "current_thread")]
        //! async fn main() -> Result<(), Error> {
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
        //!        .create_client_result()?;
        //!
        //!    object
        //!        .upload_binary("sample.txt", "test/plain", b"Hello, World!", None)
        //!        .await?;
        //!
        //!    // download binary data
        //!    object
        //!        .download("sample.txt")
        //!        .await?;
        //!   Ok(())
        //! }
        //! ```
        let object = match self
            .client
            .clone()
            .get_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .send()
            .await
        {
            Ok(object) => object,
            Err(err) => {
                return Err(crate::error::OperationError::AWSSdkS3GetObjectError(
                    err.to_string(),
                ))
            }
        };
        let result = match object.body.collect().await {
            Ok(result) => result.into_bytes().to_vec(),
            Err(err) => return Err(crate::error::OperationError::AWSSdkS3ByteStreamError(err)),
        };
        Ok(result)
    }

    pub async fn delete(&self, file_name: &str) -> Result<(), crate::error::OperationError> {
        //! Delete a file from the R2 bucket.
        //!
        //! # Example
        //!
        //! ```
        //! use cf_r2_sdk::builder::Builder;
        //! use cf_r2_sdk::error::Error;
        //! use dotenvy::dotenv;
        //! use std::env;
        //!
        //! #[tokio::main(flavor = "current_thread")]
        //! async fn main() -> Result<(), Error> {
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
        //!        .create_client_result()?;
        //!
        //!    object
        //!        .upload_binary("sample.txt", "test/plain", b"Hello, World!", None)
        //!        .await?;
        //!
        //!    // delete file
        //!    let bin: Vec<u8> = object.download("sample.txt").await?;
        //!
        //!    println!("{:?}", bin);
        //!    Ok(())
        //! }
        //! ```
        match &self
            .client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .send()
            .await
        {
            Ok(_) => (),
            Err(err) => {
                return Err(crate::error::OperationError::AWSSdkS3DeleteObjectError(
                    err.to_string(),
                ))
            }
        }
        Ok(())
    }

    pub async fn list_objects(&self) -> Result<Vec<String>, crate::error::OperationError> {
        //! Get file names vector from the R2 bucket.
        //!
        //! # Example
        //!
        //! ```
        //! use cf_r2_sdk::builder::Builder;
        //! use cf_r2_sdk::error::Error;
        //! use dotenvy::dotenv;
        //! use std::env;
        //!
        //! #[tokio::main(flavor = "current_thread")]
        //! async fn main() -> Result<(), Error> {
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
        //!        .create_client_result()?;
        //!
        //!    object
        //!       .upload_binary("sample.txt", "test/plain", b"Hello, World!", None)
        //!       .await?;
        //!
        //!    // get file names vector
        //!    let file_names: Vec<String> = object.list_objects().await?;
        //!
        //!    for file_name in file_names {
        //!       println!("{}", file_name);
        //!    }
        //!
        //!    Ok(())
        //! }
        //! ```
        let response = &mut self
            .client
            .list_objects_v2()
            .bucket(&self.bucket_name)
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
                    return Err(crate::error::OperationError::AWSSdkS3ListObjectsV2Error(
                        err.to_string(),
                    ))
                }
            }
        }
        Ok(objects)
    }
}
