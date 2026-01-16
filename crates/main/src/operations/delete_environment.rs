use moss_applib::AppRuntime;
use sapic_ipc::contracts::main::environment::{DeleteEnvironmentInput, DeleteEnvironmentOutput};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn delete_environment(
        &self,
        ctx: &R::AsyncContext,
        input: DeleteEnvironmentInput,
    ) -> joinerror::Result<DeleteEnvironmentOutput> {
        let workspace = self.workspace.load();

        if let Some(project_id) = input.project_id {
            let project = workspace.project(ctx, &project_id).await?;
            project.delete_environment(ctx, &input.id).await?;
        } else {
            workspace.delete_environment(ctx, &input.id).await?;
        }

        Ok(DeleteEnvironmentOutput { id: input.id })
    }
}
