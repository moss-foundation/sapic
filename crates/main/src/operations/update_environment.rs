use crate::MainWindow;
use moss_applib::AppRuntime;
use sapic_ipc::{
    ValidationResultExt,
    contracts::main::environment::{UpdateEnvironmentInput, UpdateEnvironmentOutput},
};
use validator::Validate;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn update_environment(
        &self,
        ctx: &R::AsyncContext,
        input: UpdateEnvironmentInput,
    ) -> joinerror::Result<UpdateEnvironmentOutput> {
        input.validate().join_err_bare()?;

        let workspace = self.workspace.load();

        let id = input.inner.id.clone();
        workspace.update_environment(ctx, input.inner).await?;

        Ok(UpdateEnvironmentOutput { id })
    }
}
