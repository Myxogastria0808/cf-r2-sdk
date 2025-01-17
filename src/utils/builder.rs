use aws_sdk_s3::config::{
    Credentials, Region, RequestChecksumCalculation, ResponseChecksumValidation,
};

use crate::utils::operator::Operator;

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
        Self::default()
    }

    pub fn set_bucket_name(mut self, bucket_name: String) -> Self {
        self.bucket_name = bucket_name;
        self
    }

    pub fn set_access_key_id(mut self, access_key_id: String) -> Self {
        self.access_key_id = access_key_id;
        self
    }

    pub fn set_secret_access_key(mut self, secret_access_key: String) -> Self {
        self.secret_access_key = secret_access_key;
        self
    }

    pub fn set_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = endpoint;
        self
    }

    pub fn set_region(mut self, region: String) -> Self {
        self.region = region;
        self
    }

    pub fn create_client(&self) -> Operator {
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
