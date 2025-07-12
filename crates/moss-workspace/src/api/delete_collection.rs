use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    models::operations::{DeleteCollectionInput, DeleteCollectionOutput},
    services::DynCollectionService,
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn delete_collection(
        &self,
        input: &DeleteCollectionInput,
    ) -> OperationResult<DeleteCollectionOutput> {
        let collection_service = self.services.get::<DynCollectionService>();
        let description = collection_service.delete_collection(&input.id).await?;

        Ok(DeleteCollectionOutput {
            id: input.id.to_owned(),
            abs_path: description.map(|d| d.abs_path),
        })
    }
}
