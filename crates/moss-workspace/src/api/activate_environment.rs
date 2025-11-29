use moss_applib::AppRuntime;

use crate::{
    Workspace,
    environment::ActivateEnvironmentItemParams,
    models::operations::{ActivateEnvironmentInput, ActivateEnvironmentOutput},
};

impl Workspace {
    pub async fn activate_environment<R: AppRuntime>(
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
