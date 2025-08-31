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

        // OPTIMIZE: Right now `stream_collections` need to do provider API calls, which is slow
        // We should consider streaming vcs summary from a different channel
        //
        // @brutusyhy, yes, absolutely, I've added a separate function to fetch VCS summary, so
        // we can stream VCS summary in a tauri channel on the background instead of returning it
        // as a part of the stream DTO.

        while let Some(desc) = stream.next().await {
            let event = StreamCollectionsEvent {
                id: desc.id,
                name: desc.name,
                order: desc.order,
                expanded: desc.expanded,
                branch: desc.vcs.map(|vcs| vcs.branch),
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
