use anyhow::Context as _;
use moss_collection::collection::{self};
use moss_common::api::{OperationError, OperationResult, OperationResultExt};

use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::{UpdateCollectionEntryInput, UpdateCollectionEntryOutput},
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn update_collection(
        &self,
        input: UpdateCollectionEntryInput,
    ) -> OperationResult<UpdateCollectionEntryOutput> {
        input.validate()?;

        let collections = self
            .collections()
            .await
            .context("Failed to get collections")?;

        let collections_lock = collections.write().await;
        let item = collections_lock
            .get(&input.id)
            .context("Collection not found")
            .map_err_as_not_found()?
            .clone();

        if let Some(new_name) = input.new_name {
            let mut item_lock = item.write().await;
            item_lock
                .modify(collection::ModifyParams {
                    name: Some(new_name.clone()),
                })
                .await
                .map_err(|e| OperationError::Internal(e.to_string()))?;

            item_lock.name = new_name;
        }

        Ok(UpdateCollectionEntryOutput { id: input.id })
    }
}
