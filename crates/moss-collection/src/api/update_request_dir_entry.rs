use moss_common::api::{OperationError, OperationResult};
use moss_common::sanitized::SanitizedName;
use std::sync::Arc;
use validator::Validate;

use crate::worktree::{
    ChangesDiffSet, Worktree,
    common::{is_dir, path_not_ends_with_extension, path_starts_with, validate_entry},
    snapshot::EntryRef,
};
use crate::{
    collection::Collection,
    models::operations::{UpdateRequestDirEntryInput, UpdateRequestDirEntryOutput},
};

impl Collection {
    pub async fn update_request_dir_entry(
        &self,
        input: UpdateRequestDirEntryInput,
    ) -> OperationResult<UpdateRequestDirEntryOutput> {
        input.validate()?;

        let worktree = self.worktree().await?;
        let entry = {
            let snapshot_lock = worktree.read().await;
            let entry = snapshot_lock
                .entry_by_id(input.id)
                .ok_or(OperationError::NotFound(format!(
                    "request directory with id {}",
                    input.id,
                )))?;
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

        let changes = if let Some(new_name) = input.name {
            self.process_request_renaming(&worktree, &entry, &SanitizedName::new(&new_name))
                .await?
        } else {
            Arc::from((vec![]).into_boxed_slice())
        };

        Ok(UpdateRequestDirEntryOutput {
            changed_paths: changes,
        })
    }

    async fn process_request_renaming(
        &self,
        worktree: &Arc<Worktree>,
        entry: &EntryRef,
        new_name: &SanitizedName,
    ) -> OperationResult<ChangesDiffSet> {
        let mut new_path = entry.path.to_path_buf();
        new_path.set_file_name(new_name);
        if new_path == entry.path.to_path_buf() {
            return Ok(Arc::from((vec![]).into_boxed_slice()));
        }

        let changes = worktree
            .rename_entry(Arc::clone(&entry.path), new_path)
            .await?;

        // TODO: update the state database

        Ok(changes)
    }
}
