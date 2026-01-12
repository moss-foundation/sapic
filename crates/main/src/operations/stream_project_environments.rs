use crate::MainWindow;
use moss_applib::AppRuntime;
use moss_common::continue_if_err;
use moss_environment::AnyEnvironment;
use sapic_base::project::types::primitives::ProjectId;
use sapic_ipc::contracts::main::environment::{
    EnvironmentGroup, StreamEnvironmentsEvent, StreamEnvironmentsOutput,
    StreamProjectEnvironmentsInput, StreamProjectEnvironmentsOutput,
};
use tauri::ipc::Channel as TauriChannel;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn stream_project_environments(
        &self,
        ctx: &R::AsyncContext,
        input: StreamProjectEnvironmentsInput,
        channel: TauriChannel<StreamEnvironmentsEvent>,
    ) -> joinerror::Result<StreamProjectEnvironmentsOutput> {
        let project = self
            .workspace
            .load()
            .project(ctx, &input.project_id)
            .await?;

        let environments = project.environments(ctx).await?;
        let active_environment = project.active_environment(ctx).await?;

        let mut total_returned = 0;
        for environment in environments {
            let desc = continue_if_err!(environment.describe(ctx).await, |e| {
                tracing::warn!(
                    "failed to describe environment {}: {}",
                    environment.id.to_string(),
                    e
                )
            });

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

        Ok(StreamProjectEnvironmentsOutput { total_returned })
    }
}
