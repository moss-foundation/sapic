use moss_applib::AppRuntime;

use crate::{
    models::operations::{CreateResourceInput, CreateResourceOutput},
    project::Project,
};

impl Project {
    pub async fn create_resource<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        input: CreateResourceInput,
    ) -> joinerror::Result<CreateResourceOutput> {
        match input {
            CreateResourceInput::Item(input) => self.create_item_resource::<R>(ctx, input).await,
            CreateResourceInput::Dir(input) => self.create_dir_resource::<R>(ctx, input).await,
        }
    }
}
