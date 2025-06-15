use std::sync::atomic::Ordering;

use anyhow::Context as _;
use moss_applib::context::Context;
use moss_collection::collection::{self};
use moss_common::api::{OperationError, OperationResult, OperationResultExt};

use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::{UpdateCollectionInput, UpdateCollectionOutput},
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn update_collection<C: Context<R>>(
        &mut self,
        ctx: &C,
        input: UpdateCollectionInput,
    ) -> OperationResult<UpdateCollectionOutput> {
        input.validate()?;

        let collections = self
            .collections_mut(ctx)
            .await
            .context("Failed to get collections")?;

        let item = collections
            .get(&input.id)
            .context("Collection not found")
            .map_err_as_not_found()?
            .clone();

        if let Some(new_name) = input.new_name {
            item.modify(collection::ModifyParams {
                name: Some(new_name.clone()),
            })
            .await
            .map_err(|e| OperationError::Internal(e.to_string()))?;
        }

        if let (Some(new_order), Some(order_atomic)) = (input.order, &item.order) {
            order_atomic.store(new_order, Ordering::Relaxed);
        }

        Ok(UpdateCollectionOutput { id: input.id })
    }
}
