use moss_applib::AppRuntime;
use sapic_core::context::AnyAsyncContext;
use sapic_ipc::contracts::main::environment::{
    DescribeEnvironmentInput, DescribeEnvironmentOutput,
};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn describe_environment(
        &self,
        ctx: &dyn AnyAsyncContext,
        input: &DescribeEnvironmentInput,
    ) -> joinerror::Result<DescribeEnvironmentOutput> {
        let desc = if let Some(project_id) = &input.project_id {
            let project = self.workspace.load().project(ctx, project_id).await?;
            project
                .describe_environment(ctx, &input.environment_id)
                .await?
        } else {
            let workspace = self.workspace.load();

            workspace
                .describe_environment(ctx, &input.environment_id)
                .await?
        };

        Ok(DescribeEnvironmentOutput {
            name: desc.name,
            color: desc.color,
            variables: desc
                .variables
                .into_iter()
                .map(|(_, var_info)| var_info)
                .collect(),
        })
    }
}
