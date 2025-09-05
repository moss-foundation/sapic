use moss_applib::AppRuntime;

use crate::{
    Workspace,
    models::operations::{ExportCollectionInput, ExportCollectionOutput},
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn export_collection(
        &self,
        _ctx: &R::AsyncContext,
        input: &ExportCollectionInput,
    ) -> joinerror::Result<ExportCollectionOutput> {
        let id = input.inner.id.clone();

        self.collection_service
            .export_collection(&id, &input.inner)
            .await?;

        Ok(ExportCollectionOutput {})
    }
}
