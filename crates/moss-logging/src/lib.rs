pub mod api;
mod constants;
pub mod models;

mod writers;

use anyhow::Result;
use moss_app::service::prelude::AppService;
use moss_fs::FileSystem;
use nanoid::nanoid;
use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    fs, io,
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
    constants::{APP_SCOPE, ID_LENGTH, SESSION_SCOPE},
    models::types::LogEntryInfo,
    writers::{rollinglog_writer::RollingLogWriter, taurilog_writer::TauriLogWriter},
};

fn new_id() -> String {
    nanoid!(ID_LENGTH)
}

pub const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3f%z";
pub const FILE_TIME_FORMAT: &'static str = "%Y-%m-%d-%H-%M";

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
pub struct LoggingService {
    fs: Arc<dyn FileSystem>,
    applog_path: PathBuf,
    sessionlog_path: PathBuf,
    applog_queue: Arc<Mutex<VecDeque<LogEntryInfo>>>,
    sessionlog_queue: Arc<Mutex<VecDeque<LogEntryInfo>>>,
    _applog_writerguard: WorkerGuard,
    _sessionlog_writerguard: WorkerGuard,
    _taurilog_writerguard: WorkerGuard,
}

impl LoggingService {
    pub fn new<R: TauriRuntime>(
        fs: Arc<dyn FileSystem>,
        app_handle: AppHandle<R>,
        applog_path: &Path,
        session_id: &Uuid,
    ) -> Result<LoggingService> {
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
}

impl LoggingService {
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

impl AppService for LoggingService {}

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
        let logging_service = LoggingService::new(
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
