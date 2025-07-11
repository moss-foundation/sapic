use futures::StreamExt;
use moss_applib::context::Context;
use moss_common::api::OperationResult;
use tauri::{Runtime as TauriRuntime, ipc::Channel as TauriChannel};

use crate::{
    models::{events::StreamCollectionsEvent, operations::StreamCollectionsOutput},
    services::DynCollectionService,
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn stream_collections<C: Context<R>>(
        &self,
        _ctx: &C,
        channel: TauriChannel<StreamCollectionsEvent>,
    ) -> OperationResult<StreamCollectionsOutput> {
        let collections = self.services.get::<DynCollectionService>();
        let stream = collections.list_collections();
        tokio::pin!(stream);

        let mut total_returned = 0;
        while let Some(collection) = stream.next().await {
            if let Err(e) = channel.send(StreamCollectionsEvent {
                id: collection.id,
                name: collection.name,
                repository: None, // TODO: get from collection manifest
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
