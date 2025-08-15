use futures::StreamExt;
use moss_applib::AppRuntime;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    Workspace,
    models::{
        events::StreamEnvironmentsEvent, operations::StreamEnvironmentsOutput,
        primitives::CollectionId,
    },
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
                collection_id: item.collection_id.map(|id| CollectionId::from(id)),
                name: item.display_name,
                order: item.order,
            }) {
                println!("Error sending environment event: {:?}", e); // TODO: log error
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
