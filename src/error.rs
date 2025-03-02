/// OperationError is an error type that represents the error occurred during the operation.
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

/// BuilderError is an error type that represents the error occurred during the builder process.
#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("BucketNameNotSetError: Bucket name is not set.")]
    BucketNameNotSetError,
    #[error("AccessKeyIdNotSetError: Access key id is not set.")]
    AccessKeyIdNotSetError,
    #[error("SecretAccessKey: Secret access key is not set.")]
    SecretAccessKeyNotSetError,
    #[error("EndpointNotSetError: Endpoint is not set.")]
    EndpointNotSetError,
}

/// Error is an error type that represents the error occurred during the operation or the builder process.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    OperationError(#[from] OperationError),
    #[error(transparent)]
    BuilderError(#[from] BuilderError),
}
