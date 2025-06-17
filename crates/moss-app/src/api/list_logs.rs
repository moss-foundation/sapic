use crate::{
    models::{
        operations::{ListLogsInput, ListLogsOutput},
        primitives::LogLevel,
    },
    services::log_service::{LogFilter, LogService},
};
use chrono::NaiveDate;
use moss_common::api::OperationResult;
use tracing::Level;

fn get_level(level: &LogLevel) -> Level {
    match level {
        LogLevel::TRACE => Level::TRACE,
        LogLevel::DEBUG => Level::DEBUG,
        LogLevel::INFO => Level::INFO,
        LogLevel::WARN => Level::WARN,
        LogLevel::ERROR => Level::ERROR,
    }
}

impl LogService {
    pub async fn list_logs(&self, input: &ListLogsInput) -> OperationResult<ListLogsOutput> {
        let filter = LogFilter {
            // Skip invalid dates
            dates: input
                .dates
                .iter()
                .filter_map(|date| NaiveDate::from_ymd_opt(date.year as i32, date.month, date.day))
                .collect(),
            levels: input.levels.iter().map(get_level).collect(),
            resource: input.resource.clone(),
        };

        match self.list_logs_with_filter(&filter).await {
            Ok(contents) => Ok(ListLogsOutput { contents }),
            Err(e) => Err(e.into()),
        }
    }
}
