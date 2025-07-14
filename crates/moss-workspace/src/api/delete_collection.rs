use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    models::operations::{DeleteCollectionInput, DeleteCollectionOutput},
    services::DynCollectionService,
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn delete_collection(
        &self,
        ctx: &R::AsyncContext,
        input: &DeleteCollectionInput,
    ) -> joinerror::Result<DeleteCollectionOutput> {
        let collection_service = self.services.get::<DynCollectionService<R>>();
        let description = collection_service.delete_collection(ctx, &input.id).await?;

        Ok(DeleteCollectionOutput {
            id: input.id.to_owned(),
            abs_path: description.map(|d| d.abs_path),
        })
    }
}
