use moss_applib::AppRuntime;
use validator::Validate;

use crate::{
    models::operations::{UpdateEnvironmentInput, UpdateEnvironmentOutput},
    services::environment_service::{EnvironmentService, UpdateEnvironmentItemParams},
    workspace::Workspace,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn update_environment(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateEnvironmentInput,
    ) -> joinerror::Result<UpdateEnvironmentOutput> {
        input.validate()?;

        let environments = self.services.get::<EnvironmentService<R>>();
        environments
            .update_environment(
                ctx,
                &input.id,
                UpdateEnvironmentItemParams {
                    name: input.name,
                    expanded: input.expanded,
                    order: input.order,
                    color: input.color,
                    vars_to_add: input.vars_to_add,
                    vars_to_update: input.vars_to_update,
                    vars_to_delete: input.vars_to_delete,
                },
            )
            .await?;

        Ok(UpdateEnvironmentOutput { id: input.id })
    }
}
