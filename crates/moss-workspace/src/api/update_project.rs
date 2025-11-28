use moss_applib::AppRuntime;
use sapic_ipc::ValidationResultExt;
use validator::Validate;

use crate::{
    models::operations::{UpdateProjectInput, UpdateProjectOutput},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn update_project(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateProjectInput,
    ) -> joinerror::Result<UpdateProjectOutput> {
        input.validate().join_err_bare()?;

        let id = input.inner.id.clone().into();
        self.project_service
            .update_project(ctx, &id, input.inner)
            .await?;

        Ok(UpdateProjectOutput { id })
    }
}
