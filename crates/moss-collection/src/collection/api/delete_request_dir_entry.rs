use anyhow::Context as _;
use moss_common::api::OperationResult;
use std::sync::Arc;

use crate::{
    collection::{
        Collection,
        worktree::common::{is_dir, path_not_ends_with, path_starts_with, validate_entry},
    },
    models::operations::{DeleteRequestDirEntryInput, DeleteRequestDirEntryOutput},
};

impl Collection {
    pub async fn delete_request_dir_entry(
        &self,
        input: DeleteRequestDirEntryInput,
    ) -> OperationResult<DeleteRequestDirEntryOutput> {
        let worktree = self.worktree().await?;

        let entry = {
            let snapshot_lock = worktree.read().await;
            let entry = snapshot_lock
                .entry_by_id(input.id)
                .context("Entry not found")?; // TODO: replace with OperationError::NotFound

            Arc::clone(&entry)
        };

        validate_entry(
            &entry,
            &[
                is_dir(),
                path_not_ends_with(".request"),
                path_starts_with("requests"),
            ],
        )?;

        let changes = worktree.remove_entry(&entry.path).await?;

        // TODO: update the state database

        Ok(DeleteRequestDirEntryOutput {
            changed_paths: changes,
        })
    }
}
