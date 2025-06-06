use crate::{LoggingService, models::operations::DeleteLogsInput};
use moss_common::api::OperationResult;

impl LoggingService {
    pub fn delete_logs(&self, input: &DeleteLogsInput) -> OperationResult<()> {
        for input in &input.inputs {
            self.delete_log(input)?;
        }
        Ok(())
    }
}
