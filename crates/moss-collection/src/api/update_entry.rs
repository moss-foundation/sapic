use moss_common::api::OperationResult;

use crate::{
    collection::Collection,
    models::operations::{UpdateEntryInput, UpdateEntryOutput},
};

impl Collection {
    pub async fn update_entry(
        &self,
        input: UpdateEntryInput,
    ) -> OperationResult<UpdateEntryOutput> {
        match input {
            UpdateEntryInput::Item(input) => {
                let output = self.update_item_entry(input).await?;
                Ok(UpdateEntryOutput::Item(output))
            }
            UpdateEntryInput::Dir(input) => {
                let output = self.update_dir_entry(input).await?;
                Ok(UpdateEntryOutput::Dir(output))
            }
        }
    }
}
