mod rollinglog_writer;
mod taurilog_writer;

use anyhow::Result;
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use moss_applib::Service;
use moss_common::api::{OperationError, OperationResult};
use moss_fs::{CreateOptions, FileSystem};
use nanoid::nanoid;
use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    fs,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
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
    pub const FILE_TIME_FORMAT: &'static str = "%Y-%m-%d-%H-%M";
}

fn new_id() -> String {
    nanoid!(ID_LENGTH)
}

const DUMP_THRESHOLD: usize = 10;

pub struct LogPayload {
    pub resource: Option<String>,
    pub message: String,
}

pub enum LogScope {
    App,
    Session,
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

    // TODO: should use LogServiceError
    // TODO: should accept a list of log entry refs
    pub async fn delete_log(&self, input: &LogEntryRef) -> OperationResult<LogItemSourceInfo> {
        let datetime =
            DateTime::parse_from_str(&input.timestamp, TIMESTAMP_FORMAT).map_err(|_| {
                OperationError::InvalidInput("The input timestamp is invalid".to_string())
            })?;
        {
            let mut applog_queue_lock = self.applog_queue.lock();
            let idx = applog_queue_lock.iter().position(|x| x.id == input.id);
            if let Some(idx) = idx {
                applog_queue_lock.remove(idx);
                return Ok(LogItemSourceInfo {
                    id: input.id.clone(),
                    file_path: None,
                });
            }
        }
        {
            let mut sessionlog_queue_lock = self.sessionlog_queue.lock();
            let idx = sessionlog_queue_lock.iter().position(|x| x.id == input.id);
            if let Some(idx) = idx {
                sessionlog_queue_lock.remove(idx);
                return Ok(LogItemSourceInfo {
                    id: input.id.clone(),
                    file_path: None,
                });
            }
        }
        {
            let log_files = self.identify_log_file(&self.applog_path, datetime).await?;
            for file in log_files {
                let updated = self.update_log_file(&file, &input.id).await?;
                if updated {
                    return Ok(LogItemSourceInfo {
                        id: input.id.clone(),
                        file_path: Some(file),
                    });
                }
            }
        }
        {
            let log_files = self
                .identify_log_file(&self.sessionlog_path, datetime)
                .await?;
            for file in log_files {
                let updated = self.update_log_file(&file, &input.id).await?;
                if updated {
                    return Ok(LogItemSourceInfo {
                        id: input.id.clone(),
                        file_path: Some(file),
                    });
                }
            }
        }

        Err(OperationError::NotFound(format!(
            "Log id {} not found",
            input.id
        )))
    }

    async fn identify_log_file(
        &self,
        path: &Path,
        datetime: DateTime<FixedOffset>,
    ) -> Result<Vec<PathBuf>> {
        // Use timestamp to identify which file to update
        // We just need to check the last two log files that starts before the log entry's timestamp
        // Two because of potential timestamp rounding issue

        // OPTIMIZE: We might use a binary search here but I'm not sure if it's necessary
        let mut file_list = Vec::new();

        let mut read_dir = self.fs.read_dir(path).await?;
        while let Some(entry) = read_dir.next_entry().await.unwrap_or(None) {
            let path = entry.path();
            if !path.is_file() || path.file_stem().is_none() {
                continue;
            }
            let stem = path.file_stem().unwrap().to_string_lossy().to_string();
            // example: 2025-06-06-18-00.log
            if let Ok(dt) = NaiveDateTime::parse_from_str(&stem, FILE_TIME_FORMAT) {
                file_list.push((path, dt));
            }
            // Skip a log file if its name is not well-formatted
        }

        // Sort the log files chronologically and find the ones that might contain the entry
        file_list.sort_by(|a, b| a.1.cmp(&b.1));
        let files = file_list
            .into_iter()
            .filter(|item| item.1 <= datetime.naive_local())
            .rev()
            .take(2)
            .map(|item| item.0)
            .collect::<Vec<_>>();
        Ok(files)
    }

    async fn update_log_file(&self, path: &Path, id: &str) -> Result<bool> {
        // Try to delete the entry with given id
        // If deleted, return Ok(true)
        let mut new_content = String::new();
        let mut deleted = false;

        {
            let f = self.fs.open_file(path).await?;
            let reader = BufReader::new(f);
            for line in reader.lines() {
                let line = line?;
                let log_entry: LogEntryInfo = serde_json::from_str(&line)?;
                if log_entry.id == id {
                    // Splice this line from the output content
                    deleted = true;
                } else {
                    new_content.push_str(&line);
                    new_content.push('\n');
                }
            }
        }

        // We don't need to update the file if no deletion is made
        if deleted {
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
            Ok(true)
        } else {
            Ok(false)
        }
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
