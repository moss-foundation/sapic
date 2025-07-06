use moss_applib::context::Context;
use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::{DeleteCollectionInput, DeleteCollectionOutput},
    services::collection_service::CollectionService,
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn delete_collection<C: Context<R>>(
        &self,
        _ctx: &C,
        input: &DeleteCollectionInput,
    ) -> OperationResult<DeleteCollectionOutput> {
        let collection_service = self.services.get::<CollectionService>();
        let description = collection_service.delete_collection(input.id).await?;

        Ok(DeleteCollectionOutput {
            id: input.id,
            abs_path: description.map(|d| d.abs_path),
        })
    }
}
