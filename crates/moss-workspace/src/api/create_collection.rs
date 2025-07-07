use crate::{
    context::AnyWorkspaceContext,
    models::operations::{CreateCollectionInput, CreateCollectionOutput},
    services::collection_service::{CollectionItemCreateParams, CollectionService},
    workspace::Workspace,
};
use moss_common::{api::OperationResult, new_nanoid};
use tauri::Runtime as TauriRuntime;
use validator::Validate;

impl<R: TauriRuntime> Workspace<R> {
    pub async fn create_collection<C: AnyWorkspaceContext<R>>(
        &self,
        _ctx: &C,
        input: &CreateCollectionInput,
    ) -> OperationResult<CreateCollectionOutput> {
        input.validate()?;

        debug_assert!(input.external_path.is_none(), "Is not implemented");

        let collection_service = self.services.get::<CollectionService>();
        let id = new_nanoid();

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
            id: id.to_string(),
            name: description.name,
            order: description.order,
            expanded: description.expanded,
            icon_path: description.icon_path,
            abs_path: description.abs_path,
            external_path: description.external_path,
        })
    }
}
