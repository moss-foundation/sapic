use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    models::operations::{CreateEnvironmentInput, CreateEnvironmentOutput},
    services::environment_service::{CreateEnvironmentItemParams, EnvironmentService},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn create_environment(
        &self,
        ctx: &R::AsyncContext,
        input: CreateEnvironmentInput,
    ) -> joinerror::Result<CreateEnvironmentOutput> {
        input.validate()?;

        let environments = self.services.get::<EnvironmentService<R>>();
        let result = environments
            .create_environment(
                ctx,
                CreateEnvironmentItemParams {
                    collection_id: input.collection_id,
                    name: input.name.clone(),
                    order: input.order,
                    color: input.color.clone(),
                },
            )
            .await?;

        Ok(CreateEnvironmentOutput {
            id: result.id,
            collection_id: result.collection_id,
            name: result.display_name,
            order: result.order,
            color: result.color,
            expanded: result.expanded,
            abs_path: result.abs_path,
        })
    }
}
