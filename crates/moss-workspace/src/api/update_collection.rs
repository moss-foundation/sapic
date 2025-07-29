use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    models::operations::{UpdateCollectionInput, UpdateCollectionOutput},
    services::{DynCollectionService, collection_service::CollectionItemUpdateParams},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn update_collection(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateCollectionInput,
    ) -> joinerror::Result<UpdateCollectionOutput> {
        input.validate()?;
        let id = input.id.clone().into();
        let collections = self.services.get::<DynCollectionService<R>>();
        collections
            .update_collection(
                ctx,
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
