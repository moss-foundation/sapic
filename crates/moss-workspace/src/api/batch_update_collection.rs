use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    api::BatchUpdateCollectionOp,
    models::operations::{BatchUpdateCollectionInput, BatchUpdateCollectionOutput},
    workspace::Workspace,
};

impl<R: AppRuntime> BatchUpdateCollectionOp<R> for Workspace<R> {
    async fn batch_update_collection(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateCollectionInput,
    ) -> joinerror::Result<BatchUpdateCollectionOutput> {
        input.validate().join_err_bare()?;

        let mut ids = Vec::new();
        for item in input.items {
            let id = item.id.clone();
            self.collection_service
                .update_collection(ctx, &id, item)
                .await?;

            ids.push(id);
        }

        Ok(BatchUpdateCollectionOutput { ids })
    }
}
