use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    app::App,
    models::operations::{BatchDeleteLogInput, BatchDeleteLogOutput},
    services::log_service::LogService,
};

impl<R: AppRuntime> App<R> {
    pub async fn batch_delete_log(
        &self,
        ctx: &R::AsyncContext,
        input: &BatchDeleteLogInput,
    ) -> OperationResult<BatchDeleteLogOutput> {
        let log_service = self.services.get::<LogService<R>>();
        match log_service.delete_logs(ctx, input.ids.iter()).await {
            Ok(output) => Ok(BatchDeleteLogOutput {
                deleted_entries: output,
            }),
            Err(e) => Err(e.into()),
        }
    }
}
