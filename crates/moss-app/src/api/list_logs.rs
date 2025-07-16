use chrono::NaiveDate;
use moss_applib::AppRuntime;
use moss_common::api::OperationResult;

use crate::{
    app::App,
    models::operations::{ListLogsInput, ListLogsOutput},
    services::log_service::{LogFilter, LogService},
};

impl<R: AppRuntime> App<R> {
    pub async fn list_logs(
        &self,
        _ctx: &R::AsyncContext,
        input: &ListLogsInput,
    ) -> OperationResult<ListLogsOutput> {
        let log_service = self.services.get::<LogService<R>>();

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

        match log_service.list_logs_with_filter(&filter).await {
            Ok(contents) => Ok(ListLogsOutput { contents }),
            Err(e) => Err(e.into()),
        }
    }
}
