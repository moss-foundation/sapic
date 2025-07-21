use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    models::operations::{BatchUpdateCollectionInput, BatchUpdateCollectionOutput},
    services::{DynCollectionService, collection_service::CollectionItemUpdateParams},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn batch_update_collection(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateCollectionInput,
    ) -> joinerror::Result<BatchUpdateCollectionOutput> {
        input.validate()?;
        let collections = self.services.get::<DynCollectionService<R>>();

        let mut ids = Vec::new();
        for item in input.items {
            collections
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
