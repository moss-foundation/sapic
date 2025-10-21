use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;

use crate::{
    models::operations::{UpdateResourceInput, UpdateResourceOutput},
    project::Project,
};

impl<R: AppRuntime> Project<R> {
    pub async fn update_entry(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        input: UpdateResourceInput,
    ) -> joinerror::Result<UpdateResourceOutput> {
        match input {
            UpdateResourceInput::Item(input) => {
                let output = self.update_item_entry(ctx, app_delegate, input).await?;
                Ok(UpdateResourceOutput::Item(output))
            }
            UpdateResourceInput::Dir(input) => {
                let output = self.update_dir_entry(ctx, input).await?;
                Ok(UpdateResourceOutput::Dir(output))
            }
        }
    }
}
