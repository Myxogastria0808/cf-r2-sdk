use crate::{error::BuilderError, operator::Operator};
use aws_sdk_s3::config::{
    Credentials, Region, RequestChecksumCalculation, ResponseChecksumValidation,
};

/// Builder for creating a new [Operator] instance.
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
///    Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Builder {
    bucket_name: Option<String>,
    access_key_id: Option<String>,
    secret_access_key: Option<String>,
    endpoint: Option<String>,
    region: String,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            bucket_name: None,
            access_key_id: None,
            secret_access_key: None,
            endpoint: None,
            region: "auto".to_string(),
        }
    }
}

impl Builder {
    pub fn new() -> Self {
        //! Create a new [Builder] instance with default values.
        //!
        //! default value of region is "auto".
        Self::default()
    }

    pub fn set_bucket_name(mut self, bucket_name: String) -> Self {
        //! Set the bucket name.
        self.bucket_name = Some(bucket_name);
        self
    }

    pub fn set_access_key_id(mut self, access_key_id: String) -> Self {
        //! Set the access key id.
        self.access_key_id = Some(access_key_id);
        self
    }

    pub fn set_secret_access_key(mut self, secret_access_key: String) -> Self {
        //! Set the secret access key.
        self.secret_access_key = Some(secret_access_key);
        self
    }

    pub fn set_endpoint(mut self, endpoint: String) -> Self {
        //! Set the endpoint.
        self.endpoint = Some(endpoint);
        self
    }

    pub fn set_region(mut self, region: String) -> Self {
        //! Set the region.
        self.region = region;
        self
    }

    #[deprecated(since = "3.1.0", note = "use create_client_result() instead")]
    pub fn create_client(&self) -> Operator {
        //! Create a new [Operator] instance.
        let credentials = Credentials::new(
            self.access_key_id
                .as_ref()
                .expect("Access key id is not set."),
            self.secret_access_key
                .as_ref()
                .expect("Secret access key is not set."),
            None,
            None,
            "",
        );

        let config = aws_sdk_s3::config::Builder::new()
            .credentials_provider(credentials)
            .region(Region::new(self.region.clone()))
            .endpoint_url(self.endpoint.as_ref().expect("Endpoint is not set."))
            .clone()
            .build();

        Operator::new(
            self.bucket_name.clone().expect("Bucket name is not set."),
            aws_sdk_s3::Client::from_conf(config),
        )
    }

    pub fn create_client_result(&self) -> Result<Operator, BuilderError> {
        //! Create a new [Operator] instance.
        let bucket_name = match &self.bucket_name {
            Some(bucket_name) => bucket_name.clone(),
            None => Err(BuilderError::BucketNameNotSetError)?,
        };
        let access_key_id = match &self.access_key_id {
            Some(access_key_id) => access_key_id,
            None => Err(BuilderError::AccessKeyIdNotSetError)?,
        };
        let secret_access_key = match &self.secret_access_key {
            Some(secret_access_key) => secret_access_key,
            None => Err(BuilderError::SecretAccessKeyNotSetError)?,
        };
        let endpoint = match &self.endpoint {
            Some(endpoint) => endpoint,
            None => Err(BuilderError::EndpointNotSetError)?,
        };

        let credentials = Credentials::new(access_key_id, secret_access_key, None, None, "");

        let config = aws_sdk_s3::config::Builder::new()
            .credentials_provider(credentials)
            .region(Region::new(self.region.clone()))
            .endpoint_url(endpoint)
            .set_request_checksum_calculation(Some(RequestChecksumCalculation::WhenRequired))
            .set_response_checksum_validation(Some(ResponseChecksumValidation::WhenRequired))
            .clone()
            .build();

        Ok(Operator::new(
            bucket_name,
            aws_sdk_s3::Client::from_conf(config),
        ))
    }
}
