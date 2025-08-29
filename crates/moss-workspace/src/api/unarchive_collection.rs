use moss_applib::AppRuntime;

use crate::{
    Workspace,
    models::operations::{UnarchiveCollectionInput, UnarchiveCollectionOutput},
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn unarchive_collection(
        &self,
        ctx: &R::AsyncContext,
        input: UnarchiveCollectionInput,
    ) -> joinerror::Result<UnarchiveCollectionOutput> {
        let abs_path = self
            .collection_service
            .unarchive_collection(ctx, &input.id)
            .await?;

        Ok(UnarchiveCollectionOutput {
            id: input.id,
            abs_path: abs_path.into(),
        })
    }
}
