use chrono::NaiveDate;
use moss_common::api::OperationResult;

use crate::{
    models::operations::{ListLogsInput, ListLogsOutput},
    services::log_service::{LogFilter, LogService},
};

// TODO: impl App
impl LogService {
    pub async fn list_logs(&self, input: &ListLogsInput) -> OperationResult<ListLogsOutput> {
        let filter = LogFilter {
            // Skip invalid dates
            dates: input
                .dates
                .iter()
                .filter_map(|date| NaiveDate::from_ymd_opt(date.year as i32, date.month, date.day))
                .collect(),
            levels: input.levels.iter().map(|level| (*level).into()).collect(),
            resource: input.resource.clone(),
        };

        match self.list_logs_with_filter(&filter).await {
            Ok(contents) => Ok(ListLogsOutput { contents }),
            Err(e) => Err(e.into()),
        }
    }
}
