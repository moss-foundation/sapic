use moss_applib::AppRuntime;
use sapic_ipc::ValidationResultExt;
use validator::Validate;

use crate::{
    Workspace,
    models::operations::{ExportProjectInput, ExportProjectOutput},
};

impl Workspace {
    pub async fn export_project<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        input: &ExportProjectInput,
    ) -> joinerror::Result<ExportProjectOutput> {
        input.validate().join_err_bare()?;
        let id = input.inner.id.clone();

        let archive_path = self
            .project_service
            .export_collection(ctx, &id, &input.inner)
            .await?;

        Ok(ExportProjectOutput { archive_path })
    }
}
