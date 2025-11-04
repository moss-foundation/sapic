use moss_applib::AppRuntime;

use crate::{
    app::Window,
    models::operations::{BatchDeleteLogInput, BatchDeleteLogOutput},
};

impl<R: AppRuntime> Window<R> {
    pub async fn batch_delete_log(
        &self,
        ctx: &R::AsyncContext,
        input: &BatchDeleteLogInput,
    ) -> joinerror::Result<BatchDeleteLogOutput> {
        let output = self.log_service.delete_logs(ctx, input.ids.iter()).await?;

        Ok(BatchDeleteLogOutput {
            deleted_entries: output,
        })
    }
}
