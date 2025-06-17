mod rollinglog_writer;
mod taurilog_writer;

use anyhow::Result;
use chrono::{DateTime, Days, FixedOffset, Local, NaiveDate, NaiveDateTime, TimeZone};
use moss_applib::Service;
use moss_common::api::{OperationError, OperationResult};
use moss_fs::{CreateOptions, FileSystem};
use nanoid::nanoid;
use parking_lot::Mutex;
use std::{
    collections::{HashSet, VecDeque},
    ffi::OsStr,
    fs,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use thiserror::Error;
use tracing::{Level, debug, error, info, trace, warn};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    filter::filter_fn,
    fmt::{
        format::{FmtSpan, JsonFields},
        time::ChronoLocal,
    },
    prelude::*,
};
use uuid::Uuid;

use crate::{
    models::types::{LogEntryInfo, LogEntryRef, LogItemSourceInfo},
    services::log_service::{
        constants::*, rollinglog_writer::RollingLogWriter, taurilog_writer::TauriLogWriter,
    },
};

pub mod constants {
    pub const APP_SCOPE: &'static str = "app";
    pub const SESSION_SCOPE: &'static str = "session";

    pub const ID_LENGTH: usize = 10;

    pub const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3f%z";

    pub const FILE_TIMESTAMP_FORMAT: &'static str = "%Y_%m_%dT%H_%M_%S%z";
}

fn new_id() -> String {
    nanoid!(ID_LENGTH)
}

const DUMP_THRESHOLD: usize = 10;

#[derive(Error, Debug)]
pub enum LogServiceError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("log entry with id {id} is not found")]
    NotFound { id: String },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl Into<OperationError> for LogServiceError {
    fn into(self) -> OperationError {
        match self {
            LogServiceError::InvalidInput(_) => OperationError::InvalidInput(self.to_string()),
            LogServiceError::NotFound { .. } => OperationError::NotFound(self.to_string()),
            LogServiceError::Unknown(e) => OperationError::Unknown(e),
        }
    }
}

pub type LogServiceResult<T> = std::result::Result<T, LogServiceError>;

pub struct LogPayload {
    pub resource: Option<String>,
    pub message: String,
}

pub enum LogScope {
    App,
    Session,
}

// Empty field means that no filter will be applied
#[derive(Default)]
pub struct LogFilter {
    pub dates: HashSet<NaiveDate>,
    pub levels: HashSet<Level>,
    pub resource: Option<String>,
}

