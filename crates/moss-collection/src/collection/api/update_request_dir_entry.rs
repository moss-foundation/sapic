use std::sync::Arc;

use anyhow::Context;
use moss_common::api::{OperationError, OperationResult};
use validator::Validate;

use crate::{
    collection::{
        Collection,
        worktree::snapshot::{self, EntryRef},
    },
    models::operations::UpdateRequestDirEntryInput,
    worktree,
};

impl Collection {
    pub async fn update_request_dir_entry(
        &self,
        input: UpdateRequestDirEntryInput,
    ) -> OperationResult<()> {
        input.validate()?;

        let snapshot = self.worktree().await?.lock().await;

        let entry = snapshot.entry_by_id(input.id).context("Entry not found")?; // TODO: replace with OperationError::NotFound

        if !entry.is_dir() {
            return Err(OperationError::Validation(format!(
                "Entry is not a directory: {}",
                entry.path.display()
            )));
        }

        if let Some(new_name) = input.name {
            self.process_dir_renaming(&entry, &new_name).await?;
        }

        Ok(())
    }

    async fn process_dir_renaming(&self, entry: &EntryRef, new_name: &str) -> OperationResult<()> {
        let worktree = self.worktree().await?;

        let new_path = entry.path.with_file_name(&format!("{}.request", new_name));
        worktree
            .rename_entry(Arc::clone(&entry.path), new_path)
            .await?;

        Ok(())
    }
}
