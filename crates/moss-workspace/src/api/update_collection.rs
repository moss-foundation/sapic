use anyhow::Context as _;
use moss_applib::context::Context;
use moss_collection::collection;
use moss_common::api::{OperationError, OperationResult, OperationResultExt};

use crate::{
    models::operations::{UpdateCollectionInput, UpdateCollectionOutput},
    workspace::Workspace,
};

use crate::models::operations::ChangeInput;
use moss_collection::collection::Change;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

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

        if input.new_name.is_some() || input.new_repo.is_some() || input.new_icon.is_some() {
            let item_lock = item.write().await;
            item_lock
                .modify(collection::ModifyParams {
                    name: input.new_name,
                    repo: match input.new_repo {
                        None => None,
                        Some(ChangeInput::Update(repo_url)) => Some(Change::Update(repo_url)),
                        Some(ChangeInput::Remove) => Some(Change::Remove),
                    },
                    icon: match input.new_icon {
                        None => None,
                        Some(ChangeInput::Update(icon_path)) => Some(Change::Update(icon_path)),
                        Some(ChangeInput::Remove) => Some(Change::Remove),
                    },
                })
                .await
                .map_err(|e| OperationError::Internal(e.to_string()))?;
        }

        Ok(UpdateCollectionOutput { id: input.id })
    }
}
