use moss_common::api::OperationResult;
use tauri::{Runtime as TauriRuntime, ipc::Channel};

use crate::{Workspace, models::events::ListEnvironmentsEvent};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn list_environments(
        &self,
        channel: Channel<ListEnvironmentsEvent>,
    ) -> OperationResult<()> {
        let collections = self.collections().await?;
        for (&collection_id, collection) in collections.read().await.iter() {
            let collection_lock = collection.read().await;
            for (&environment_id, environment) in
                collection_lock.environments().await?.read().await.iter()
            {
                let _ = channel.send(ListEnvironmentsEvent {
                    id: environment_id,
                    collection_id: Some(collection_id),
                    name: environment.name.clone(),
                    order: environment.order,
                });
            }
        }

        let environments = self.environments().await?;
        Ok(())
    }
}
