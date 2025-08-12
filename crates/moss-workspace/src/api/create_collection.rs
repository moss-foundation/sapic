use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    models::{
        operations::{CreateCollectionInput, CreateCollectionOutput},
        primitives::CollectionId,
    },
    services::collection_service::CollectionItemCreateParams,
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn create_collection(
        &self,
        ctx: &R::AsyncContext,
        input: &CreateCollectionInput,
    ) -> joinerror::Result<CreateCollectionOutput> {
        input.validate().join_err_bare()?;

        debug_assert!(input.external_path.is_none(), "Is not implemented");

        let id = CollectionId::new();

        let description = self
            .collection_service
            .create_collection(
                ctx,
                &id,
                CollectionItemCreateParams {
                    name: input.name.to_owned(),
                    order: input.order.to_owned(),
                    external_path: input.external_path.to_owned(),
                    icon_path: input.icon_path.to_owned(),
                    repository: input.repository.to_owned(),
                },
            )
            .await?;

        Ok(CreateCollectionOutput {
            id,
            name: description.name,
            order: description.order,
            expanded: description.expanded,
            icon_path: description.icon_path,
            abs_path: description.abs_path,
            external_path: description.external_path,
        })
    }
}
