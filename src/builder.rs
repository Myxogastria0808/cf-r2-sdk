use crate::operator::Operator;
use aws_sdk_s3::config::{
    Credentials, Region, RequestChecksumCalculation, ResponseChecksumValidation,
};

/// Builder for creating a new [Operator] instance.
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
/// }
/// ```
#[derive(Debug)]
pub struct Builder {
    bucket_name: String,
    access_key_id: String,
    secret_access_key: String,
    endpoint: String,
    region: String,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            bucket_name: String::new(),
            access_key_id: String::new(),
            secret_access_key: String::new(),
            endpoint: String::new(),
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
        self.bucket_name = bucket_name;
        self
    }

    pub fn set_access_key_id(mut self, access_key_id: String) -> Self {
        //! Set the access key id.
        self.access_key_id = access_key_id;
        self
    }

    pub fn set_secret_access_key(mut self, secret_access_key: String) -> Self {
        //! Set the secret access key.
        self.secret_access_key = secret_access_key;
        self
    }

    pub fn set_endpoint(mut self, endpoint: String) -> Self {
        //! Set the endpoint.
        self.endpoint = endpoint;
        self
    }

    pub fn set_region(mut self, region: String) -> Self {
        //! Set the region.
        self.region = region;
        self
    }

    pub fn create_client(&self) -> Operator {
        //! Create a new [Operator] instance.
        let credentials =
            Credentials::new(&self.access_key_id, &self.secret_access_key, None, None, "");

        let config = aws_sdk_s3::config::Builder::new()
            .credentials_provider(credentials)
            .region(Region::new(self.region.clone()))
            .endpoint_url(&self.endpoint)
            .set_request_checksum_calculation(Some(RequestChecksumCalculation::WhenRequired))
            .set_response_checksum_validation(Some(ResponseChecksumValidation::WhenRequired))
            .clone()
            .build();

        Operator {
            bucket_name: self.bucket_name.clone(),
            client: aws_sdk_s3::Client::from_conf(config),
        }
    }
}
