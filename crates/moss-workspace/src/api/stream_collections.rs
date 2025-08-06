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
        while let Some(collection) = stream.next().await {
            if let Err(e) = channel.send(StreamCollectionsEvent {
                id: collection.id,
                name: collection.name,
                order: collection.order,
                expanded: collection.expanded,
                repository: collection.repository,
                picture_path: collection.icon_path,
            }) {
                println!("Error sending collection event: {:?}", e); // TODO: log error
            } else {
                total_returned += 1;
            }
        }

        Ok(StreamCollectionsOutput { total_returned })
    }
}
