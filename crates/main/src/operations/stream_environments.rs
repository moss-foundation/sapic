use moss_applib::AppRuntime;
use moss_common::continue_if_err;
use moss_environment::AnyEnvironment;
use sapic_ipc::contracts::main::environment::{
    EnvironmentGroup, StreamEnvironmentsEvent, StreamEnvironmentsOutput,
};
use std::error::Error;
use tauri::ipc::Channel as TauriChannel;

use crate::{MainWindow, workspace::GLOBAL_ACTIVE_ENVIRONMENT_KEY};

impl<R: AppRuntime> MainWindow<R> {
    pub async fn stream_environments(
        &self,
        ctx: &R::AsyncContext,
        channel: TauriChannel<StreamEnvironmentsEvent>,
    ) -> joinerror::Result<StreamEnvironmentsOutput> {
        let workspace = self.workspace.load();
        let environments = workspace.environments(ctx).await?;
        let active_environments = workspace.active_environments(ctx).await?;

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
            let env_group_key = if let Some(project_id) = &project_id {
                project_id.inner()
            } else {
                GLOBAL_ACTIVE_ENVIRONMENT_KEY.to_string().into()
            };

            let is_active = active_environments.get(&env_group_key) == Some(&id);

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
            // FIXME: Is returning environment groups still necessary?
            // Looks like the only things associated with them are expanded flag and order
            // Which would be handled on the frontend
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
