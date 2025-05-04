use moss_common::api::OperationResult;
use validator::Validate;

use crate::{
    collection::Collection,
    models::operations::{CreateRequestEntryInput, CreateRequestEntryOutput},
};

impl Collection {
    pub async fn create_request_entry(
        &self,
        input: CreateRequestEntryInput,
    ) -> OperationResult<CreateRequestEntryOutput> {
        input.validate()?;

        let worktree = self.worktree().await?;

        todo!()
    }
}