impl LogFilter {
    pub fn check_entry(&self, log_entry: &LogEntryInfo) -> Result<bool> {
        let date = NaiveDate::parse_from_str(&log_entry.timestamp, TIMESTAMP_FORMAT)?;
        if !self.dates.is_empty() && !self.dates.contains(&date) {
            return Ok(false);
        }

        let level = log_entry.level.clone();
        if !self.levels.is_empty() && !self.levels.contains(&level.into()) {
            return Ok(false);
        }

        if let Some(resource_filter) = self.resource.as_ref() {
            // With resource filter, skip entries without resource field
            if log_entry.resource.is_none() {
                return Ok(false);
            }

            let resource = log_entry.resource.as_ref().unwrap();
            if resource_filter != resource {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

/// Logs structure
/// App Log Path: logs\
/// Session Log Path: {App Log Path}\sessions\{session_id}\
pub struct LogService {
    fs: Arc<dyn FileSystem>,
    applog_path: PathBuf,
    sessionlog_path: PathBuf,
    applog_queue: Arc<Mutex<VecDeque<LogEntryInfo>>>,
    sessionlog_queue: Arc<Mutex<VecDeque<LogEntryInfo>>>,
    _applog_writerguard: WorkerGuard,
    _sessionlog_writerguard: WorkerGuard,
    _taurilog_writerguard: WorkerGuard,
}

impl Service for LogService {}

impl LogService {
    pub fn new<R: TauriRuntime>(
        fs: Arc<dyn FileSystem>,
        app_handle: AppHandle<R>,
        applog_path: &Path,
        session_id: &Uuid,
    ) -> Result<LogService> {
        // Rolling log file format
        let standard_log_format = tracing_subscriber::fmt::format()
            .with_file(false)
            .with_line_number(false)
            .with_target(false)
            .with_timer(ChronoLocal::new(TIMESTAMP_FORMAT.to_string()))
            .json()
            .flatten_event(true)
            .with_current_span(true);

        // Console log format
        let instrument_log_format = tracing_subscriber::fmt::format()
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .with_timer(ChronoLocal::new(TIMESTAMP_FORMAT.to_string()))
            .compact()
            .with_ansi(true);

        let sessionlog_path = applog_path.join("sessions").join(session_id.to_string());
        fs::create_dir_all(&sessionlog_path)?;

        // Create non-blocking writers
        let applog_queue = Arc::new(Mutex::new(VecDeque::new()));
        let (applog_writer, _applog_writerguard) =
            tracing_appender::non_blocking(RollingLogWriter::new(
                applog_path.to_path_buf(),
                DUMP_THRESHOLD,
                applog_queue.clone(),
            ));

        let sessionlog_queue = Arc::new(Mutex::new(VecDeque::new()));
        let (sessionlog_writer, _sessionlog_writerguard) =
            tracing_appender::non_blocking(RollingLogWriter::new(
                sessionlog_path.clone(),
                DUMP_THRESHOLD,
                sessionlog_queue.clone(),
            ));

        let (taurilog_writer, _taurilog_writerguard) =
            tracing_appender::non_blocking(TauriLogWriter::new(app_handle.clone()));

        let subscriber = tracing_subscriber::registry()
            .with(
                // Showing all logs (including span events) to the console
                tracing_subscriber::fmt::layer()
                    .event_format(instrument_log_format)
                    .with_span_events(FmtSpan::CLOSE)
                    .with_ansi(true)
                    .with_writer(io::stdout),
            )
            .with(
                // Emitting all logs to the frontend at LOGGING_SERVICE_CHANNEL
                tracing_subscriber::fmt::layer()
                    .event_format(standard_log_format.clone())
                    .fmt_fields(JsonFields::default())
                    .with_writer(taurilog_writer),
            )
            .with(
                // Rolling writer for app-scope logs
                tracing_subscriber::fmt::layer()
                    .event_format(standard_log_format.clone())
                    .fmt_fields(JsonFields::default())
                    .with_writer(applog_writer)
                    .with_filter(filter_fn(|metadata| {
                        metadata.level() < &Level::TRACE && metadata.target() == APP_SCOPE
                    })),
            )
            .with(
                // Rolling writer for session-scope logs
                tracing_subscriber::fmt::layer()
                    .event_format(standard_log_format.clone())
                    .fmt_fields(JsonFields::default())
                    .with_writer(sessionlog_writer)
                    .with_filter(filter_fn(|metadata| {
                        metadata.level() < &Level::TRACE && metadata.target() == SESSION_SCOPE
                    })),
            );

        tracing::subscriber::set_global_default(subscriber)?;
        Ok(Self {
            fs,
            applog_path: applog_path.to_path_buf(),
            sessionlog_path,
            applog_queue,
            sessionlog_queue,
            _applog_writerguard,
            _sessionlog_writerguard,
            _taurilog_writerguard,
        })
    }

    pub(crate) async fn list_logs_with_filter(
        &self,
        filter: &LogFilter,
    ) -> LogServiceResult<Vec<LogEntryInfo>> {
        // Combining app and session logs from both the queue and the files
        let app_logs = self
            .combine_logs(&self.applog_path, filter, self.applog_queue.clone())
            .await?;
        let session_logs = self
            .combine_logs(&self.sessionlog_path, filter, self.sessionlog_queue.clone())
            .await?;
        let merged_logs = LogService::merge_logs_chronologically(app_logs, session_logs)
            .into_iter()
            .map(|item| item.1)
            .collect();

        Ok(merged_logs)
    }

    pub(crate) async fn delete_logs(
        &self,
        input: Vec<&LogEntryRef>,
    ) -> LogServiceResult<Vec<LogItemSourceInfo>> {
        let mut file_entries = Vec::new();
        let mut result = Vec::new();
        for entry_ref in input {
            let datetime = DateTime::parse_from_str(&entry_ref.timestamp, TIMESTAMP_FORMAT);
            if let Err(_e) = datetime {
                // TODO: Notify the frontend that an error occured in parsing the timestamp
                continue;
            }

            // Try deleting from applog queue
            let mut applog_queue_lock = self.applog_queue.lock();
            let idx = applog_queue_lock.iter().position(|x| x.id == entry_ref.id);
            if let Some(idx) = idx {
                applog_queue_lock.remove(idx);
                result.push(LogItemSourceInfo {
                    id: entry_ref.id.clone(),
                    file_path: None,
                });
                continue;
            }
            drop(applog_queue_lock);

            // Try deleting from sessionlog queue
            let mut sessionlog_queue_lock = self.sessionlog_queue.lock();
            let idx = sessionlog_queue_lock
                .iter()
                .position(|x| x.id == entry_ref.id);
            if let Some(idx) = idx {
                sessionlog_queue_lock.remove(idx);
                result.push(LogItemSourceInfo {
                    id: entry_ref.id.clone(),
                    file_path: None,
                });
                continue;
            }
            drop(sessionlog_queue_lock);

            // Try deleting the entry from the log files
            file_entries.push((entry_ref.id.clone(), datetime.expect("Already checked")))
        }
        file_entries.sort_by(|a, b| a.1.cmp(&b.1));
        if file_entries.is_empty() {
            return Ok(result);
        }

        // Deleting the remaining entries from the files
        result.extend(self.delete_logs_from_files(file_entries).await?);
        // TODO: Reporting entries that were not found during deletion?
        Ok(result)
    }
}
impl LogService {
    // Tracing disallows non-constant value for `target`
    // So we have to manually match it
    pub fn trace(&self, scope: LogScope, payload: LogPayload) {
        match scope {
            LogScope::App => {
                trace!(
                    target: APP_SCOPE,
                    id = new_id(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                trace!(
                    target: SESSION_SCOPE,
                    id = new_id(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
        }
    }

    pub fn debug(&self, scope: LogScope, payload: LogPayload) {
        match scope {
            LogScope::App => {
                debug!(
                    target: APP_SCOPE,
                    id = new_id(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                debug!(
                    target: SESSION_SCOPE,
                    id = new_id(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
        }
    }

    pub fn info(&self, scope: LogScope, payload: LogPayload) {
        match scope {
            LogScope::App => {
                info!(
                    target: APP_SCOPE,
                    id = new_id(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                info!(
                    target: SESSION_SCOPE,
                    id = new_id(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
        }
    }

    pub fn warn(&self, scope: LogScope, payload: LogPayload) {
        match scope {
            LogScope::App => {
                warn!(
                    target: APP_SCOPE,
                    id = new_id(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                warn!(
                    target: SESSION_SCOPE,
                    id = new_id(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
        }
    }

    pub fn error(&self, scope: LogScope, payload: LogPayload) {
        match scope {
            LogScope::App => {
                error!(
                    target: APP_SCOPE,
                    id = new_id(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                error!(
                    target: SESSION_SCOPE,
                    id = new_id(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
        }
    }
}

impl LogService {
    async fn find_log_files_by_range(
        &self,
        path: &Path,
        start: Option<&DateTime<FixedOffset>>,
        end: Option<&DateTime<FixedOffset>>,
    ) -> Result<Vec<PathBuf>> {
        // Find log files that cover the entire range of logs to delete
        let mut file_list = Vec::new();

        let mut read_dir = self.fs.read_dir(path).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            let path = entry.path();
            if !path.is_file()
                || path.file_stem().is_none()
                || path.extension() != Some(OsStr::new("log"))
            {
                continue;
            }
            let stem = path.file_stem().unwrap().to_string_lossy().to_string();
            if let Ok(dt) = DateTime::parse_from_str(&stem, FILE_TIMESTAMP_FORMAT) {
                file_list.push((path, dt));
            }
            // Skip a log file if its name is not well-formatted
            // TODO: Delete invalid log files?
        }

        if file_list.is_empty() {
            return Ok(Vec::new());
        }
        // Sort the log files chronologically and find all log files that might cover the entries
        file_list.sort_by(|a, b| a.1.cmp(&b.1));

        // If the start datetime coincides with a logfile's timestamp, we start from that file
        // Otherwise, we start from the latest file that began before the start timestamp
        // Similarly, we end at the file with the end_dt, or the latest file before it
        // Both start_idx and end_idx are inclusive

        let start_idx = if let Some(start) = start {
            file_list
                    .binary_search_by(|(_, dt)| dt.cmp(&start))
                    .unwrap_or_else(|idx|
                        // Clamp to the oldest file
                        if idx == 0 { 0 } else { idx - 1 })
        } else {
            0
        };

        let end_idx = if let Some(end) = end {
            file_list
                .binary_search_by(|(_, dt)| dt.cmp(&end))
                .unwrap_or_else(|idx| idx - 1)
        } else {
            file_list.len() - 1
        };

        let files = file_list[start_idx..=end_idx]
            .into_iter()
            .map(|(path, _)| path.to_path_buf())
            .collect();
        Ok(files)
    }
}

/// Helper methods for delete_logs
impl LogService {
    async fn delete_logs_from_files(
        &self,
        entries: Vec<(String, DateTime<FixedOffset>)>,
    ) -> LogServiceResult<Vec<LogItemSourceInfo>> {
        let start = entries.first().unwrap().1.clone();
        let end = entries.last().unwrap().1.clone();

        let mut deleted_entries = Vec::new();
        let mut ids_to_delete = entries.iter().map(|x| x.0.clone()).collect::<HashSet<_>>();

        // Deleting entries from app log files
        {
            let log_files = self
                .find_log_files_by_range(&self.applog_path, Some(&start), Some(&end))
                .await?;
            for file in log_files {
                deleted_entries.extend(self.update_log_file(&file, &mut ids_to_delete).await?);
            }
        }

        // Deleting entries from session log files
        {
            let log_files = self
                .find_log_files_by_range(&self.sessionlog_path, Some(&start), Some(&end))
                .await?;
            for file in log_files {
                deleted_entries.extend(self.update_log_file(&file, &mut ids_to_delete).await?);
            }
        }

        Ok(deleted_entries)
    }

    async fn update_log_file(
        &self,
        path: &Path,
        ids: &mut HashSet<String>,
    ) -> Result<Vec<LogItemSourceInfo>> {
        let mut new_content = String::new();
        let mut removed_entries = Vec::new();

        let f = self.fs.open_file(path).await?;
        let reader = BufReader::new(f);
        for line in reader.lines() {
            let line = line?;
            let log_entry: LogEntryInfo = serde_json::from_str(&line)?;
            if ids.contains(&log_entry.id) {
                // Splice this line from the output content
                removed_entries.push(LogItemSourceInfo {
                    id: log_entry.id.clone(),
                    file_path: Some(path.to_path_buf()),
                });
            } else {
                new_content.push_str(&line);
                new_content.push('\n');
            }
        }
        // We don't need to update the file if no deletion is made
        if !removed_entries.is_empty() {
            self.fs
                .create_file_with(
                    path,
                    new_content.as_bytes(),
                    CreateOptions {
                        overwrite: true,
                        ignore_if_exists: true,
                    },
                )
                .await?;
        }

        // TODO: Should we delete a file if all entries in it are deleted?
        Ok(removed_entries)
    }
}

/// Helper methods for list_logs
impl LogService {
    async fn parse_file_with_filter(
        &self,
        records: &mut Vec<(NaiveDateTime, LogEntryInfo)>,
        file_path: &Path,
        filter: &LogFilter,
    ) -> Result<()> {
        // In the log files, each line is a LogEntry JSON object
        // Entries in each log files are already sorted chronologically
        let file = self.fs.open_file(file_path).await?;

        for line in BufReader::new(file).lines() {
            let line = line?;
            let log_entry: LogEntryInfo = serde_json::from_str(&line)?;

            if filter.check_entry(&log_entry)? {
                let timestamp =
                    NaiveDateTime::parse_from_str(&log_entry.timestamp, TIMESTAMP_FORMAT)?;
                records.push((timestamp, log_entry));
            }
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

        let mut dates = filter.dates.iter().collect::<Vec<_>>();
        dates.sort();
        let start = dates
            .first()
            .and_then(|date| date.and_hms_milli_opt(0, 0, 0, 0))
            .map(|dt| naive_to_local_fixed(&dt));
        let end = dates
            .last()
            .and_then(|date| date.and_hms_milli_opt(0, 0, 0, 0))
            .and_then(|dt|
            // Ensure that the records on the last day in the filter is also checked
            dt.checked_add_days(Days::new(1)))
            .map(|dt| naive_to_local_fixed(&dt));

        let files = self
            .find_log_files_by_range(path, start.as_ref(), end.as_ref())
            .await?;

        for file in files {
            self.parse_file_with_filter(&mut result, &file, filter)
                .await?
        }

        result.extend({
            let lock = queue.lock();

            lock.clone()
                .into_iter()
                .filter(|entry| matches!(filter.check_entry(&entry), Ok(true)))
                .filter_map(|entry| {
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

// TODO: Maybe there's a better way to do this
/// Convert a NaiveDateTime to a DateTime<FixedOffset>
fn naive_to_local_fixed(naive: &NaiveDateTime) -> DateTime<FixedOffset> {
    // Grab the current local offset (in seconds east of UTC)
    let offset_seconds = Local::now().offset().local_minus_utc();

    // Build a FixedOffset from it
    let fixed_offset =
        FixedOffset::east_opt(offset_seconds).expect("The offset seconds must be valid");

    // Attach it to your NaiveDateTime
    //    from_local_datetime returns a LocalResult — unwrap() here
    //    since you know your naive is “valid” in that offset.
    fixed_offset.from_local_datetime(naive).unwrap()
}

#[cfg(test)]
mod tests {
    use moss_fs::RealFileSystem;
    use moss_testutils::random_name::random_string;
    use std::{fs::create_dir_all, sync::atomic::AtomicUsize, time::Duration};
    use tauri::{Listener, Manager};
    use tokio::fs::remove_dir_all;

    use super::*;
    use crate::constants::LOGGING_SERVICE_CHANNEL;

    fn random_app_log_path() -> PathBuf {
        Path::new("tests").join("data").join(random_string(10))
    }

    #[tokio::test]
    async fn test_taurilog_writer() {
        let test_app_log_path = random_app_log_path();
        create_dir_all(&test_app_log_path).unwrap();

        let fs = Arc::new(RealFileSystem::new());
        let mock_app = tauri::test::mock_app();
        let session_id = Uuid::new_v4();
        let logging_service = LogService::new(
            fs,
            mock_app.app_handle().clone(),
            &test_app_log_path,
            &session_id,
        )
        .unwrap();

        let counter = Arc::new(AtomicUsize::new(0));

        {
            let counter = counter.clone();
            mock_app.listen(LOGGING_SERVICE_CHANNEL, move |_| {
                counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            });
        }

        logging_service.debug(
            LogScope::App,
            LogPayload {
                resource: None,
                message: "".to_string(),
            },
        );

        tokio::time::sleep(Duration::from_millis(100)).await;
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);

        remove_dir_all(&test_app_log_path).await.unwrap()
    }
}
