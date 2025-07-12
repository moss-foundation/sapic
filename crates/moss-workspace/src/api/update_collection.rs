use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::{UpdateCollectionInput, UpdateCollectionOutput},
    services::{DynCollectionService, collection_service::CollectionItemUpdateParams},
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn update_collection(
        &mut self,
        input: UpdateCollectionInput,
    ) -> OperationResult<UpdateCollectionOutput> {
        input.validate()?;
        let id = input.id.clone().into();
        let collections = self.services.get::<DynCollectionService>();
        collections
            .update_collection(
                &id,
                CollectionItemUpdateParams {
                    name: input.name,
                    order: input.order,
                    expanded: input.expanded,
                    repository: input.repository,
                    icon_path: input.icon_path,
                },
            )
            .await?;

        Ok(UpdateCollectionOutput { id: input.id })
    }
}
