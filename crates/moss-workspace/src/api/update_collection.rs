use moss_applib::context::Context;
use moss_common::api::OperationResult;
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
        _ctx: &C,
        input: UpdateCollectionInput,
    ) -> OperationResult<UpdateCollectionOutput> {
        input.validate()?;
        let id = input.id.clone().into();
        let collections = self.services.get::<CollectionService>();
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
