use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    Workspace,
    models::{
        operations::{CloneCollectionInput, CloneCollectionOutput},
        primitives::CollectionId,
    },
    services::collection_service::CollectionItemCloneParams,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn clone_collection(
        &self,
        ctx: &R::AsyncContext,
        input: &CloneCollectionInput,
    ) -> joinerror::Result<CloneCollectionOutput> {
        input.validate().join_err_bare()?;

        let id = CollectionId::new();

        let description = self
            .collection_service
            .clone_collection(
                ctx,
                &id,
                CollectionItemCloneParams {
                    order: input.order,
                    repository: input.repository.clone(),
                },
            )
            .await?;

        Ok(CloneCollectionOutput {
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
