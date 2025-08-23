use moss_api::ext::ValidationResultExt;
use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    models::operations::{CreateEnvironmentInput, CreateEnvironmentOutput},
    services::environment_service::CreateEnvironmentItemParams,
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn create_environment(
        &self,
        ctx: &R::AsyncContext,
        input: CreateEnvironmentInput,
    ) -> joinerror::Result<CreateEnvironmentOutput> {
        input.validate().join_err_bare()?;

        let result = self
            .environment_service
            .create_environment(
                ctx,
                CreateEnvironmentItemParams {
                    collection_id: input.collection_id,
                    name: input.name.clone(),
                    order: input.order,
                    color: input.color.clone(),
                    variables: input.variables,
                },
            )
            .await?;

        Ok(CreateEnvironmentOutput {
            id: result.id,
            collection_id: result.collection_id.map(|id| id.into()),
            name: result.display_name,
            order: result.order,
            color: result.color,
            abs_path: result.abs_path.to_path_buf(),
        })
    }
}
