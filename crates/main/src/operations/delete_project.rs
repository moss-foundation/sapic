use moss_applib::AppRuntime;
use sapic_ipc::contracts::main::project::{DeleteProjectInput, DeleteProjectOutput};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn delete_project(
        &self,
        ctx: &R::AsyncContext,
        input: &DeleteProjectInput,
    ) -> joinerror::Result<DeleteProjectOutput> {
        let path = self.workspace.load().delete_project(ctx, &input.id).await?;

        Ok(DeleteProjectOutput {
            id: input.id.clone(),
            abs_path: path.map(|path| path.clone().into()),
        })
    }
}
