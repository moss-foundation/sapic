use moss_common::api::{OperationError, OperationResult};
use std::sync::Arc;

use crate::worktree::common::{
    is_dir, path_not_ends_with_extension, path_starts_with, validate_entry,
};
use crate::{
    collection::Collection,
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
                .ok_or(OperationError::NotFound {
                    name: input.id.to_string(),
                    path: Default::default(),
                })?;
            Arc::clone(&entry)
        };

        validate_entry(
            &entry,
            &[
                is_dir(),
                path_not_ends_with_extension("request"),
                path_starts_with("requests"),
            ],
        )?;

        let changed_paths = worktree.remove_entry(&entry.path).await?;

        // TODO: update the state database

        Ok(DeleteRequestDirEntryOutput { changed_paths })
    }
}
