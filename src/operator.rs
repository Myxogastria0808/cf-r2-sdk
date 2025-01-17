use aws_sdk_s3::primitives::ByteStream;
use tokio::{fs::File, io::AsyncReadExt};

/// Operator for uploading, downloading, and deleting files to a S3 bucket.
///
/// # Example
///
/// ```
/// use cf_r2_sdk::{builder::Builder, operator::Operator};
///
/// #[tokio::main]
/// async fn main() {
///     let object: cf_r2_sdk::operator::Operator = Builder::new()
///         .set_bucket_name("bucket_name")
///         .set_access_key_id("access_key_id")
///         .set_secret_access_key("secret_access")
///         .set_endpoint("endpoint_url")
///         .set_region("auto")
///         .create_client();
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
        //! use cf_r2_sdk::{builder::Builder, operator::Operator};
        //!
        //! #[tokio::main]
        //! async fn main() {
        //!     let object: cf_r2_sdk::operator::Operator = Builder::new()
        //!         .set_bucket_name("bucket_name")
        //!         .set_access_key_id("access_key_id")
        //!         .set_secret_access_key("secret_access")
        //!         .set_endpoint("endpoint_url")
        //!         .set_region("auto")
        //!         .create_client();
        //!
        //!    let _ = object
        //!        .upload_file("sample.jpg", "image/jpeg", "./data/sample.jpg")
        //!        .await;
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
        //! use cf_r2_sdk::{builder::Builder, operator::Operator};
        //!
        //! #[tokio::main]
        //! async fn main() {
        //!     let object: cf_r2_sdk::operator::Operator = Builder::new()
        //!         .set_bucket_name("bucket_name")
        //!         .set_access_key_id("access_key_id")
        //!         .set_secret_access_key("secret_access")
        //!         .set_endpoint("endpoint_url")
        //!         .set_region("auto")
        //!         .create_client();
        //!
        //!    let _ = object
        //!        .upload_binary("sample.txt", "test/plain", b"Hello, World!")
        //!        .await;
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

    pub async fn download(&self, file_name: &str) -> Result<Vec<u8>, crate::error::OperationError> {
        //! Download a file as binary data from the S3 bucket.
        //!
        //! # Example
        //!
        //! ```
        //! use cf_r2_sdk::{builder::Builder, operator::Operator};
        //!
        //! #[tokio::main]
        //! async fn main() {
        //!     let object: cf_r2_sdk::operator::Operator = Builder::new()
        //!         .set_bucket_name("bucket_name")
        //!         .set_access_key_id("access_key_id")
        //!         .set_secret_access_key("secret_access")
        //!         .set_endpoint("endpoint_url")
        //!         .set_region("auto")
        //!         .create_client();
        //!
        //!    let _ = object
        //!        .download("sample.txt")
        //!        .await;
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

    pub async fn delete(&self, file_name: &str) -> Result<(), crate::error::OperationError> {
        //! Delete a file from the S3 bucket.
        //!
        //! # Example
        //!
        //! ```
        //! use cf_r2_sdk::{builder::Builder, operator::Operator};
        //!
        //! #[tokio::main]
        //! async fn main() {
        //!     let object: cf_r2_sdk::operator::Operator = Builder::new()
        //!         .set_bucket_name("bucket_name")
        //!         .set_access_key_id("access_key_id")
        //!         .set_secret_access_key("secret_access")
        //!         .set_endpoint("endpoint_url")
        //!         .set_region("auto")
        //!         .create_client();
        //!
        //!    let _ = object
        //!        .delete("sample.txt")
        //!        .await;
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
}
