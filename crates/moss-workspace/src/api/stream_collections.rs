use futures::StreamExt;
use moss_applib::AppRuntime;
use moss_common::api::OperationResult;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    models::{events::StreamCollectionsEvent, operations::StreamCollectionsOutput},
    services::DynCollectionService,
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn stream_collections(
        &self,
        ctx: &R::AsyncContext,
        channel: TauriChannel<StreamCollectionsEvent>,
    ) -> OperationResult<StreamCollectionsOutput> {
        let collections = self.services.get::<DynCollectionService<R>>();
        let stream = collections.list_collections(ctx).await;
        tokio::pin!(stream);

        let mut total_returned = 0;
        while let Some(collection) = stream.next().await {
            if let Err(e) = channel.send(StreamCollectionsEvent {
                id: collection.id,
                name: collection.name,
                repository: collection.repository,
                order: collection.order,
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
