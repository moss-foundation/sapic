use moss_applib::AppRuntime;

use crate::{
    Workspace,
    models::operations::{ArchiveProjectInput, ArchiveProjectOutput},
};

impl Workspace {
    pub async fn archive_project<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        input: ArchiveProjectInput,
    ) -> joinerror::Result<ArchiveProjectOutput> {
        self.project_service.archive_project(ctx, &input.id).await?;

        Ok(ArchiveProjectOutput { id: input.id })
    }
}
