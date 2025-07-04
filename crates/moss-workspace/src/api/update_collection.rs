use anyhow::Context as _;
use moss_applib::context::Context;
use moss_collection::collection;
use moss_common::api::{OperationError, OperationResult, OperationResultExt};
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::{UpdateCollectionInput, UpdateCollectionOutput},
    services::collection_service::{CollectionItemUpdateParams, CollectionService},
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn update_collection<C: Context<R>>(
        &mut self,
        ctx: &C,
        input: UpdateCollectionInput,
    ) -> OperationResult<UpdateCollectionOutput> {
        input.validate()?;

        let collections = self.services.get::<CollectionService>();
        collections
            .update_collection(
                input.id,
                CollectionItemUpdateParams {
                    name: input.name,
                    order: input.order,
                    expanded: input.expanded,
                    repository: input.repository,
                    icon: input.icon,
                },
            )
            .await?;

        // let collections = self
        //     .collections_mut(ctx)
        //     .await
        //     .context("Failed to get collections")?;

        // let item = collections
        //     .get(&input.id)
        //     .context("Collection not found")
        //     .map_err_as_not_found()?
        //     .clone();

        // let need_modify =
        //     input.name.is_some() || input.repository.is_some() || input.icon.is_some();

        // if need_modify {
        //     let item_lock = item.write().await;

        //     let repository = input.repository.map(|repo| match repo {
        //         ChangeRepository::Update(repo_url) => Change::Update(repo_url),
        //         ChangeRepository::Remove => Change::Remove,
        //     });

        //     let icon = input.icon.map(|icon| match icon {
        //         ChangeIcon::Update(icon_path) => Change::Update(icon_path),
        //         ChangeIcon::Remove => Change::Remove,
        //     });

        //     item_lock
        //         .modify(collection::ModifyParams {
        //             name: input.name,
        //             repository,
        //             icon,
        //         })
        //         .await
        //         .map_err(|e| OperationError::Internal(e.to_string()))?;
        // }

        Ok(UpdateCollectionOutput { id: input.id })
    }
}
