use chrono::{DateTime, FixedOffset, NaiveDate};
use serde_json::Value as JsonValue;
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    str::FromStr,
};
use tracing::Level;

use crate::{
    FILE_DATE_FORMAT, LoggingService, TIMESTAMP_FORMAT,
    constants::{LEVEL_LIT, RESOURCE_LIT},
    models::{
        operations::{ListLogsInput, ListLogsOutput},
        types::{LogEntry, LogLevel},
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
    pub fn list_logs(&self, input: &ListLogsInput) -> anyhow::Result<ListLogsOutput> {
        // Combining both app and session log
        let filter: LogFilter = input.clone().into();
        let app_logs = self.combine_logs(&self.applog_path, &filter)?;
        let session_logs = self.combine_logs(&self.sessionlog_path, &filter)?;
        let merged_logs = LoggingService::merge_logs_chronologically(app_logs, session_logs);

        let log_entries: Vec<LogEntry> = merged_logs
            .into_iter()
            .map(|(_dt, value)| serde_json::from_value(value))
            .collect::<anyhow::Result<Vec<_>, _>>()?;
        Ok(ListLogsOutput {
            contents: log_entries,
        })
    }
}

impl LoggingService {
    fn parse_file_with_filter(
        records: &mut Vec<(DateTime<FixedOffset>, JsonValue)>,
        path: &Path,
        filter: &LogFilter,
    ) -> anyhow::Result<()> {
        // In the log files, each line is a LogEntry JSON object
        // Entries in each log files are already sorted chronologically
        let file = File::open(path)?;

        for line in BufReader::new(file).lines() {
            let line = line?;
            let value: JsonValue = serde_json::from_str(&line)?;

            if !filter.levels.is_empty() {
                let level = Level::from_str(value.get(LEVEL_LIT).unwrap().as_str().unwrap())?;
                if !filter.levels.contains(&level) {
                    continue;
                }
            }

            if let Some(resource_filter) = filter.resource.as_ref() {
                if let Some(resource) = value.get(RESOURCE_LIT).and_then(|v| v.as_str()) {
                    if resource_filter != resource {
                        continue;
                    }
                } else {
                    // With resource filter, skip entries without resource field
                }
            }

            let timestamp = value
                .get("timestamp")
                .and_then(|v| v.as_str())
                .map(|ts| DateTime::<FixedOffset>::parse_from_str(ts, TIMESTAMP_FORMAT))
                .unwrap()?;

            records.push((timestamp, value));
        }
        Ok(())
    }

    fn combine_logs(
        &self,
        path: &Path,
        filter: &LogFilter,
    ) -> anyhow::Result<Vec<(DateTime<FixedOffset>, JsonValue)>> {
        // Combine all log entries in app/session log path according to a certain filter
        let mut result = Vec::new();
        let mut log_files = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() || path.extension() != Some(OsStr::new("log")) {
                continue;
            }

            let file_date = NaiveDate::parse_from_str(
                &path.file_stem().unwrap().to_string_lossy().to_string(),
                FILE_DATE_FORMAT,
            )?;

            log_files.push((path, file_date));
        }
        log_files.sort_by_key(|p| p.1);
        for (path, date_time) in &log_files {
            if filter.dates.is_empty() || filter.dates.contains(date_time) {
                LoggingService::parse_file_with_filter(&mut result, path, filter)?
            }
        }

        Ok(result)
    }

    fn merge_logs_chronologically(
        a: Vec<(DateTime<FixedOffset>, JsonValue)>,
        b: Vec<(DateTime<FixedOffset>, JsonValue)>,
    ) -> Vec<(DateTime<FixedOffset>, JsonValue)> {
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
