use moss_applib::{AppRuntime, errors::ValidationResultExt};
use sapic_ipc::contracts::welcome::workspace::{CreateWorkspaceInput, CreateWorkspaceOutput};
use validator::Validate;

use crate::WelcomeWindow;

impl<R: AppRuntime> WelcomeWindow<R> {
    pub async fn create_workspace(
        &self,
        _ctx: &R::AsyncContext,
        input: &CreateWorkspaceInput,
    ) -> joinerror::Result<CreateWorkspaceOutput> {
        input.validate().join_err_bare()?;

        let output = self
            .workspace_ops
            .create_workspace(input.name.clone())
            .await?;

        Ok(CreateWorkspaceOutput {
            id: output.id,
            abs_path: output.abs_path,
        })
    }
}
