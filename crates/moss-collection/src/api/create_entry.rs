use moss_common::api::OperationResult;

use crate::{
    collection::Collection,
    models::operations::{CreateEntryInput, CreateEntryOutput},
};

impl Collection {
    pub async fn create_entry(
        &self,
        input: CreateEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        match input {
            CreateEntryInput::Item(input) => self.create_item_entry(input).await,
            CreateEntryInput::Dir(input) => self.create_dir_entry(input).await,
        }
    }
}
