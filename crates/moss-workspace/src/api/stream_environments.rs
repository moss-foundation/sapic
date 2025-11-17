use futures::StreamExt;
use moss_applib::AppRuntime;
use moss_logging::session;
use moss_project::models::primitives::ProjectId;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    Workspace,
    models::{events::StreamEnvironmentsEvent, operations::StreamEnvironmentsOutput},
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn stream_environments(
        &self,
        ctx: &R::AsyncContext,
        channel: TauriChannel<StreamEnvironmentsEvent>,
    ) -> joinerror::Result<StreamEnvironmentsOutput> {
        let stream = self.environment_service.list_environments(ctx).await;
        tokio::pin!(stream);

        let mut total_returned = 0;
        while let Some(item) = stream.next().await {
            if let Err(e) = channel.send(StreamEnvironmentsEvent {
                id: item.id,
                project_id: item.project_id.map(|id| ProjectId::from(id)),
                is_active: item.is_active,
                name: item.display_name,
                order: item.order,
                total_variables: item.total_variables,
            }) {
                session::error!(format!(
                    "failed to send environment event through tauri channel: {}",
                    e.to_string()
                ));
            } else {
                total_returned += 1;
            }
        }

        Ok(StreamEnvironmentsOutput {
            total_returned,
            groups: self
                .environment_service
                .list_environment_groups(ctx)
                .await?,
        })
    }
}
