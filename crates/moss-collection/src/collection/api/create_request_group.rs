use anyhow::Context;
use moss_common::api::{OperationError, OperationResult};
use moss_fs::utils::encode_path;
use moss_storage::collection_storage::entities::request_store_entities::{
    GroupEntity, RequestNodeEntity,
};
use validator::Validate;

use crate::collection::Collection;
use crate::collection_registry::{CollectionRequestGroupData, RequestNode};
use crate::constants::REQUESTS_DIR;
use crate::models::operations::{CreateRequestGroupInput, CreateRequestGroupOutput};

impl Collection {
    pub async fn create_request_group(
        &self,
        input: CreateRequestGroupInput,
    ) -> OperationResult<CreateRequestGroupOutput> {
        input.validate()?;

        let encoded_path = encode_path(&input.path, None)?;
        let request_group_abs_path = self.abs_path.join(REQUESTS_DIR).join(&encoded_path);

        if request_group_abs_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: request_group_abs_path
                    .file_name()
                    .expect("The path should never end with a root")
                    .to_string_lossy()
                    .to_string(),
                path: input.path,
            });
        }

        let request_store = self.state_db_manager.request_store().await;
        let request_nodes = self.registry().await?.requests_nodes();

        let (mut txn, table) = request_store.begin_write()?;

        table.insert(
            &mut txn,
            encoded_path.to_string_lossy().to_string().to_string(),
            &RequestNodeEntity::Group(GroupEntity { order: None }),
        )?;

        self.fs
            .create_dir(&request_group_abs_path)
            .await
            .context("Failed to create the request group directory")?;

        txn.commit()?;

        let request_group_key = {
            let mut lock = request_nodes.write().await;
            lock.insert(RequestNode::Group(CollectionRequestGroupData {
                name: input
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                path: encoded_path,
                order: None,
                spec_file_name: None,
            }))
        };

        Ok(CreateRequestGroupOutput {
            key: request_group_key,
        })
    }
}
