use moss_applib::AppRuntime;
use sapic_base::environment::{PredefinedEnvironment, types::primitives::EnvironmentId};
use sapic_ipc::{
    ValidationResultExt,
    contracts::main::{
        OpenInTarget,
        workspace::{CreateWorkspaceInput, CreateWorkspaceOutput},
    },
};
use sapic_system::environment::environment_service::CreateEnvironmentItemParams;
use std::cell::LazyCell;
use validator::Validate;

use crate::MainWindow;

// FIXME: Where should I put this?
const PREDEFINED_ENVIRONMENTS: LazyCell<Vec<PredefinedEnvironment>> = LazyCell::new(|| {
    vec![PredefinedEnvironment {
        name: "Globals".to_string(),
        color: Some("#3574F0".to_string()),
    }]
});

impl<R: AppRuntime> MainWindow<R> {
    pub async fn create_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &CreateWorkspaceInput,
    ) -> joinerror::Result<CreateWorkspaceOutput> {
        input.validate().join_err_bare()?;

        let output = self.workspace_ops.create(ctx, input.name.clone()).await?;

        for env in PREDEFINED_ENVIRONMENTS.iter() {
            self.environment_ops
                .create_workspace_environment(
                    ctx,
                    &output.id,
                    CreateEnvironmentItemParams {
                        env_id: EnvironmentId::new(),
                        project_id: None,
                        name: env.name.clone(),
                        order: 0,
                        color: env.color.clone(),
                        variables: vec![],
                    },
                )
                .await?;
        }

        Ok(CreateWorkspaceOutput {
            id: output.id,
            will_replace: matches!(input.open_on_creation, OpenInTarget::CurrentWindow),
            abs_path: output.abs_path,
        })
    }
}
