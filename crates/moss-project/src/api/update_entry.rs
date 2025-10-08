use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{
    models::operations::{UpdateEntryInput, UpdateEntryOutput},
    project::Project,
};

impl<R: AppRuntime> Project<R> {
    pub async fn update_entry(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: UpdateEntryInput,
    ) -> joinerror::Result<UpdateEntryOutput> {
        match input {
            UpdateEntryInput::Item(input) => {
                let output = self.update_item_entry(ctx, app_delegate, input).await?;
                Ok(UpdateEntryOutput::Item(output))
            }
            UpdateEntryInput::Dir(input) => {
                let output = self.update_dir_entry(ctx, input).await?;
                Ok(UpdateEntryOutput::Dir(output))
            }
        }
    }
}
