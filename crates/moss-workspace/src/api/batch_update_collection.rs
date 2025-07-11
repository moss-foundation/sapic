use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::{BatchUpdateCollectionInput, BatchUpdateCollectionOutput},
    services::{DynCollectionService, collection_service::CollectionItemUpdateParams},
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn batch_update_collection(
        &self,
        input: BatchUpdateCollectionInput,
    ) -> OperationResult<BatchUpdateCollectionOutput> {
        input.validate()?;
        let collections = self.services.get::<DynCollectionService>();

        let mut ids = Vec::new();
        for item in input.items {
            collections
                .update_collection(
                    &item.id,
                    CollectionItemUpdateParams {
                        order: item.order,
                        expanded: item.expanded,
                        name: None,
                        repository: None,
                        icon_path: None,
                    },
                )
                .await?;

            ids.push(item.id);
        }

        Ok(BatchUpdateCollectionOutput { ids })
    }
}
