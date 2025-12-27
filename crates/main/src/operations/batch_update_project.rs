use moss_applib::AppRuntime;
use sapic_ipc::{
    ValidationResultExt,
    contracts::main::project::{BatchUpdateProjectInput, BatchUpdateProjectOutput},
};
use validator::Validate;

use crate::MainWindow;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn batch_update_project(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateProjectInput,
    ) -> joinerror::Result<BatchUpdateProjectOutput> {
        input.validate().join_err_bare()?;

        let mut ids = Vec::new();

        let workspace = self.workspace.load();
        for item in input.items {
            let id = item.id.clone();
            workspace.update_project(ctx, item).await?;

            ids.push(id);
        }

        Ok(BatchUpdateProjectOutput { ids })
    }
}
