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
    async fn test_upload_and_download_binary() {
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

        let _ = object
            .upload_binary("test.txt", "text/plain", b"Hello, World!")
            .await;

        let bin = object.download("test.txt").await;
        match bin {
            Ok(bin) => assert_eq!(bin, b"Hello, World!"),
            Err(e) => panic!("Error: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_upload_and_download_file() {
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
        let _ = object
            .upload_file("sample.jpg", "image/jpeg", file_path)
            .await;

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
    }

    #[tokio::test]
    async fn test_upload_and_delete() {
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

        let _ = object
            .upload_binary("text.txt", "text/plain", b"Hello, World!")
            .await;

        let _ = object.delete("text.txt").await;

        let bin = object.download("text.txt").await;
        if bin.is_ok() {
            panic!("Error: {:?}", bin)
        }
    }
}
