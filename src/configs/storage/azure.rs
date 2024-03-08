use std::{fs::File, io::Read};

use azure_storage::StorageCredentials;
use azure_storage_blobs::{
    blob::{BlobBlockType, BlockList},
    prelude::{BlockId, ClientBuilder, ContainerClient},
};
use log::{debug, error, info, trace};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

use crate::{extension::string_or_env, traits::Storage};

#[derive(Debug, Serialize, Deserialize)]
pub struct Azure {
    path: Option<String>,
    #[serde(deserialize_with = "string_or_env")]
    account: String,
    #[serde(deserialize_with = "string_or_env")]
    access_key: String,
    #[serde(deserialize_with = "string_or_env")]
    container: String,
    chunk_size: Option<usize>,
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

        let mut file = File::open(local_temp_file).unwrap();
        let mut total_bytes_uploaded: usize = 0;
        let mut blocks = BlockList::default();

        let chunk_size = self.chunk_size.unwrap_or(16);

        loop {
            let mut buffer = vec![0; chunk_size * 1024 * 1024];
            match file.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    buffer.truncate(n);
                    let block_id = BlockId::new(format!("{total_bytes_uploaded:016x}"));
                    trace!("Azure upload block id: {:?} {}", block_id, n);
                    blocks
                        .blocks
                        .push(BlobBlockType::Uncommitted(block_id.clone()));
                    match blob_client.put_block(block_id, buffer).await {
                        Ok(response) => {
                            trace!("Azure upload block response: {:?}", response);
                            total_bytes_uploaded += n;
                        }
                        Err(e) => {
                            error!("Error uploading block: {:?}", e);
                            return Err(());
                        }
                    }
                }
                Err(e) => {
                    error!("Error uploading block: {:?}", e);
                    return Err(());
                }
            }
        }

        let result = blob_client.put_block_list(blocks).await;

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
