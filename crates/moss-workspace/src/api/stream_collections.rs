use moss_applib::context::Context;
use moss_common::api::OperationResult;
use tauri::{Runtime as TauriRuntime, ipc::Channel as TauriChannel};

use crate::{models::events::StreamCollectionsEvent, workspace::Workspace};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn stream_collections<C: Context<R>>(
        &self,
        ctx: &C,
        channel: TauriChannel<StreamCollectionsEvent>,
    ) -> OperationResult<()> {
        let collections = self.collections(ctx).await?;

        for collection in collections.values() {
            let collection_lock = collection.read().await;
            let manifest = collection_lock.manifest().await;
            if let Err(e) = channel.send(StreamCollectionsEvent {
                id: collection_lock.id,
                name: manifest.name.clone(),
                order: collection_lock.order,
            }) {
                println!("Error sending collection event: {:?}", e); // TODO: log error
            }
        }

        Ok(())
    }
}
