use moss_common::api::OperationResult;

use crate::{
    models::{
        operations::{BatchDeleteLogInput, BatchDeleteLogOutput},
        types::LogItemSourceInfo,
    },
    services::log_service::LogService,
};

impl LogService {
    pub async fn batch_delete_log(
        &self,
        input: &BatchDeleteLogInput,
    ) -> OperationResult<BatchDeleteLogOutput> {
        let mut deleted_entries: Vec<LogItemSourceInfo> = Vec::new();
        for input in input.iter() {
            deleted_entries.push(self.delete_log(input).await?);
        }

        Ok(BatchDeleteLogOutput { deleted_entries })
    }
}
