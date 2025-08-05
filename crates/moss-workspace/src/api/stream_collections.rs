use futures::StreamExt;
use moss_applib::AppRuntime;
use moss_common::api::OperationResult;
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
    ) -> OperationResult<StreamCollectionsOutput> {
        let stream = self
            .collection_service
            .list_collections(ctx, self.github_client.clone(), self.gitlab_client.clone())
            .await;
        tokio::pin!(stream);

        let mut total_returned = 0;
        while let Some(collection) = stream.next().await {
            if let Err(e) = channel.send(StreamCollectionsEvent {
                id: collection.id,
                name: collection.name,
                order: collection.order,
                expanded: collection.expanded,
                repository: collection.repository,
                repository_info: collection.repository_info,
                contributors: collection.contributors,
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
