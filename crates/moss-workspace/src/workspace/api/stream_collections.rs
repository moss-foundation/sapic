use moss_common::api::OperationResult;
use tauri::{Runtime as TauriRuntime, ipc::Channel};

use crate::{models::events::StreamCollectionsEvent, workspace::Workspace};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn stream_collections(
        &self,
        channel: Channel<StreamCollectionsEvent>,
    ) -> OperationResult<()> {
        let collections = self.collections().await?;
        let collections_lock = collections.read().await;

        for collection in collections_lock.values() {
            if let Err(e) = channel.send(StreamCollectionsEvent {
                id: collection.id,
                display_name: collection.display_name.clone(),
                order: collection.order,
            }) {
                println!("Error sending collection event: {:?}", e); // TODO: log error
            }
        }

        Ok(())
    }
}
