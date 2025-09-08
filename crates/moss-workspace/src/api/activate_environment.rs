use moss_applib::AppRuntime;

use crate::{
    Workspace,
    models::operations::{ActivateEnvironmentInput, ActivateEnvironmentOutput},
    services::environment_service::ActivateEnvironmentItemParams,
};

impl<R: AppRuntime> Workspace<R> {
    pub async fn activate_environment(
        &self,
        ctx: &R::AsyncContext,
        input: ActivateEnvironmentInput,
    ) -> joinerror::Result<ActivateEnvironmentOutput> {
        self.environment_service
            .activate_environment(
                ctx,
                ActivateEnvironmentItemParams {
                    environment_id: input.environment_id.clone(),
                },
            )
            .await?;

        Ok(ActivateEnvironmentOutput {
            environment_id: input.environment_id,
        })
    }
}
