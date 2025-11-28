use moss_applib::{AppRuntime, errors::ValidationResultExt};
use sapic_ipc::contracts::main::{
    OpenInTarget,
    workspace::{CreateWorkspaceInput, CreateWorkspaceOutput},
};
use validator::Validate;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn create_workspace(
        &self,
        _ctx: &R::AsyncContext,
        input: &CreateWorkspaceInput,
    ) -> joinerror::Result<CreateWorkspaceOutput> {
        input.validate().join_err_bare()?;

        let output = self.workspace_ops.create(input.name.clone()).await?;

        Ok(CreateWorkspaceOutput {
            id: output.id,
            will_replace: matches!(input.open_on_creation, OpenInTarget::CurrentWindow),
            abs_path: output.abs_path,
        })
    }
}
