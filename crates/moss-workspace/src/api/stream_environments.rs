use futures::StreamExt;
use moss_applib::AppRuntime;
// use futures::{StreamExt, stream};
use moss_common::api::OperationResult;
use tauri::ipc::Channel as TauriChannel;

use crate::{
    Workspace,
    models::{events::StreamEnvironmentsEvent, primitives::CollectionId},
    services::environment_service::EnvironmentService,
};

// const MAX_CONCURRENCY_LIMIT: usize = 10;

impl<R: AppRuntime> Workspace<R> {
    pub async fn stream_environments(
        &self,
        _ctx: &R::AsyncContext,
        channel: TauriChannel<StreamEnvironmentsEvent>,
    ) -> OperationResult<()> {
        let environments = self.services.get::<EnvironmentService<R>>();
        let stream = environments.list_environments().await;
        tokio::pin!(stream);

        let mut total_returned = 0;
        while let Some(environment) = stream.next().await {
            if let Err(e) = channel.send(StreamEnvironmentsEvent {
                id: environment.id,
                collection_id: environment.collection_id.map(|id| CollectionId::from(id)),
                name: environment.name,
                order: environment.order,
            }) {
                println!("Error sending environment event: {:?}", e); // TODO: log error
            } else {
                total_returned += 1;
            }
        }

        Ok(StreamEnvironmentsOutput { total_returned })

        // let collections = self.collections(ctx).await?;
        // let environments = self.environments(ctx).await?;

        // // Collect data upfront to avoid lifetime issues

        // let collections_data: Vec<_> = collections
        //     .iter()
        //     .map(|(&id, collection)| (id, collection.clone()))
        //     .collect();
        // let environments_data: Vec<_> = environments
        //     .iter()
        //     .map(|(&id, env)| (id, env.name.clone()))
        //     .collect();

        // // Create a stream from collection environments
        // let collection_environments_stream = stream::iter(collections_data)
        //     .map(|(collection_id, collection)| async move {
        //         let collection_lock = collection.read().await;
        //         let events: Vec<_> = collection_lock
        //             .list_environments()
        //             .await?
        //             .iter()
        //             .map(|e| StreamEnvironmentsEvent {
        //                 id: e.id,
        //                 collection_id: Some(collection_id),
        //                 name: e.name.clone(),
        //                 order: e.order,
        //             })
        //             .collect();

        //         anyhow::Ok(events)
        //     })
        //     .buffer_unordered(MAX_CONCURRENCY_LIMIT)
        //     .map(|result| {
        //         match result {
        //             Ok(events) => stream::iter(events),
        //             Err(err) => {
        //                 println!("failed to list environments for a collection: {:#}", err); // TODO: log this error
        //                 stream::iter(Vec::new())
        //             }
        //         }
        //     })
        //     .flatten();

        // // Create a stream from standalone environments
        // let standalone_environments_stream =
        //     stream::iter(environments_data).map(|(environment_id, name)| {
        //         StreamEnvironmentsEvent {
        //             id: environment_id,
        //             collection_id: None,
        //             name,
        //             order: None, // TODO: restore this value from cache
        //         }
        //     });

        // collection_environments_stream
        //     .chain(standalone_environments_stream)
        //     .for_each(|event| async {
        //         let _ = channel.send(event);
        //     })
        //     .await;

        // Ok(())
    }
}
