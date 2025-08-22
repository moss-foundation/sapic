use futures::StreamExt;
use moss_applib::AppRuntime;
use moss_logging::session;
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
            // TODO: It might be better to separate the sending of information fetched from HTTP
            // from the main streaming, which will make the application more responsive
            // Right now the latency from HTTP requests slows down this operation quite a lot

            let collection = self.collection(&desc.id).await;
            if collection.is_none() {
                // This should never happen since the collection is already returned from the stream
                continue;
            }
            let collection = collection.unwrap();

            let branch_info = collection.get_current_branch_info().await;

            let branch = match branch_info {
                Ok(branch) => Some(branch),
                Err(e) => {
                    // TODO: Tell the frontend that we failed to fetch current branch info
                    // TODO: Logging Resource
                    session::error!(format!(
                        "failed to fetch current branch info: {}",
                        e.to_string()
                    ));
                    None
                }
            };

            let event = StreamCollectionsEvent {
                id: desc.id,
                name: desc.name,
                order: desc.order,
                expanded: desc.expanded,
                repository: desc.repository,
                branch,
                icon_path: desc.icon_path,
            };

            if let Err(e) = channel.send(event) {
                session::error!(format!(
                    "failed to send collection event through tauri channel: {}",
                    e.to_string()
                ));
            } else {
                total_returned += 1;
            }
        }

        Ok(StreamCollectionsOutput { total_returned })
    }
}
