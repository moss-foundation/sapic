use moss_applib::{AppRuntime, errors::ValidationResultExt};
use validator::Validate;

use crate::{
    models::{
        operations::{CreateCollectionInput, CreateCollectionOutput},
        primitives::CollectionId,
    },
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn create_collection(
        &self,
        ctx: &R::AsyncContext,
        input: &CreateCollectionInput,
    ) -> joinerror::Result<CreateCollectionOutput> {
        input.validate().join_err_bare()?;

        debug_assert!(input.inner.external_path.is_none(), "Is not implemented");

        let id = CollectionId::new();

        let account = if input.inner.git_params.is_some() {
            self.active_profile.first().await
        } else {
            None
        };
        let description = self
            .collection_service
            .create_collection(ctx, &id, account, &input.inner)
            .await?;

        Ok(CreateCollectionOutput {
            id: description.id,
            name: description.name,
            order: description.order,
            expanded: description.expanded,
            icon_path: description.icon_path,
            abs_path: description.abs_path,
            external_path: description.external_path,
        })
    }
}
