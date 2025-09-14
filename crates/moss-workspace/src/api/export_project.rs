use crate::{
    Workspace,
    models::operations::{ExportProjectInput, ExportProjectOutput},
};
use moss_applib::{AppRuntime, errors::ValidationResultExt};
use validator::Validate;

impl<R: AppRuntime> Workspace<R> {
    pub async fn export_project(
        &self,
        _ctx: &R::AsyncContext,
        input: &ExportProjectInput,
    ) -> joinerror::Result<ExportProjectOutput> {
        input.validate().join_err_bare()?;
        let id = input.inner.id.clone();

        let archive_path = self
            .project_service
            .export_collection(&id, &input.inner)
            .await?;

        Ok(ExportProjectOutput { archive_path })
    }
}
