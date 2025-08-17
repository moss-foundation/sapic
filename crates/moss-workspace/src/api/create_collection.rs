use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    models::operations::{CreateCollectionInput, CreateCollectionOutput},
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

        let description = self
            .collection_service
            .create_collection(ctx, &input.inner)
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
