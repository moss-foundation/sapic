use moss_applib::AppRuntime;

use crate::{
    models::operations::{DeleteProjectInput, DeleteProjectOutput},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn delete_project(
        &self,
        ctx: &R::AsyncContext,
        input: &DeleteProjectInput,
    ) -> joinerror::Result<DeleteProjectOutput> {
        let abs_path = self.project_service.delete_project(ctx, &input.id).await?;

        Ok(DeleteProjectOutput {
            id: input.id.to_owned(),
            abs_path: abs_path.map(|path| path.into()),
        })
    }
}
