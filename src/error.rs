/// Error type of cf-r2-sdk.
#[derive(Debug, thiserror::Error)]
pub enum OperationError {
    #[error(transparent)]
    FileOpenError(#[from] std::io::Error),
    #[error("{0}")]
    AWSSdkS3PutObjectError(String),
    #[error("{0}")]
    AWSSdkS3GetObjectError(String),
    #[error("{0}")]
    AWSSdkS3DeleteObjectError(String),
    #[error("{0}")]
    AWSSdkS3ListObjectsV2Error(String),
    #[error(transparent)]
    AWSSdkS3ByteStreamError(#[from] aws_sdk_s3::primitives::ByteStreamError),
}
