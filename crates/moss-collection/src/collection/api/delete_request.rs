use anyhow::{anyhow, Context as _, Result};
use moss_fs::RemoveOptions;

use crate::collection::{Collection, OperationError, REQUESTS_DIR};
use crate::models::operations::DeleteRequestInput;

impl Collection {
    pub async fn delete_request(&self, input: DeleteRequestInput) -> Result<()> {
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

        let request_store = self.state_db_manager.request_store().await;
        let (mut txn, table) = request_store.begin_write()?;
        table.remove(
            &mut txn,
            request_dir_relative_path.to_string_lossy().to_string(),
        )?;

        txn.commit()?;

        Ok(())
    }
}
