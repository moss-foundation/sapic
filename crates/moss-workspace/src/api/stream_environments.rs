use futures::StreamExt;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_logging::session;
use sapic_base::project::types::primitives::ProjectId;
use std::sync::Arc;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    Workspace,
    models::{events::StreamEnvironmentsEvent, operations::StreamEnvironmentsOutput},
};

impl Workspace {
    pub async fn stream_environments<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        _app_delegate: AppDelegate<R>,
        channel: TauriChannel<StreamEnvironmentsEvent>,
    ) -> joinerror::Result<StreamEnvironmentsOutput> {
        let stream = self
            .environment_service
            .list_environments(Arc::new(ctx.clone()))
            .await;
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
