use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    collection::Collection,
    models::operations::{UpdateEntryInput, UpdateEntryOutput},
};

impl<R: AppRuntime> Collection<R> {
    pub async fn update_entry(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateEntryInput,
    ) -> OperationResult<UpdateEntryOutput> {
        match input {
            UpdateEntryInput::Item(input) => {
                let output = self.update_item_entry(ctx, input).await?;
                Ok(UpdateEntryOutput::Item(output))
            }
            UpdateEntryInput::Dir(input) => {
                let output = self.update_dir_entry(ctx, input).await?;
                Ok(UpdateEntryOutput::Dir(output))
            }
        }
    }
}
