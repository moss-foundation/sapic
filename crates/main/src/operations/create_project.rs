use moss_applib::AppRuntime;
use sapic_ipc::{
    ValidationResultExt,
    contracts::main::project::{CreateProjectInput, CreateProjectOutput},
};
use validator::Validate;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn create_project(
        &self,
        ctx: &R::AsyncContext,
        input: &CreateProjectInput,
    ) -> joinerror::Result<CreateProjectOutput> {
        input.validate().join_err_bare()?;

        let project = self
            .workspace
            .load()
            .create_project(ctx, input.inner.clone())
            .await?;

        let details = project.handle.details(ctx).await?;

        Ok(CreateProjectOutput {
            id: project.id,
            name: details.name,
            order: project.order,
            expanded: true, // HACK: hardcoded value
            icon_path: project.handle.icon_path(),
            abs_path: project.handle.internal_abs_path().to_path_buf(),
            external_path: project.handle.external_abs_path().map(|p| p.to_path_buf()),
        })
    }
}
