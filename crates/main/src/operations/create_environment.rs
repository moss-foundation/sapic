use moss_applib::AppRuntime;
use sapic_ipc::{
    ValidationResultExt,
    contracts::main::environment::{CreateEnvironmentInput, CreateEnvironmentOutput},
};
use validator::Validate;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn create_environment(
        &self,
        ctx: &R::AsyncContext,
        input: CreateEnvironmentInput,
    ) -> joinerror::Result<CreateEnvironmentOutput> {
        input.validate().join_err_bare()?;

        let workspace = self.workspace.load();

        let result = if let Some(project_id) = &input.project_id {
            let project = workspace.project(ctx, &project_id).await?;
            project.create_environment(ctx, input).await?
        } else {
            workspace.create_environment(ctx, input).await?
        };

        Ok(CreateEnvironmentOutput {
            id: result.id,
            project_id: result.project_id,
            name: result.display_name,
            order: result.order,
            color: result.color,
            abs_path: result.abs_path.to_path_buf(),
        })
    }
}
