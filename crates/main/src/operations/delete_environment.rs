use crate::MainWindow;
use moss_applib::AppRuntime;
use sapic_ipc::contracts::main::environment::{DeleteEnvironmentInput, DeleteEnvironmentOutput};

impl<R: AppRuntime> MainWindow<R> {
    pub async fn delete_environment(
        &self,
        ctx: &R::AsyncContext,
        input: DeleteEnvironmentInput,
    ) -> joinerror::Result<DeleteEnvironmentOutput> {
        let workspace = self.workspace.load();

        workspace.delete_environment(ctx, &input.id).await?;

        Ok(DeleteEnvironmentOutput { id: input.id })
    }
}
