use moss_common::api::OperationResult;

use crate::{
    LoggingService,
    models::operations::{DeleteLogOutput, DeleteLogsInput, DeleteLogsOutput},
};

impl LoggingService {
    pub async fn delete_logs(&self, input: &DeleteLogsInput) -> OperationResult<DeleteLogsOutput> {
        let mut deleted_entries: Vec<DeleteLogOutput> = Vec::new();
        for input in &input.entries {
            deleted_entries.push(self.delete_log(input).await?);
        }
        Ok(DeleteLogsOutput { deleted_entries })
    }
}
