use anyhow::{Context, Result};
use moss_fs::utils::encode_path;
use moss_fs::RemoveOptions;
use validator::Validate;

use crate::collection::Collection;
use crate::constants::REQUESTS_DIR;
use crate::models::operations::{DeleteRequestGroupInput, DeleteRequestInput};

impl Collection {
    pub async fn delete_request_group(&self, input: DeleteRequestGroupInput) -> Result<()> {
        input.validate()?;
        // FIXME: we won't need this once we implement `ResourceKey`
        let encoded_path = encode_path(&input.path, None)?;

        let requests = self.registry().await?.requests_nodes();

        // TODO: logging an error when encounter an error with leased key

        let keys_to_delete = {
            let requests_lock = requests.read().await;
            requests_lock
                .iter()
                .filter_map(|(key, iter_slot)| {
                    if iter_slot.value().path().starts_with(&encoded_path) {
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

        let request_group_full_path = self.abs_path.join(REQUESTS_DIR).join(&encoded_path);

        if !request_group_full_path.exists() {
            return Ok(());
        }

        // TODO: Self-healing for failure?
        self.fs
            .remove_dir(
                &request_group_full_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await
            .context("Failed to remove the request group directory")?;

        Ok(())
    }
}
