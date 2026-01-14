use moss_applib::AppRuntime;
use moss_common::continue_if_err;
use sapic_ipc::contracts::main::environment::{
    EnvironmentGroup, StreamEnvironmentsEvent, StreamEnvironmentsOutput,
};
use tauri::ipc::Channel as TauriChannel;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn stream_environments(
        &self,
        ctx: &R::AsyncContext,
        channel: TauriChannel<StreamEnvironmentsEvent>,
    ) -> joinerror::Result<StreamEnvironmentsOutput> {
        let workspace = self.workspace.load();
        let environments = workspace.environments(ctx).await?;
        let active_environment = workspace.active_environment(ctx).await?;

        let mut total_returned = 0;
        for environment in environments {
            let desc = continue_if_err!(
                workspace.describe_environment(ctx, &environment.id).await,
                |e| {
                    tracing::warn!(
                        "failed to describe environment {}: {}",
                        environment.id.to_string(),
                        e
                    )
                }
            );

            let id = environment.id;
            let project_id = environment.project_id;

            let is_active = active_environment == Some(id.clone());

            if let Err(e) = channel.send(StreamEnvironmentsEvent {
                id,
                project_id,
                is_active,
                name: desc.name,
                order: None,
                total_variables: desc.variables.len(),
            }) {
                tracing::error!("failed to send stream environments event: {}", e);
            } else {
                total_returned += 1;
            }
        }

        Ok(StreamEnvironmentsOutput {
            // FIXME: Remove the concept of environment group
            groups: workspace
                .environment_groups(ctx)
                .await?
                .into_iter()
                .map(|group_id| {
                    EnvironmentGroup {
                        project_id: group_id.inner(),
                        // FIXME: These should be removed from the backend
                        expanded: false,
                        order: None,
                    }
                })
                .collect(),
            total_returned,
        })
    }
}
