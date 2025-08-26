use cf_r2_sdk::builder::Builder;
use cf_r2_sdk::error::Error;
use dotenvy::dotenv;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
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
        .create_client_result()?;

    // upload binary data
    object
        .upload_binary("simple.txt", "test/plain", b"Hello, World!", None)
        .await?;
    Ok(())
}
