use crate::{
    models::operations::{DeleteCollectionInput, DeleteCollectionOutput},
    services::collection_service::CollectionService,
    workspace::Workspace,
};
use moss_applib::context::Context;
use moss_common::{NanoId, api::OperationResult};
use tauri::Runtime as TauriRuntime;

impl<R: TauriRuntime> Workspace<R> {
    pub async fn delete_collection<C: Context<R>>(
        &self,
        _ctx: &C,
        input: &DeleteCollectionInput,
    ) -> OperationResult<DeleteCollectionOutput> {
        let collection_service = self.services.get::<CollectionService>();
        let id: NanoId = input.id.clone().into();
        let description = collection_service.delete_collection(&id).await?;

        Ok(DeleteCollectionOutput {
            id: id.to_string(),
            abs_path: description.map(|d| d.abs_path),
        })
    }
}
