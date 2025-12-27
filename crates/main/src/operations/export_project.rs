use moss_applib::AppRuntime;
use sapic_ipc::{
    ValidationResultExt,
    contracts::main::project::{ExportProjectInput, ExportProjectOutput, ExportProjectParams},
};
use validator::Validate;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn export_project(
        &self,
        ctx: &R::AsyncContext,
        input: &ExportProjectInput,
    ) -> joinerror::Result<ExportProjectOutput> {
        input.validate().join_err_bare()?;

        let archive_path = self
            .workspace
            .load()
            .export_project(
                ctx,
                ExportProjectParams {
                    id: input.inner.id.clone(),
                    destination: input.inner.destination.clone(),
                },
            )
            .await?;

        Ok(ExportProjectOutput { archive_path })
    }
}
