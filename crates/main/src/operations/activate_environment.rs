use moss_applib::AppRuntime;
use sapic_core::context::AnyAsyncContext;
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

        workspace
            .activate_environment(ctx, &input.environment_id)
            .await?;

        Ok(ActivateEnvironmentOutput {
            environment_id: input.environment_id,
        })
    }
}
