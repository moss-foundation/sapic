use anyhow::Context as _;
use moss_common::api::{OperationError, OperationResult};
use moss_fs::utils::encode_name;
use moss_fs::RenameOptions;
use moss_storage::collection_storage::entities::request_store_entities::{
    RequestEntity, RequestNodeEntity,
};
use validator::Validate;

use crate::collection::{Collection, REQUESTS_DIR};
use crate::models::operations::RenameRequestInput;

impl Collection {
    pub async fn rename_request(&self, input: RenameRequestInput) -> OperationResult<()> {
        input
            .validate()
            .map_err(|error| OperationError::Validation(error.to_string()))?;

        let request_nodes = self.registry().await?.requests_nodes();
        let mut requests_lock = request_nodes.write().await;

        let mut lease_request_data = requests_lock.lease(input.key)?;

        if !lease_request_data.is_request() {
            return Err(OperationError::Validation(format!(
                "Resource {} is not a request",
                input.key
            )));
        }

        if lease_request_data.name() == input.new_name {
            return Ok(());
        }

        let request_dir_relative_path_old = lease_request_data.path().to_owned();
        let request_dir_path_old = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&request_dir_relative_path_old);
        if !request_dir_path_old.exists() {
            return Err(OperationError::NotFound {
                name: lease_request_data.name().to_string(),
                path: request_dir_path_old,
            });
        }

        let request_dir_relative_path_new = lease_request_data
            .path()
            .parent()
            .context("Failed to get the parent directory")?
            .join(format!("{}.request", encode_name(&input.new_name)));

        let request_dir_path_new = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&request_dir_relative_path_new);
        if request_dir_path_new.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.new_name,
                path: request_dir_path_new,
            });
        }

        self.fs
            .rename(
                &request_dir_path_old,
                &request_dir_path_new,
                RenameOptions::default(),
            )
            .await
            .context("Failed to rename the request directory")?;

        let request_store = self.collection_storage.request_store().await;
        let mut txn = self.collection_storage.begin_write().await?;

        request_store.set_request_node(
            &mut txn,
            request_dir_relative_path_old,
            RequestNodeEntity::Request(RequestEntity {
                order: lease_request_data.order(),
            }),
        )?;

        lease_request_data.set_name(input.new_name);
        lease_request_data.set_path(request_dir_relative_path_new.clone());

        Ok(txn.commit()?)
    }
}
