use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    collection::Collection,
    models::operations::{CreateEntryInput, CreateEntryOutput},
};

impl<R: AppRuntime> Collection<R> {
    pub async fn create_entry(
        &self,
        ctx: &R::AsyncContext,
        input: CreateEntryInput,
    ) -> OperationResult<CreateEntryOutput> {
        match input {
            CreateEntryInput::Item(input) => self.create_item_entry(ctx, input).await,
            CreateEntryInput::Dir(input) => self.create_dir_entry(ctx, input).await,
        }
    }
}
