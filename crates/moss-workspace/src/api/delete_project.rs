use moss_applib::AppRuntime;

use crate::{
    models::operations::{DeleteProjectInput, DeleteProjectOutput},
    workspace::Workspace,
};

impl Workspace {
    pub async fn delete_project<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        input: &DeleteProjectInput,
    ) -> joinerror::Result<DeleteProjectOutput> {
        let abs_path = self
            .project_service
            .delete_project::<R>(ctx, &input.id)
            .await?;

        Ok(DeleteProjectOutput {
            id: input.id.to_owned(),
            abs_path: abs_path.map(|path| path.into()),
        })
    }
}
