mod rollinglog_writer;
mod taurilog_writer;

use anyhow::{Result, anyhow};
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use moss_applib::ServiceMarker;
use moss_common::{api::OperationError, nanoid::new_nanoid};
use moss_db::primitives::AnyValue;
use moss_fs::{CreateOptions, FileSystem};
use moss_storage::{
    GlobalStorage,
    primitives::segkey::SegKey,
    storage::operations::{GetItem, TransactionalRemoveItem},
};
use nanoid::nanoid;
use std::{
    collections::{HashSet, VecDeque},
    ffi::OsStr,
    fs,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
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

use crate::{
    models::types::{LogEntryInfo, LogItemSourceInfo},
    services::log_service::{
        constants::*, rollinglog_writer::RollingLogWriter, taurilog_writer::TauriLogWriter,
    },
};

pub mod constants {
    pub const APP_SCOPE: &'static str = "app";
    pub const SESSION_SCOPE: &'static str = "session";

    pub const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3f%z";

    pub const FILE_TIMESTAMP_FORMAT: &'static str = "%Y_%m_%dT%H_%M_%S%z";
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
    storage: Arc<dyn GlobalStorage>,
    _applog_writerguard: WorkerGuard,
    _sessionlog_writerguard: WorkerGuard,
    _taurilog_writerguard: WorkerGuard,
}

impl ServiceMarker for LogService {}

impl LogService {
    pub fn new<R: TauriRuntime>(
        fs: Arc<dyn FileSystem>,
        app_handle: AppHandle<R>,
        applog_path: &Path,
        session_id: &str,
        storage: Arc<dyn GlobalStorage>,
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
                storage.clone(),
            ));

        let sessionlog_queue = Arc::new(Mutex::new(VecDeque::new()));
        let (sessionlog_writer, _sessionlog_writerguard) =
            tracing_appender::non_blocking(RollingLogWriter::new(
                sessionlog_path.clone(),
                DUMP_THRESHOLD,
                sessionlog_queue.clone(),
                storage.clone(),
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

        // FIXME: This is a hack to avoid panic when running multiple tests
        // We should find a better way to handle this
        if let Err(_) = tracing::subscriber::set_global_default(subscriber) {
            // Global subscriber already set
        }

        Ok(Self {
            fs,
            applog_path: applog_path.to_path_buf(),
            sessionlog_path,
            applog_queue,
            sessionlog_queue,
            storage,
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
        input: impl Iterator<Item = &str>,
    ) -> LogServiceResult<Vec<LogItemSourceInfo>> {
        let mut file_entries = Vec::new();
        let mut result = Vec::new();
        for entry_id in input {
            // Try deleting from applog queue
            let mut applog_queue_lock = self
                .applog_queue
                .lock()
                .map_err(|_| anyhow!("Mutex poisoned"))?;
            let idx = applog_queue_lock.iter().position(|x| x.id == entry_id);
            if let Some(idx) = idx {
                applog_queue_lock.remove(idx);
                result.push(LogItemSourceInfo {
                    id: entry_id.to_string(),
                    file_path: None,
                });
                continue;
            }
            drop(applog_queue_lock);

            // Try deleting from sessionlog queue
            let mut sessionlog_queue_lock = self
                .sessionlog_queue
                .lock()
                .map_err(|_| anyhow!("Mutex poisoned"))?;
            let idx = sessionlog_queue_lock.iter().position(|x| x.id == entry_id);
            if let Some(idx) = idx {
                sessionlog_queue_lock.remove(idx);
                result.push(LogItemSourceInfo {
                    id: entry_id.to_string(),
                    file_path: None,
                });
                continue;
            }
            drop(sessionlog_queue_lock);

            // Try deleting the entry from the log files
            file_entries.push(entry_id);
        }
        if file_entries.is_empty() {
            return Ok(result);
        }

        // Deleting the remaining entries from the files
        result.extend(self.delete_logs_from_files(&file_entries).await?);
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
                    id = new_nanoid(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                trace!(
                    target: SESSION_SCOPE,
                    id = new_nanoid(),
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
                    id = new_nanoid(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                debug!(
                    target: SESSION_SCOPE,
                    id = new_nanoid(),
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
                    id = new_nanoid(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                info!(
                    target: SESSION_SCOPE,
                    id = new_nanoid(),
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
                    id = new_nanoid(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                warn!(
                    target: SESSION_SCOPE,
                    id = new_nanoid(),
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
                    id = new_nanoid(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                error!(
                    target: SESSION_SCOPE,
                    id = new_nanoid(),
                    resource = payload.resource,
                    message = payload.message
                )
            }
        }
    }
}

/// Helper methods for delete_logs
impl LogService {
    fn find_files_to_update(&self, entries: &[&str]) -> Result<HashSet<PathBuf>> {
        let mut files = HashSet::new();
        for entry_id in entries {
            let segkey = SegKey::new(&entry_id).to_segkey_buf();
            let log_store = self.storage.log_store();
            let value = GetItem::get(log_store.as_ref(), segkey)?;
            let path: PathBuf = AnyValue::deserialize(&value)?;
            files.insert(path);
        }

        Ok(files)
    }

    async fn delete_logs_from_files(
        &self,
        entries: &[&str],
    ) -> LogServiceResult<Vec<LogItemSourceInfo>> {
        let mut deleted_entries = Vec::new();
        let mut ids_to_delete = entries
            .iter()
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();

        let log_files = self.find_files_to_update(entries)?;
        for file in log_files {
            deleted_entries.extend(self.update_log_file(&file, &mut ids_to_delete).await?);
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

        let mut write_txn = self.storage.begin_write()?;
        let log_store = self.storage.log_store();

        for line in reader.lines() {
            let line = line?;
            let log_entry: LogEntryInfo = serde_json::from_str(&line)?;
            if ids.contains(&log_entry.id) {
                // Splice this line from the output content
                removed_entries.push(LogItemSourceInfo {
                    id: log_entry.id.clone(),
                    file_path: Some(path.to_path_buf()),
                });
                // Remove the entry from the database
                let segkey = SegKey::new(&log_entry.id).to_segkey_buf();
                TransactionalRemoveItem::remove(log_store.as_ref(), &mut write_txn, segkey)?;
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
        write_txn.commit()?;

        // TODO: Should we delete a file if all entries in it are deleted?
        Ok(removed_entries)
    }
}

/// Helper methods for list_logs
impl LogService {
    async fn find_files_by_dates(
        &self,
        path: &Path,
        dates_filter: &HashSet<NaiveDate>,
    ) -> Result<Vec<PathBuf>> {
        // Find log files with the given dates
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

            let parse_result = DateTime::parse_from_str(&stem, FILE_TIMESTAMP_FORMAT);
            if parse_result.is_err() {
                // Ignore files with invalid timestamp name
                continue;
            }
            let dt = parse_result?;
            let naive_date = dt.date_naive();
            // Either we have no dates filter, or the filter contains the file date
            if dates_filter.is_empty() || dates_filter.contains(&naive_date) {
                file_list.push((path, dt));
            }
        }

        // Sort the log files chronologically
        file_list.sort_by_key(|(_, dt)| *dt);

        Ok(file_list.into_iter().map(|(path, _)| path).collect())
    }
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
                // FIXME: Should we simply skip the line if the timestamp is invalid?
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
        // And append the current log queue at the end if they pass the filter
        let mut result = Vec::new();

        let dates = filter.dates.iter().cloned().collect::<HashSet<_>>();
        let files = self.find_files_by_dates(path, &dates).await?;

        // The files are sorted chronologically, so are the log entries within a file
        // This will produce a vec of sorted LogEntryInfo
        for file in files {
            self.parse_file_with_filter(&mut result, &file, &filter)
                .await?;
        }

        // The logs in the queue must be more recent than the logs in files
        // So we append them to the end
        result.extend({
            let lock = queue.lock().map_err(|_| anyhow!("Mutex poisoned"))?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::LOGGING_SERVICE_CHANNEL;
    use moss_fs::RealFileSystem;
    use moss_storage::global_storage::GlobalStorageImpl;
    use moss_testutils::random_name::random_string;
    use std::{fs::create_dir_all, sync::atomic::AtomicUsize, time::Duration};
    use tauri::{Listener, Manager};
    use tokio::fs::remove_dir_all;

    fn random_app_log_path() -> PathBuf {
        Path::new("tests").join("data").join(random_string(10))
    }

    #[tokio::test]
    async fn test_taurilog_writer() {
        let test_app_log_path = random_app_log_path();
        create_dir_all(&test_app_log_path).unwrap();

        let fs = Arc::new(RealFileSystem::new());
        let mock_app = tauri::test::mock_app();
        let session_id = new_nanoid();
        let storage = Arc::new(GlobalStorageImpl::new(&test_app_log_path).unwrap());
        let logging_service = LogService::new(
            fs,
            mock_app.app_handle().clone(),
            &test_app_log_path,
            &session_id,
            storage.clone(),
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
