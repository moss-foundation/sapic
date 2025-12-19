use moss_applib::AppRuntime;
use sapic_ipc::contracts::main::project::{UnarchiveProjectInput, UnarchiveProjectOutput};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn unarchive_project(
        &self,
        ctx: &R::AsyncContext,
        input: UnarchiveProjectInput,
    ) -> joinerror::Result<UnarchiveProjectOutput> {
        let id = input.id;
        self.workspace.load().unarchive_project(ctx, &id).await?;

        Ok(UnarchiveProjectOutput { id })
    }
}
