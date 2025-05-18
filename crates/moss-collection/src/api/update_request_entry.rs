use moss_common::api::{OperationError, OperationResult};
use moss_common::sanitized::sanitized_name::SanitizedName;
use std::sync::Arc;
use validator::Validate;

use crate::collection::Collection;
use crate::models::{
    operations::{UpdateRequestEntryInput, UpdateRequestEntryOutput},
    primitives::ChangesDiffSet,
};
use crate::worktree::util::validation::{
    is_dir, path_ends_with_extension, path_starts_with, validate_entry,
};
use crate::worktree::{physical_snapshot::PhysicalEntry, physical_worktree::PhysicalWorktree};

impl Collection {
    pub async fn update_request_entry(
        &self,
        input: UpdateRequestEntryInput,
    ) -> OperationResult<UpdateRequestEntryOutput> {
        todo!()

        // input.validate()?;

        // let worktree = self.worktree().await?;
        // let entry = {
        //     let snapshot_lock = worktree.read().await;
        //     let entry = snapshot_lock
        //         .entry_by_id(input.id)
        //         .ok_or(OperationError::NotFound(format!(
        //             "request entry with id {}",
        //             input.id,
        //         )))?;

        //     Arc::clone(&entry)
        // };

        // validate_entry(
        //     &entry,
        //     &[
        //         is_dir(),
        //         path_ends_with_extension("request"),
        //         path_starts_with("requests"),
        //     ],
        // )?;

        // let changes = if let Some(new_name) = input.name {
        //     self.process_dir_renaming(&worktree, &entry, &SanitizedName::new(&new_name))
        //         .await?
        // } else {
        //     Arc::from((vec![]).into_boxed_slice())
        // };

        // Ok(UpdateRequestEntryOutput {
        //     changed_paths: changes,
        // })
    }

    // async fn process_dir_renaming(
    //     &self,
    //     worktree: &Arc<PhysicalWorktree>,
    //     entry: &Arc<PhysicalEntry>,
    //     new_name: &SanitizedName,
    // ) -> OperationResult<ChangesDiffSet> {
    //     let mut new_path = entry.path.to_path_buf();
    //     new_path.set_file_name(format!("{new_name}.request"));
    //     if new_path == entry.path.to_path_buf() {
    //         return Ok(Arc::from((vec![]).into_boxed_slice()));
    //     }

    //     let changes = worktree
    //         .rename_entry(Arc::clone(&entry.path), new_path)
    //         .await?;

    //     // TODO: update the state database

    //     Ok(changes)
    // }
}
