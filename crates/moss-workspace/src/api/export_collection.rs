use moss_applib::AppRuntime;

use crate::{
    Workspace,
    models::operations::{ExportCollectionInput, ExportCollectionOutput},
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn export_collection(
        &self,
        ctx: &R::AsyncContext,
        input: &ExportCollectionInput,
    ) -> joinerror::Result<ExportCollectionOutput> {
        let id = input.inner.id.clone();

        self.collection_service
            .export_collection(ctx, &id, &input.inner)
            .await?;

        Ok(ExportCollectionOutput {})
    }
}
