use moss_applib::AppRuntime;

use crate::{
    Workspace,
    models::operations::{UnarchiveProjectInput, UnarchiveProjectOutput},
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn unarchive_project(
        &self,
        ctx: &R::AsyncContext,
        input: UnarchiveProjectInput,
    ) -> joinerror::Result<UnarchiveProjectOutput> {
        self.project_service
            .unarchive_project(ctx, &input.id)
            .await?;

        Ok(UnarchiveProjectOutput { id: input.id })
    }
}
