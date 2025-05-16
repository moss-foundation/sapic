use moss_common::api::{OperationError, OperationResult};
use std::sync::Arc;

use crate::collection::Collection;
use crate::models::operations::{DeleteRequestEntryInput, DeleteRequestEntryOutput};
use crate::worktree::common::{is_dir, path_ends_with_extension, path_starts_with, validate_entry};

impl Collection {
    pub async fn delete_request_entry(
        &self,
        input: DeleteRequestEntryInput,
    ) -> OperationResult<DeleteRequestEntryOutput> {
        let worktree = self.worktree().await?;

        let entry = {
            let snapshot_lock = worktree.read().await;
            let entry = snapshot_lock
                .entry_by_id(input.id)
                .ok_or(OperationError::NotFound(format!(
                    "request entry with id {}",
                    input.id,
                )))?;
            Arc::clone(&entry)
        };

        validate_entry(
            &entry,
            &[
                is_dir(),
                path_ends_with_extension("request"),
                path_starts_with("requests"),
            ],
        )?;

        let changed_paths = worktree.remove_entry(&entry.path).await?;

        // TODO: update the state database

        Ok(DeleteRequestEntryOutput { changed_paths })
    }
}
