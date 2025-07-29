use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    models::{
        operations::{CreateCollectionInput, CreateCollectionOutput},
        primitives::CollectionId,
    },
    services::{DynCollectionService, collection_service::CollectionItemCreateParams},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn create_collection(
        &self,
        ctx: &R::AsyncContext,
        input: &CreateCollectionInput,
    ) -> joinerror::Result<CreateCollectionOutput> {
        input.validate()?;

        debug_assert!(input.external_path.is_none(), "Is not implemented");

        let collection_service = self.services.get::<DynCollectionService<R>>();
        let id = CollectionId::new();

        let description = collection_service
            .create_collection(
                ctx,
                &id,
                CollectionItemCreateParams {
                    name: input.name.to_owned(),
                    order: input.order.to_owned(),
                    repository: input.repository.to_owned(),
                    external_path: input.external_path.to_owned(),
                    icon_path: input.icon_path.to_owned(),
                },
            )
            .await?;

        Ok(CreateCollectionOutput {
            id: id,
            name: description.name,
            order: description.order,
            expanded: description.expanded,
            icon_path: description.icon_path,
            abs_path: description.abs_path,
            external_path: description.external_path,
        })
    }
}
