use anyhow::Context as _;
use moss_common::api::{OperationError, OperationResult};
use moss_fs::RemoveOptions;

use crate::collection::{Collection, REQUESTS_DIR};
use crate::models::operations::DeleteRequestInput;

impl Collection {
    pub async fn delete_request(&self, input: DeleteRequestInput) -> OperationResult<()> {
        let request_data = {
            let request_nodes = self.registry().await?.requests_nodes();
            let mut requests_lock = request_nodes.write().await;
            if !requests_lock.get(input.key)?.is_request() {
                return Err(OperationError::Validation(format!(
                    "Resource {} is not a request",
                    input.key
                ))
                .into());
            }
            requests_lock.remove(input.key)?
        };

        let request_dir_relative_path = request_data.path().clone();
        let request_dir_abs_path = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&request_dir_relative_path);

        // TODO: Add logging when the request was already deleted from the fs?
        // TODO: Self-healing process
        self.fs
            .remove_dir(
                &request_dir_abs_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await
            .context("Failed to remove the request directory")?;

        let request_store = self.collection_storage.request_store().await;
        let mut txn = self.collection_storage.begin_write().await?;
        request_store.delete_request_node(&mut txn, request_dir_relative_path.clone())?;

        txn.commit()?;

        Ok(())
    }
}
