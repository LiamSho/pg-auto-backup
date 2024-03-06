use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::{ClientBuilder, ContainerClient};
use log::{debug, error, info};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::traits::Storage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Azure {
    path: Option<String>,
    account: String,
    access_key: String,
    container: String,
}

static AZ_CLIENT: OnceCell<ContainerClient> = OnceCell::new();

impl Storage for Azure {
    async fn preflight_check(&self) {
        let client = get_azure_blob_container_client(&self);
        AZ_CLIENT
            .set(client)
            .expect("Failed to set Azure Blob Container client in preflight check");

        let client = AZ_CLIENT.get().unwrap();
        let result = client.exists().await;
        match result {
            Ok(val) => match val {
                true => info!("Preflight check: Azure Blob Storage container exists"),
                false => {
                    panic!("Preflight check: Azure Blob Storage container does not exist");
                }
            },
            Err(e) => {
                panic!("Error during Azure Blob Storage preflight check: {}", e);
            }
        }
    }

    async fn save_file(
        &self,
        local_temp_file: &str,
        save_path: &str,
        file_name: String,
    ) -> Result<String, ()> {
        let key = match &self.path {
            Some(val) => format!("{}/{}/{}", val, save_path, file_name),
            None => format!("{}/{}", save_path, file_name),
        };

        let client = AZ_CLIENT.get().unwrap();
        let blob_client = client.blob_client(key);

        let body = std::fs::read(local_temp_file).unwrap();
        let result = blob_client.put_block_blob(body).await;

        match result {
            Ok(val) => {
                let etag = val.etag;
                debug!("File uploaded to Azure Blob successfully, etag: {}", &etag);
                Ok(format!("Azure with etag: {}", etag))
            }
            Err(e) => {
                error!("Error uploading file: {:?}", e);
                Err(())
            }
        }
    }
}

fn get_azure_blob_container_client(azure: &Azure) -> ContainerClient {
    let credential = StorageCredentials::access_key(&azure.account, azure.access_key.clone());

    ClientBuilder::new(&azure.account, credential).container_client(&azure.container)
}
