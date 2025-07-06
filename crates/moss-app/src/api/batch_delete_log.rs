use moss_common::api::OperationResult;
use tauri::Runtime as TauriRuntime;

use crate::{
    app::App,
    models::operations::{BatchDeleteLogInput, BatchDeleteLogOutput},
    services::log_service::LogService,
};

impl<R: TauriRuntime> App<R> {
    pub async fn batch_delete_log(
        &self,
        input: &BatchDeleteLogInput,
    ) -> OperationResult<BatchDeleteLogOutput> {
        let log_service = self.services.get::<LogService>();
        match log_service
            .delete_logs(input.0.iter().map(|s| s.as_str()))
            .await
        {
            Ok(output) => Ok(BatchDeleteLogOutput {
                deleted_entries: output,
            }),
            Err(e) => Err(e.into()),
        }
    }
}
