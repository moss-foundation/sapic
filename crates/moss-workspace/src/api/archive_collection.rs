use moss_applib::AppRuntime;

use crate::{
    Workspace,
    models::operations::{ArchiveCollectionInput, ArchiveCollectionOutput},
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn archive_collection(
        &self,
        ctx: &R::AsyncContext,
        input: ArchiveCollectionInput,
    ) -> joinerror::Result<ArchiveCollectionOutput> {
        let abs_path = self
            .collection_service
            .archive_collection(ctx, &input.id)
            .await?;

        Ok(ArchiveCollectionOutput {
            id: input.id,
            abs_path: abs_path.into(),
        })
    }
}
