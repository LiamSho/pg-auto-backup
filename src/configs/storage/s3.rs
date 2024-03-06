use std::path::Path;

use aws_sdk_s3::{
    config::{BehaviorVersion, Credentials, Region},
    primitives::ByteStream,
};
use log::{error, info};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::traits::Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct S3 {
    pub access_key: String,
    pub access_secret: String,
    pub endpoint: String,
    pub region: String,
    pub bucket: String,
    pub enforce_path_style: bool,
}

static S3_CLIENT: OnceCell<aws_sdk_s3::Client> = OnceCell::new();

impl Storage for S3 {
    async fn preflight_check(&self) {
        let client = get_aws_s3_client(&self);
        S3_CLIENT
            .set(client)
            .expect("Failed to set S3 client in preflight check");

        let client = S3_CLIENT.get().unwrap();
        let result = client.list_buckets().send().await;

        match result {
            Ok(val) => {
                let bucket = val.buckets.unwrap();
                let bucket_exist = bucket
                    .iter()
                    .any(|b| b.name.is_some() && b.name.as_ref().unwrap() == &self.bucket);
                if bucket_exist {
                    info!("Preflight check success: S3 bucket {} exists", &self.bucket);
                } else {
                    panic!(
                        "Preflight check failed: S3 bucket {} does not exist",
                        &self.bucket
                    );
                }
            }
            Err(e) => panic!("Error during S3 preflight check: {:?}", e),
        }
    }

    async fn save_file(&self, local_temp_file: &str, save_path: &str, file_name: String) {
        let key = format!("{}/{}", save_path, file_name);

        let client = S3_CLIENT.get().unwrap();
        let body = ByteStream::from_path(Path::new(local_temp_file)).await;
        let result = client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(body.unwrap())
            .send()
            .await;

        match result {
            Ok(val) => info!(
                "File uploaded successfully, SHA256: {:?}",
                val.checksum_sha256
            ),
            Err(e) => error!("Error uploading file: {:?}", e),
        };
    }
}

fn get_aws_s3_client(s3: &S3) -> aws_sdk_s3::Client {
    let credential = Credentials::new(
        &s3.access_key,
        &s3.access_secret,
        None,
        None,
        "pg-auto-backup-config",
    );

    let config = aws_sdk_s3::config::Builder::new()
        .endpoint_url(&s3.endpoint)
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new(s3.region.clone()))
        .credentials_provider(credential)
        .force_path_style(s3.enforce_path_style)
        .build();

    aws_sdk_s3::Client::from_conf(config)
}
