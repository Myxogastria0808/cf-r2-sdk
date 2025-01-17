use aws_sdk_s3::primitives::ByteStream;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug)]
pub struct Operator {
    pub bucket_name: String,
    pub client: aws_sdk_s3::Client,
}

impl Operator {
    pub async fn upload_file(&self, file_name: &str, mime_type: &str, file_path: &str) {
        let mut file = File::open(file_path).await.expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .await
            .expect("Failed to close file");

        let _ = &self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .content_type(mime_type)
            .body(ByteStream::from(buffer))
            .send()
            .await
            .unwrap();
    }

    pub async fn upload_binary(&self, file_name: &str, mime_type: &str, binary: &[u8]) {
        let _ = &self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .content_type(mime_type)
            .body(ByteStream::from(binary.to_vec()))
            .send()
            .await
            .unwrap();
    }

    pub async fn download(&self, file_name: &str) -> Vec<u8> {
        let object = self
            .client
            .clone()
            .get_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .send()
            .await
            .unwrap();
        object.body.collect().await.unwrap().into_bytes().to_vec()
    }

    pub async fn delete(&self, file_name: &str) {
        let _ = &self
            .client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(file_name)
            .send()
            .await
            .unwrap();
    }
}
