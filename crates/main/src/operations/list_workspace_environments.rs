use joinerror::ResultExt;
use moss_applib::AppRuntime;
use sapic_ipc::contracts::main::environment::{
    ListEnvironmentItem, ListWorkspaceEnvironmentsOutput,
};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn list_workspace_environments(
        &self,
        ctx: &R::AsyncContext,
    ) -> joinerror::Result<ListWorkspaceEnvironmentsOutput> {
        let workspace = self.workspace.load();
        let environments = workspace.environments(ctx).await?;
        let active_environment = workspace.active_environment(ctx).await?;

        let mut items = vec![];
        for environment in environments {
            let desc = workspace
                .describe_environment(ctx, &environment.id)
                .await
                .join_err_with::<()>(|| {
                    format!(
                        "failed to describe environment {}",
                        environment.id.to_string()
                    )
                })?;

            items.push(ListEnvironmentItem {
                id: environment.id.clone(),
                is_active: active_environment == Some(environment.id),
                name: desc.name,
                color: desc.color,
                total_variables: desc.variables.len(),
            });
        }

        Ok(ListWorkspaceEnvironmentsOutput { items })
    }
}
