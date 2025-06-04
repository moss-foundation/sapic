mod makewriter;
mod models;
mod tokens;

use anyhow::Result;
use chrono::{DateTime, FixedOffset, NaiveDate};
use moss_app::service::prelude::AppService;
use serde_json::Value as JsonValue;
use std::{
    collections::HashSet,
    ffi::OsStr,
    fs,
    fs::File,
    io,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    str::FromStr,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tracing::{Level, debug, error, info, trace, warn};
use tracing_appender::{non_blocking::WorkerGuard, rolling::Rotation};
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
    makewriter::TauriMakeWriter,
    models::{
        operations::{ListLogsInput, ListLogsOutput},
        types::{LogEntry, LogLevel},
    },
    tokens::{APP_SCOPE, LEVEL_LIT, RESOURCE_LIT, SESSION_SCOPE},
};

pub const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3f%z";
pub const FILE_DATE_FORMAT: &'static str = "%Y-%m-%d-%H-%M";

pub const LOGGING_SERVICE_CHANNEL: &'static str = "logging";

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

pub struct LogPayload {
    pub resource: Option<String>,
    pub message: String,
}

pub enum LogScope {
    App,
    Session,
}

// TODO: in-memory log
pub struct LoggingService {
    app_log_path: PathBuf,
    session_path: PathBuf,
    _app_log_guard: WorkerGuard,
    _session_log_guard: WorkerGuard,
}

impl LoggingService {
    fn parse_file_with_filter(
        records: &mut Vec<(DateTime<FixedOffset>, JsonValue)>,
        path: &Path,
        filter: &LogFilter,
    ) -> Result<()> {
        // In the log created by tracing-appender, each line is a JSON object for a JsonValue
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
    ) -> Result<Vec<(DateTime<FixedOffset>, JsonValue)>> {
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

impl LoggingService {
    pub fn new<R: TauriRuntime>(
        app_handle: AppHandle<R>,
        app_log_path: &Path,
        session_log_path: &Path,
        session_id: &Uuid,
    ) -> Result<LoggingService> {
        let standard_log_format = tracing_subscriber::fmt::format()
            .with_file(false)
            .with_line_number(false)
            .with_target(false)
            .with_timer(ChronoLocal::new(TIMESTAMP_FORMAT.to_string()))
            .json()
            .flatten_event(true)
            .with_current_span(true);

        let instrument_log_format = tracing_subscriber::fmt::format()
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .with_timer(ChronoLocal::new(TIMESTAMP_FORMAT.to_string()))
            .compact()
            .with_ansi(true);

        let session_path = session_log_path.join(session_id.to_string());
        let session_log_appender = tracing_appender::rolling::Builder::new()
            .rotation(Rotation::MINUTELY)
            .filename_suffix("log")
            .build(&session_path)?;

        let app_log_appender = tracing_appender::rolling::Builder::new()
            .rotation(Rotation::MINUTELY)
            .filename_suffix("log")
            .build(&app_log_path)?;
        let (session_log_writer, _session_log_guard) =
            tracing_appender::non_blocking(session_log_appender);
        let (app_log_writer, _app_log_guard) = tracing_appender::non_blocking(app_log_appender);

        let subscriber = tracing_subscriber::registry()
            .with(
                // Session log subscriber
                tracing_subscriber::fmt::layer()
                    .event_format(standard_log_format.clone())
                    .with_writer(session_log_writer)
                    .fmt_fields(JsonFields::default())
                    .with_filter(filter_fn(|metadata| {
                        metadata.level() < &Level::TRACE && metadata.target() == SESSION_SCOPE
                    })),
            )
            .with(
                // App log subscriber
                tracing_subscriber::fmt::layer()
                    .event_format(standard_log_format.clone())
                    .with_writer(app_log_writer)
                    .fmt_fields(JsonFields::default())
                    .with_filter(filter_fn(|metadata| {
                        metadata.level() < &Level::TRACE && metadata.target() == APP_SCOPE
                    })),
            )
            .with(
                // Showing all logs (including span events) to the console
                tracing_subscriber::fmt::layer()
                    .event_format(instrument_log_format)
                    .with_span_events(FmtSpan::CLOSE)
                    .with_ansi(true)
                    .with_writer(io::stdout),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .event_format(standard_log_format)
                    .fmt_fields(JsonFields::default())
                    .with_writer(TauriMakeWriter {
                        app_handle: app_handle.clone(),
                    }),
            );

        tracing::subscriber::set_global_default(subscriber)?;
        Ok(Self {
            _app_log_guard,
            _session_log_guard,
            app_log_path: app_log_path.to_path_buf(),
            session_path,
        })
    }

    pub fn list_logs(&self, input: &ListLogsInput) -> Result<ListLogsOutput> {
        // Combining both app and session log
        let filter: LogFilter = input.clone().into();
        let app_logs = self.combine_logs(&self.app_log_path, &filter)?;
        let session_logs = self.combine_logs(&self.session_path, &filter)?;
        let merged_logs = LoggingService::merge_logs_chronologically(app_logs, session_logs);

        let log_entries: Vec<LogEntry> = merged_logs
            .into_iter()
            .map(|(_dt, value)| serde_json::from_value(value))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ListLogsOutput {
            contents: log_entries,
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
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                trace!(
                    target: SESSION_SCOPE,
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
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                debug!(
                    target: SESSION_SCOPE,
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
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                info!(
                    target: SESSION_SCOPE,
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
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                warn!(
                    target: SESSION_SCOPE,
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
                    resource = payload.resource,
                    message = payload.message
                )
            }
            LogScope::Session => {
                error!(
                    target: SESSION_SCOPE,
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
    use tauri::{Listener, Manager};
    use tracing::instrument;

    #[instrument(level = "trace", skip_all)]
    async fn create_collection<R: TauriRuntime>(
        path: &Path,
        name: &str,
        log_service: &LoggingService,
    ) {
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
    async fn create_request<R: TauriRuntime>(
        collection_path: &Path,
        name: &str,
        log_service: &LoggingService,
    ) {
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
    async fn something_terrible<R: TauriRuntime>(log_service: &LoggingService) {
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

    const TEST_SESSION_LOG_FOLDER: &'static str = "logs/session";
    const TEST_APP_LOG_FOLDER: &'static str = "logs/app";
    #[tokio::test]
    async fn test() {
        let mock_app = tauri::test::mock_app();
        let session_id = Uuid::new_v4();
        let logging_service = LoggingService::new(
            mock_app.app_handle().clone(),
            Path::new(TEST_APP_LOG_FOLDER),
            Path::new(TEST_SESSION_LOG_FOLDER),
            &session_id,
        )
        .unwrap();

        mock_app.listen(LOGGING_SERVICE_CHANNEL, |event| {
            println!("{}", event.payload())
        });

        let collection_path = Path::new("").join("TestCollection");
        let request_path = Path::new("").join("TestCollection").join("TestRequest");

        create_collection(Path::new(""), "TestCollection", &logging_service).await;
        create_request(&collection_path, "TestRequest", &logging_service).await;
        something_terrible(&logging_service).await;

        let input = ListLogsInput {
            dates: vec![],
            levels: vec![LogLevel::INFO],
            resource: None,
        };

        let output = logging_service
            .list_logs(&input)
            .unwrap()
            .contents
            .into_iter()
            .map(|entry| serde_json::to_value(entry).unwrap().to_string())
            .collect::<Vec<_>>()
            .join("\n");

        fs::write("logs/filtered", output).unwrap();
    }
}
