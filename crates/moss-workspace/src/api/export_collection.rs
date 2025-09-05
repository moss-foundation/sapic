use crate::{
    Workspace,
    models::operations::{ExportCollectionInput, ExportCollectionOutput},
};
use moss_applib::{AppRuntime, errors::ValidationResultExt};
use validator::Validate;

impl<R: AppRuntime> Workspace<R> {
    pub async fn export_collection(
        &self,
        _ctx: &R::AsyncContext,
        input: &ExportCollectionInput,
    ) -> joinerror::Result<ExportCollectionOutput> {
        input.validate().join_err_bare()?;
        let id = input.inner.id.clone();

        let archive_path = self
            .collection_service
            .export_collection(&id, &input.inner)
            .await?;

        Ok(ExportCollectionOutput { archive_path })
    }
}
