use joinerror::ResultExt;
use moss_applib::AppRuntime;
use sapic_ipc::contracts::main::environment::{
    ListEnvironmentItem, ListProjectEnvironmentsInput, ListProjectEnvironmentsOutput,
};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn list_project_environments(
        &self,
        ctx: &R::AsyncContext,
        input: ListProjectEnvironmentsInput,
    ) -> joinerror::Result<ListProjectEnvironmentsOutput> {
        let project = self
            .workspace
            .load()
            .project(ctx, &input.project_id)
            .await?;
        let environments = project.environments(ctx).await?;
        let active_environment = project.active_environment(ctx).await?;

        let mut items = vec![];
        for environment in environments {
            let desc = project
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

        Ok(ListProjectEnvironmentsOutput { items })
    }
}
