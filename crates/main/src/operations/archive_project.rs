use moss_applib::AppRuntime;
use sapic_ipc::contracts::main::project::{ArchiveProjectInput, ArchiveProjectOutput};

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn archive_project(
        &self,
        ctx: &R::AsyncContext,
        input: ArchiveProjectInput,
    ) -> joinerror::Result<ArchiveProjectOutput> {
        let id = input.id;
        self.workspace.load().archive_project(ctx, &id).await?;

        Ok(ArchiveProjectOutput { id })
    }
}
