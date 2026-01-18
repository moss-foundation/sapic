use moss_applib::AppRuntime;
use sapic_ipc::contracts::main::environment::{
    ActivateEnvironmentInput, ActivateEnvironmentOutput,
};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn activate_environment(
        &self,
        ctx: &R::AsyncContext,
        input: ActivateEnvironmentInput,
    ) -> joinerror::Result<ActivateEnvironmentOutput> {
        let workspace = self.workspace.load();

        if let Some(project_id) = input.project_id {
            let project = workspace.project(ctx, &project_id).await?;
            project
                .activate_environment(ctx, &input.environment_id)
                .await?;
        } else {
            workspace
                .activate_environment(ctx, &input.environment_id)
                .await?;
        }

        Ok(ActivateEnvironmentOutput {
            environment_id: input.environment_id,
        })
    }
}
