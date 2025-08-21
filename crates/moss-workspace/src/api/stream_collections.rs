use futures::StreamExt;
use moss_applib::AppRuntime;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    models::{events::StreamCollectionsEvent, operations::StreamCollectionsOutput},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn stream_collections(
        &self,
        ctx: &R::AsyncContext,
        channel: TauriChannel<StreamCollectionsEvent>,
    ) -> joinerror::Result<StreamCollectionsOutput> {
        let stream = self.collection_service.list_collections(ctx).await;
        tokio::pin!(stream);

        let mut total_returned = 0;
        while let Some(desc) = stream.next().await {
            let event = StreamCollectionsEvent {
                id: desc.id,
                name: desc.name,
                order: desc.order,
                expanded: desc.expanded,
                branch: desc.vcs.and_then(|vcs| vcs.branch()),
                icon_path: desc.icon_path,
            };

            if let Err(e) = channel.send(event) {
                println!("Error sending collection event: {:?}", e); // TODO: log error
            } else {
                total_returned += 1;
            }
        }

        Ok(StreamCollectionsOutput { total_returned })
    }
}
