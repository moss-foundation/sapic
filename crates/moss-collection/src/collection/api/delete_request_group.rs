use anyhow::{anyhow, Context, Result};
use moss_fs::utils::encode_path;
use moss_fs::RemoveOptions;
use validator::Validate;

use crate::collection::Collection;
use crate::constants::REQUESTS_DIR;
use crate::models::operations::{DeleteRequestGroupInput, DeleteRequestInput};

impl Collection {
    pub async fn delete_request_group(&self, input: DeleteRequestGroupInput) -> Result<()> {
        let group_data = {
            let request_nodes = self.registry().await?.requests_nodes();
            let mut requests_lock = request_nodes.write().await;
            if !requests_lock.get(input.key)?.is_request_group() {
                return Err(anyhow!("Resource {} is not a request group", input.key));
            }
            requests_lock.remove(input.key)?
        };

        let group_dir_relative_path = group_data.path().clone();

        let requests = self.registry().await?.requests_nodes();

        // TODO: logging an error when encounter an error with leased key

        let keys_to_delete = {
            let requests_lock = requests.read().await;
            requests_lock
                .iter()
                .filter_map(|(key, iter_slot)| {
                    if iter_slot
                        .value()
                        .path()
                        .starts_with(&group_dir_relative_path)
                    {
                        Some(key)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        };

        for key in keys_to_delete {
            let result = self.delete_request(DeleteRequestInput { key }).await;
            if result.is_err() {
                // TODO: log the error
            }
        }

        let group_dir_full_path = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&group_dir_relative_path);

        if !group_dir_full_path.exists() {
            return Ok(());
        }

        let request_store = self.state_db_manager.request_store().await;
        let (mut txn, table) = request_store.begin_write()?;
        table.remove(
            &mut txn,
            group_dir_relative_path.to_string_lossy().to_string(),
        )?;
        // TODO: Self-healing for failure?
        self.fs
            .remove_dir(
                &group_dir_full_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await
            .context("Failed to remove the request group directory")?;

        txn.commit()?;

        Ok(())
    }
}
