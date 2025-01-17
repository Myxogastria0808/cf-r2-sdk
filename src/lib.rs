mod utils;

// use dotenvy::dotenv;
// use std::env;
// use utils::builder::Builder;

// #[tokio::main]
// async fn main() {
//     // load .env file
//     dotenv().expect(".env file not found.");
//     // insert a environment variable
//     let bucket_name = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
//     let endpoint_url: String =
//         env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
//     let access_key_id: String =
//         env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
//     let secret_access_key: String =
//         env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");
//     let region: String = env::var("REGION").expect("REGION not found in .env file.");

//     let object = Builder::new()
//         .set_bucket_name(bucket_name)
//         .set_access_key_id(access_key_id)
//         .set_secret_access_key(secret_access_key)
//         .set_endpoint(endpoint_url)
//         .set_region(region)
//         .create_client();

//     object
//         .upload_binary("text.txt", "text/plain", b"Hello, World!")
//         .await;

//     let bin = object.download("text.txt").await;
//     println!("{:?}", bin);

//     object.delete("text.txt").await;

//     object
//         .upload_file("tsukuba.webp", "image/webp", "./tsukuba.webp")
//         .await;

//     object
//         .upload_file("sample.txt", "text/plain", "./sample.txt")
//         .await;
// }
