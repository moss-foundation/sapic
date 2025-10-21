use moss_applib::AppRuntime;

use crate::{
    models::operations::{CreateResourceInput, CreateResourceOutput},
    project::Project,
};

impl<R: AppRuntime> Project<R> {
    pub async fn create_resource(
        &self,
        ctx: &R::AsyncContext,
        input: CreateResourceInput,
    ) -> joinerror::Result<CreateResourceOutput> {
        match input {
            CreateResourceInput::Item(input) => self.create_item_resource(ctx, input).await,
            CreateResourceInput::Dir(input) => self.create_dir_resource(ctx, input).await,
        }
    }
}
