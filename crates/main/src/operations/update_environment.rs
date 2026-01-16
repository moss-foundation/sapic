use moss_applib::AppRuntime;
use sapic_ipc::{
    ValidationResultExt,
    contracts::main::environment::{UpdateEnvironmentInput, UpdateEnvironmentOutput},
};
use validator::Validate;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn update_environment(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateEnvironmentInput,
    ) -> joinerror::Result<UpdateEnvironmentOutput> {
        input.validate().join_err_bare()?;
        let workspace = self.workspace.load();

        let id = input.inner.id.clone();

        if let Some(project_id) = &input.inner.project_id {
            let project = workspace.project(ctx, project_id).await?;
            project.update_environment(ctx, input.inner).await?;
        } else {
            workspace.update_environment(ctx, input.inner).await?;
        }

        Ok(UpdateEnvironmentOutput { id })
    }
}
