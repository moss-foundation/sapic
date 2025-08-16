use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    models::operations::{UpdateCollectionInput, UpdateCollectionOutput},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn update_collection(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateCollectionInput,
    ) -> joinerror::Result<UpdateCollectionOutput> {
        input.validate().join_err_bare()?;

        let id = input.inner.id.clone().into();
        self.collection_service
            .update_collection(ctx, &id, input.inner)
            .await?;

        Ok(UpdateCollectionOutput { id })
    }
}
