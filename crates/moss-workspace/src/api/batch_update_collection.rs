use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    api::BatchUpdateCollectionOp,
    models::operations::{BatchUpdateCollectionInput, BatchUpdateCollectionOutput},
    services::{AnyCollectionService, collection_service::CollectionItemUpdateParams},
    workspace::Workspace,
};

impl<R: AppRuntime> BatchUpdateCollectionOp<R> for Workspace<R> {
    async fn batch_update_collection(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateCollectionInput,
    ) -> joinerror::Result<BatchUpdateCollectionOutput> {
        input.validate()?;

        let mut ids = Vec::new();
        for item in input.items {
            self.collection_service
                .update_collection(
                    ctx,
                    &item.id,
                    CollectionItemUpdateParams {
                        order: item.order,
                        expanded: item.expanded,
                        name: None,
                        repository: None,
                        icon_path: None,
                    },
                )
                .await?;

            ids.push(item.id);
        }

        Ok(BatchUpdateCollectionOutput { ids })
    }
}
