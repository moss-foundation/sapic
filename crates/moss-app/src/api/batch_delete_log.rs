use moss_common::api::OperationResult;

use crate::{
    models::operations::{BatchDeleteLogInput, BatchDeleteLogOutput},
    services::log_service::LogService,
};

// TODO: impl App
impl LogService {
    pub async fn batch_delete_log(
        &self,
        input: &BatchDeleteLogInput,
    ) -> OperationResult<BatchDeleteLogOutput> {
        match self.delete_logs(input.0.iter().collect()).await {
            Ok(output) => Ok(BatchDeleteLogOutput {
                deleted_entries: output,
            }),
            Err(e) => Err(e.into()),
        }
    }
}
