use anyhow::Result;

use crate::{LoggingService, models::operations::DeleteLogsInput};

impl LoggingService {
    pub fn delete_logs(&self, input: &DeleteLogsInput) -> Result<()> {
        for input in &input.inputs {
            self.delete_log(input)?;
        }
        Ok(())
    }
}
