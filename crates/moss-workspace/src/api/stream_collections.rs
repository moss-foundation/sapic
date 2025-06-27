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
            let icon_path = collection_lock
                .abs_path()
                .join(moss_collection::dirs::ASSETS_DIR)
                .join(moss_collection::constants::COLLECTION_ICON_FILENAME);

            let manifest = collection_lock.manifest().await;
            if let Err(e) = channel.send(StreamCollectionsEvent {
                id: collection_lock.id,
                name: manifest.name.clone(),
                order: collection_lock.order,
                picture_path: icon_path.exists().then(|| icon_path),
            }) {
                println!("Error sending collection event: {:?}", e); // TODO: log error
            }
        }

        Ok(())
    }
}
