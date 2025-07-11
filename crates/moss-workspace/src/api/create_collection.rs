use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    context::AnyWorkspaceContext,
    models::{
        operations::{CreateCollectionInput, CreateCollectionOutput},
        primitives::CollectionId,
    },
    services::{DynCollectionService, collection_service::CollectionItemCreateParams},
    workspace::Workspace,
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn create_collection<C: AnyWorkspaceContext<R>>(
        &self,
        _ctx: &C,
        input: &CreateCollectionInput,
    ) -> OperationResult<CreateCollectionOutput> {
        input.validate()?;

        debug_assert!(input.external_path.is_none(), "Is not implemented");

        let collection_service = self.services.get::<DynCollectionService>();
        let id = CollectionId::new();

        let description = collection_service
            .create_collection(
                &id,
                CollectionItemCreateParams {
                    name: input.name.to_owned(),
                    order: input.order.to_owned(),
                    repository: input.repo.to_owned(),
                    external_path: input.external_path.to_owned(),
                    icon_path: input.icon_path.to_owned(),
                },
            )
            .await?;

        Ok(CreateCollectionOutput {
            id: id,
            name: description.name,
            order: description.order,
            expanded: description.expanded,
            icon_path: description.icon_path,
            abs_path: description.abs_path,
            external_path: description.external_path,
        })
    }
}
