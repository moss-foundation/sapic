use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    collection::Collection,
    models::operations::{CreateRequestDirEntryInput, CreateRequestDirEntryOutput},
};

impl Collection {
    pub async fn create_request_dir_entry(
        &self,
        input: CreateRequestDirEntryInput,
    ) -> OperationResult<CreateRequestDirEntryOutput> {
        input.validate()?;

        let worktree = self.worktree().await?;

        let encoded_path = moss_fs::utils::encode_path(&input.destination, None)?;
        let changes = worktree.create_entry(&encoded_path, true, None).await?;

        Ok(CreateRequestDirEntryOutput { changes })
    }
}
