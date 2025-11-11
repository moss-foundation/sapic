use chrono::NaiveDate;
use moss_applib::AppRuntime;

use crate::{
    logging::LogFilter,
    models::operations::{ListLogsInput, ListLogsOutput},
    window::Window,
};

impl<R: AppRuntime> Window<R> {
    pub async fn list_logs(
        &self,
        _ctx: &R::AsyncContext,
        input: &ListLogsInput,
    ) -> joinerror::Result<ListLogsOutput> {
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

        let contents = self.log_service.list_logs_with_filter(&filter).await?;
        Ok(ListLogsOutput { contents })
    }
}
