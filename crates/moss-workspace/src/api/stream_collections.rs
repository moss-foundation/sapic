use futures::StreamExt;
use moss_applib::context::Context;
use moss_common::api::OperationResult;
use tauri::{Runtime as TauriRuntime, ipc::Channel as TauriChannel};

use crate::{
    models::events::StreamCollectionsEvent, services::collection_service::CollectionService,
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn stream_collections<C: Context<R>>(
        &self,
        ctx: &C,
        channel: TauriChannel<StreamCollectionsEvent>,
    ) -> OperationResult<()> {
        let collections = self.services.get::<CollectionService>();
        let stream = collections.list_collections();
        tokio::pin!(stream);

        while let Some(collection) = stream.next().await {
            if let Err(e) = channel.send(StreamCollectionsEvent {
                id: collection.id,
                name: collection.name,
                repository: None, // TODO: get from collection manifest
                order: collection.order,
                picture_path: collection.icon_path,
            }) {
                println!("Error sending collection event: {:?}", e); // TODO: log error
            }
        }

        // for collection in collections.values() {
        //     let collection_lock = collection.read().await;
        //     let icon_path = collection_lock
        //         .abs_path()
        //         .join(moss_collection::dirs::ASSETS_DIR)
        //         .join(moss_collection::constants::COLLECTION_ICON_FILENAME);

        //     let manifest = collection_lock.manifest().await;
        //     if let Err(e) = channel.send(StreamCollectionsEvent {
        //         id: collection_lock.id,
        //         name: manifest.name.clone(),
        //         repository: manifest.repository,
        //         order: collection_lock.order,
        //         picture_path: icon_path.exists().then(|| icon_path),
        //     }) {
        //         println!("Error sending collection event: {:?}", e); // TODO: log error
        //     }
        // }

        Ok(())
    }
}
