use anyhow::Result;
use chrono::{NaiveDate, NaiveDateTime};
use moss_common::api::OperationResult;
use parking_lot::Mutex;
use std::{
    collections::{HashSet, VecDeque},
    ffi::OsStr,
    io::{BufRead, BufReader},
    path::Path,
    sync::Arc,
};
use tracing::Level;

use crate::{
    FILE_TIME_FORMAT, LoggingService, TIMESTAMP_FORMAT,
    models::{
        operations::{ListLogsInput, ListLogsOutput},
        types::{LogEntryInfo, LogLevel},
    },
};

// Empty field means that no filter will be applied
#[derive(Default)]
pub struct LogFilter {
    dates: HashSet<NaiveDate>,
    levels: HashSet<Level>,
    resource: Option<String>,
}

fn get_level(level: LogLevel) -> Level {
    match level {
        LogLevel::TRACE => Level::TRACE,
        LogLevel::DEBUG => Level::DEBUG,
        LogLevel::INFO => Level::INFO,
        LogLevel::WARN => Level::WARN,
        LogLevel::ERROR => Level::ERROR,
    }
}

impl From<ListLogsInput> for LogFilter {
    fn from(input: ListLogsInput) -> Self {
        Self {
            dates: input
                .dates
                .into_iter()
                .map(|date| {
                    NaiveDate::from_ymd_opt(date.year as i32, date.month, date.day).unwrap()
                })
                .collect(),
            levels: input.levels.into_iter().map(get_level).collect(),
            resource: input.resource,
        }
    }
}

impl LoggingService {
    pub async fn list_logs(&self, input: &ListLogsInput) -> OperationResult<ListLogsOutput> {
        // Combining both app and session log
        let filter: LogFilter = input.clone().into();
        let app_logs = self
            .combine_logs(&self.applog_path, &filter, self.applog_queue.clone())
            .await?;
        let session_logs = self
            .combine_logs(
                &self.sessionlog_path,
                &filter,
                self.sessionlog_queue.clone(),
            )
            .await?;
        let merged_logs = LoggingService::merge_logs_chronologically(app_logs, session_logs)
            .into_iter()
            .map(|item| item.1)
            .collect();

        Ok(ListLogsOutput {
            contents: merged_logs,
        })
    }
}

impl LoggingService {
    async fn parse_file_with_filter(
        &self,
        records: &mut Vec<(NaiveDateTime, LogEntryInfo)>,
        path: &Path,
        filter: &LogFilter,
    ) -> Result<()> {
        // In the log files, each line is a LogEntry JSON object
        // Entries in each log files are already sorted chronologically
        let file = self.fs.open_file(path).await?;

        for line in BufReader::new(file).lines() {
            let line = line?;
            let log_entry: LogEntryInfo = serde_json::from_str(&line)?;

            let level = log_entry.level.clone();
            if !filter.levels.is_empty() && !filter.levels.contains(&level.into()) {
                continue;
            }

            if let Some(resource_filter) = filter.resource.as_ref() {
                // With resource filter, skip entries without resource field
                if log_entry.resource.is_none() {
                    continue;
                }

                let resource = log_entry.resource.as_ref().unwrap();
                if resource_filter != resource {
                    continue;
                }
            }

            let timestamp = NaiveDateTime::parse_from_str(&log_entry.timestamp, TIMESTAMP_FORMAT)?;

            records.push((timestamp, log_entry));
        }
        Ok(())
    }

    async fn combine_logs(
        &self,
        path: &Path,
        filter: &LogFilter,
        queue: Arc<Mutex<VecDeque<LogEntryInfo>>>,
    ) -> Result<Vec<(NaiveDateTime, LogEntryInfo)>> {
        // Combine all log entries in a log folder according to a certain filter
        // And append the current log queue at the end
        let mut result = Vec::new();
        let mut log_files = Vec::new();
        let mut read_dir = self.fs.read_dir(path).await?;

        while let Some(entry) = read_dir.next_entry().await.unwrap_or(None) {
            let path = entry.path();
            // Skip non-log files
            if path.is_dir() || path.extension() != Some(OsStr::new("log")) {
                continue;
            }

            // Skip log files with ill-formatted names
            if let Ok(file_date) = NaiveDate::parse_from_str(
                &path.file_stem().unwrap().to_string_lossy().to_string(),
                FILE_TIME_FORMAT,
            ) {
                log_files.push((path, file_date));
            }
        }
        log_files.sort_by_key(|p| p.1);
        for (path, date_time) in &log_files {
            if filter.dates.is_empty() || filter.dates.contains(date_time) {
                self.parse_file_with_filter(&mut result, path, filter)
                    .await?
            }
        }

        result.extend({
            let lock = queue.lock();
            lock.clone().into_iter().filter_map(|entry| {
                if let Ok(datetime) =
                    NaiveDateTime::parse_from_str(&entry.timestamp, TIMESTAMP_FORMAT)
                {
                    Some((datetime, entry))
                } else {
                    // Skip entries in the queue that has invalid timestamp
                    None
                }
            })
        });

        Ok(result)
    }

    fn merge_logs_chronologically(
        a: Vec<(NaiveDateTime, LogEntryInfo)>,
        b: Vec<(NaiveDateTime, LogEntryInfo)>,
    ) -> Vec<(NaiveDateTime, LogEntryInfo)> {
        // Merging app logs and session logs, which are already separately sorted
        let mut iter_a = a.into_iter();
        let mut iter_b = b.into_iter();
        let mut merged = Vec::with_capacity(iter_a.size_hint().0 + iter_b.size_hint().0);

        let mut next_a = iter_a.next();
        let mut next_b = iter_b.next();

        while let (Some(ref a_val), Some(ref b_val)) = (next_a.as_ref(), next_b.as_ref()) {
            if a_val.0 <= b_val.0 {
                merged.push(next_a.take().unwrap());
                next_a = iter_a.next();
            } else {
                merged.push(next_b.take().unwrap());
                next_b = iter_b.next();
            }
        }

        if let Some(val) = next_a {
            merged.push(val);
            merged.extend(iter_a);
        }
        if let Some(val) = next_b {
            merged.push(val);
            merged.extend(iter_b);
        }

        merged
    }
}
