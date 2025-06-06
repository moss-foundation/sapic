pub mod api;
mod constants;
mod models;
mod writers;

use anyhow::Result;
use moss_app::service::prelude::AppService;
use nanoid::nanoid;
use std::{
    collections::VecDeque,
    fs, io,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tracing::{Level, debug, error, info, trace, warn};
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
    models::types::LogEntry,
    writers::{
        applog_writer::AppLogMakeWriter, sessionlog_writer::SessionLogMakeWriter,
        taurilog_writer::TauriLogMakeWriter,
    },
};

fn new_id() -> String {
    nanoid!(ID_LENGTH)
}

pub const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3f%z";
pub const FILE_DATE_FORMAT: &'static str = "%Y-%m-%d-%H-%M";

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
    applog_path: PathBuf,
    sessionlog_path: PathBuf,
    applog_queue: Arc<Mutex<VecDeque<LogEntry>>>,
    sessionlog_queue: Arc<Mutex<VecDeque<LogEntry>>>,
}

impl LoggingService {
    pub fn new<R: TauriRuntime>(
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

        let session_path = applog_path.join("sessions").join(session_id.to_string());
        fs::create_dir_all(&session_path)?;

        let applog_queue = Arc::new(Mutex::new(VecDeque::new()));
        let sessionlog_queue = Arc::new(Mutex::new(VecDeque::new()));
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
                    .with_writer(TauriLogMakeWriter {
                        app_handle: app_handle.clone(),
                    }),
            )
            .with(
                // Rolling writer for app-scope logs
                tracing_subscriber::fmt::layer()
                    .event_format(standard_log_format.clone())
                    .fmt_fields(JsonFields::default())
                    .with_writer(AppLogMakeWriter::new(
                        &applog_path,
                        10,
                        applog_queue.clone(),
                    ))
                    .with_filter(filter_fn(|metadata| {
                        metadata.level() < &Level::TRACE && metadata.target() == APP_SCOPE
                    })),
            )
            .with(
                // Rolling writer for session-scope logs
                tracing_subscriber::fmt::layer()
                    .event_format(standard_log_format.clone())
                    .fmt_fields(JsonFields::default())
                    .with_writer(SessionLogMakeWriter::new(
                        &session_path,
                        10,
                        sessionlog_queue.clone(),
                    ))
                    .with_filter(filter_fn(|metadata| {
                        metadata.level() < &Level::TRACE && metadata.target() == SESSION_SCOPE
                    })),
            );

        tracing::subscriber::set_global_default(subscriber)?;
        Ok(Self {
            applog_path: applog_path.to_path_buf(),
            sessionlog_path: session_path,
            applog_queue,
            sessionlog_queue,
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
    use super::*;
    use crate::{constants::LOGGING_SERVICE_CHANNEL, models::operations::DeleteLogInput};
    use moss_testutils::random_name::random_string;
    use std::{fs::create_dir_all, time::Duration};
    use tauri::{Listener, Manager};
    use tokio::fs::remove_dir_all;
    use tracing::instrument;

    #[instrument(level = "trace", skip_all)]
    async fn create_collection(path: &Path, name: &str, log_service: &LoggingService) {
        let collection_path = path.join(name);

        log_service.info(
            LogScope::Session,
            LogPayload {
                resource: Some(collection_path.to_string_lossy().to_string()),
                message: format!(
                    "Created collection {} at {}",
                    name,
                    collection_path.to_string_lossy().to_string()
                ),
            },
        );
        log_service.info(
            LogScope::App,
            LogPayload {
                resource: Some(collection_path.to_string_lossy().to_string()),
                message: "Successfully created collection".to_string(),
            },
        )
    }

    #[instrument(level = "trace", skip_all)]
    async fn create_request(collection_path: &Path, name: &str, log_service: &LoggingService) {
        let request_path = collection_path.join(name);
        log_service.info(
            LogScope::Session,
            LogPayload {
                resource: Some(request_path.to_string_lossy().to_string()),
                message: format!(
                    "Created request {} at {}",
                    name,
                    request_path.to_string_lossy().to_string()
                ),
            },
        );
        log_service.info(
            LogScope::App,
            LogPayload {
                resource: Some(request_path.to_string_lossy().to_string()),
                message: "Successfully created request".to_string(),
            },
        )
    }

    #[instrument(level = "trace", skip_all)]
    async fn something_terrible(log_service: &LoggingService) {
        log_service.warn(
            LogScope::App,
            LogPayload {
                resource: None,
                message: "Something bad!".to_string(),
            },
        );
        log_service.error(
            LogScope::App,
            LogPayload {
                resource: None,
                message: "Something terrible!".to_string(),
            },
        );
    }

    fn random_app_log_path() -> PathBuf {
        Path::new("test").join(random_string(10))
    }

    #[tokio::test]
    async fn test_delete_log() {
        let test_app_log_path = random_app_log_path();
        create_dir_all(&test_app_log_path).unwrap();
        let mock_app = tauri::test::mock_app();
        let session_id = Uuid::new_v4();
        let logging_service = LoggingService::new(
            mock_app.app_handle().clone(),
            &test_app_log_path,
            &session_id,
        )
        .unwrap();

        for i in 0..15 {
            logging_service.debug(
                LogScope::App,
                LogPayload {
                    resource: None,
                    message: "Test".to_string(),
                },
            );
            logging_service.debug(
                LogScope::Session,
                LogPayload {
                    resource: None,
                    message: "Test".to_string(),
                },
            )
        }

        let old_logs = logging_service
            .list_logs(&ListLogsInput {
                dates: vec![],
                levels: vec![],
                resource: None,
            })
            .unwrap()
            .contents;

        // Test deleting both from the file and from the queue

        let first_entry = old_logs.first().unwrap().clone();
        let first_id = first_entry.id;
        dbg!(&first_id);
        let first_timestamp = first_entry.timestamp;
        logging_service
            .delete_log(&DeleteLogInput {
                timestamp: first_timestamp,
                id: first_id.clone(),
            })
            .unwrap();

        let last_entry = old_logs.last().unwrap().clone();
        let last_id = last_entry.id;
        dbg!(&last_id);
        let last_timestamp = last_entry.timestamp;
        logging_service
            .delete_log(&DeleteLogInput {
                timestamp: last_timestamp,
                id: last_id.clone(),
            })
            .unwrap();

        let updated_logs = logging_service
            .list_logs(&ListLogsInput {
                dates: vec![],
                levels: vec![],
                resource: None,
            })
            .unwrap()
            .contents;

        assert_eq!(old_logs.len() - updated_logs.len(), 2);
        assert!(updated_logs.iter().find(|log| log.id == first_id).is_none());
        assert!(updated_logs.iter().find(|log| log.id == last_id).is_none());
        remove_dir_all(&test_app_log_path).await.unwrap();
    }

    #[tokio::test]
    async fn test_taurilog_writer() {
        let test_app_log_path = random_app_log_path();
        let mock_app = tauri::test::mock_app();
        let session_id = Uuid::new_v4();
        let logging_service = LoggingService::new(
            mock_app.app_handle().clone(),
            &test_app_log_path,
            &session_id,
        )
        .unwrap();

        mock_app.listen(LOGGING_SERVICE_CHANNEL, |event| {
            println!("{}", event.payload())
        });

        let collection_path = Path::new("").join("TestCollection");

        create_collection(Path::new(""), "TestCollection", &logging_service).await;
        create_request(&collection_path, "TestRequest", &logging_service).await;
        something_terrible(&logging_service).await;

        remove_dir_all(&test_app_log_path).await.unwrap()
    }
}
