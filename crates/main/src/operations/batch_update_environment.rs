use crate::MainWindow;
use moss_applib::AppRuntime;
use sapic_ipc::{
    ValidationResultExt,
    contracts::main::environment::{BatchUpdateEnvironmentInput, BatchUpdateEnvironmentOutput},
};
use validator::Validate;

impl<R: AppRuntime> MainWindow<R> {
    pub async fn batch_update_environment(
        &self,
        ctx: &R::AsyncContext,
        input: BatchUpdateEnvironmentInput,
    ) -> joinerror::Result<BatchUpdateEnvironmentOutput> {
        let workspace = self.workspace.load();
        input.validate().join_err_bare()?;

        let mut ids = Vec::new();
        for item_params in input.items {
            let id = item_params.id.clone();
            workspace.update_environment(ctx, item_params).await?;
            ids.push(id);
        }

        Ok(BatchUpdateEnvironmentOutput { ids })
    }
}
