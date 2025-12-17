use moss_applib::AppRuntime;
use sapic_ipc::{
    ValidationResultExt,
    contracts::main::{
        OpenInTarget,
        workspace::{CreateWorkspaceInput, CreateWorkspaceOutput},
    },
};
use validator::Validate;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn create_workspace(
        &self,
        ctx: &R::AsyncContext,
        input: &CreateWorkspaceInput,
    ) -> joinerror::Result<CreateWorkspaceOutput> {
        input.validate().join_err_bare()?;

        let output = self.workspace_ops.create(ctx, input.name.clone()).await?;

        Ok(CreateWorkspaceOutput {
            id: output.id,
            will_replace: matches!(input.open_on_creation, OpenInTarget::CurrentWindow),
            abs_path: output.abs_path,
        })
    }
}
