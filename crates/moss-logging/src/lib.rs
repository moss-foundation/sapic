mod models;

use anyhow::Result;
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use moss_app::service::AppService;
use moss_session::SessionService;
use serde_json::Value as JsonValue;
use std::any::Any;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fmt::{write, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::{fs, io};
use tracing::{debug, error, info, trace, warn};
#[allow(unused_imports)] // Apparently these imports are used
use tracing::{event, instrument, Instrument, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::field::MakeVisitor;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::fmt::format::{FmtSpan, JsonFields};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::prelude::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::models::operations::{ListLogsInput, ListLogsOutput};
use crate::models::types::{LogEntry, LogLevel};
pub const LEVEL_LIT: &'static str = "level";
pub const COLLECTION_LIT: &'static str = "collection";
pub const REQUEST_LIT: &'static str = "request";

pub const APP_SCOPE: &'static str = "app";
pub const SESSION_SCOPE: &'static str = "session";

pub const TIMESTAMP_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3f%z";
pub const FILE_DATE_FORMAT: &'static str = "%Y-%m-%d-%H-%M";

// Empty field means that no filter will be applied
#[derive(Default)]
pub struct LogFilter {
    dates: HashSet<NaiveDate>,
    levels: HashSet<Level>,
    collection: Option<PathBuf>,
    request: Option<PathBuf>,
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
                .map(|date| NaiveDate::from_ymd_opt(
                    date.year as i32, date.month, date.day
                ).unwrap())
                .collect(),
            levels: input
                .levels
                .into_iter()
                .map(get_level)
                .collect(),
            collection: input
                .collection
                .map(PathBuf::from),
            request: input
                .request
                .map(PathBuf::from),
        }
    }
}

impl LogFilter {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_dates(self, dates: impl IntoIterator<Item = NaiveDate>) -> Self {
        Self {
            dates: self.dates.into_iter().chain(dates.into_iter()).collect(),
            ..self
        }
    }
    pub fn add_levels(self, levels: impl IntoIterator<Item = Level>) -> Self {
        Self {
            levels: self.levels.into_iter().chain(levels.into_iter()).collect(),
            ..self
        }
    }
    pub fn select_collection(self, collection: impl AsRef<Path>) -> Self {
        Self {
            collection: Some(collection.as_ref().to_path_buf()),
            ..self
        }
    }
    pub fn select_request(self, request: impl AsRef<Path>) -> Self {
        Self {
            request: Some(request.as_ref().to_path_buf().into()),
            ..self
        }
    }

}

pub struct LogPayload {
    collection: Option<PathBuf>,
    request: Option<PathBuf>,
    message: String,
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
    )
        -> Result<()> {
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

            if filter.collection.is_some() {
                if let Some(collection) = value
                    .get(COLLECTION_LIT)
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                {
                    if filter.collection.clone().unwrap() != collection {
                        continue;
                    }
                } else {
                    // With collection filter, skip entries without collection field
                    continue;
                }
            }

            if filter.request.is_some() {
                if let Some(request) = value
                    .get(REQUEST_LIT)
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                {
                    if filter.request.clone().unwrap() != request {
                        continue;
                    }
                } else {
                    // With request filter, skip entries without request field
                    continue;
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
    )
        -> Result<Vec<(DateTime<FixedOffset>, JsonValue)>> {
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
    )
        -> Vec<(DateTime<FixedOffset>, JsonValue)> {
        let (mut i, mut j) = (0, 0);
        let mut merged = Vec::with_capacity(a.len() + b.len());
        while i < a.len() && j < b.len() {
            if a[i].0 <= b[j].0 {
                merged.push(a[i].clone());
                i += 1;
            } else {
                merged.push(b[j].clone());
                j += 1;
            }
        }
        if i < a.len() {
            merged.extend_from_slice(&a[i..]);
        }
        if j < b.len() {
            merged.extend_from_slice(&b[j..]);
        }
        merged
    }

}

impl LoggingService {
    pub fn new(
        app_log_path: &Path,
        session_log_path: &Path,
        session_service: &SessionService,
    )
        -> Result<LoggingService> {
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

        let session_id = session_service.get_session_uuid();
        let session_path = session_log_path.join(session_id);
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
                    .event_format(standard_log_format)
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
        let merged_logs =
            LoggingService::merge_logs_chronologically(app_logs, session_logs);

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
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
                    message = payload.message
                )
            }
            LogScope::Session => {
                trace!(
                    target: SESSION_SCOPE,
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
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
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
                    message = payload.message
                )
            }
            LogScope::Session => {
                debug!(
                    target: SESSION_SCOPE,
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
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
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
                    message = payload.message
                )
            }
            LogScope::Session => {
                info!(
                    target: SESSION_SCOPE,
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
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
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
                    message = payload.message
                )
            }
            LogScope::Session => {
                warn!(
                    target: SESSION_SCOPE,
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
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
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
                    message = payload.message
                )
            }
            LogScope::Session => {
                error!(
                    target: SESSION_SCOPE,
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
                    message = payload.message
                )
            }
        }
    }
}

impl AppService for LoggingService {
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn dispose(&self) {
        // TODO: Dropping the session log folder here?
    }

    fn as_any(&self) -> &(dyn Any + Send) {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[instrument(level = "trace", skip_all)]
    async fn create_collection(path: &Path, name: &str, log_service: &LoggingService) {
        let collection_path = path.join(name);

        log_service.info(
            LogScope::Session,
            LogPayload {
                collection: Some(collection_path.clone()),
                request: None,
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
                collection: None,
                request: None,
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
                collection: Some(collection_path.to_path_buf()),
                request: Some(request_path.clone()),
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
                collection: None,
                request: None,
                message: "Successfully created request".to_string(),
            },
        )
    }

    #[instrument(level = "trace", skip_all)]
    async fn something_terrible(
        collection_path: &Path,
        request_path: &Path,
        log_service: &LoggingService,
    ) {
        log_service.warn(
            LogScope::App,
            LogPayload {
                collection: Some(collection_path.to_path_buf()),
                request: Some(request_path.to_path_buf()),
                message: "Something bad!".to_string(),
            },
        );
        log_service.error(
            LogScope::App,
            LogPayload {
                collection: Some(collection_path.to_path_buf()),
                request: Some(request_path.to_path_buf()),
                message: "Something terrible!".to_string(),
            },
        );
    }

    const TEST_SESSION_LOG_FOLDER: &'static str = "logs/session";
    const TEST_APP_LOG_FOLDER: &'static str = "logs/app";
    #[test]
    fn test() {
        let session_service = SessionService::new();
        let logging_service = LoggingService::new(
            Path::new(TEST_APP_LOG_FOLDER),
            Path::new(TEST_SESSION_LOG_FOLDER),
            &session_service,
        )
        .unwrap();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        // FIXME: Solve backslash issue
        let collection_path = Path::new("").join("TestCollection");
        let request_path = Path::new("").join("TestCollection").join("TestRequest");

        runtime.block_on(async {
            create_collection(Path::new(""), "TestCollection", &logging_service).await;
            create_request(&collection_path, "TestRequest", &logging_service).await;
            something_terrible(&collection_path, &request_path, &logging_service).await;
        });

        let input = ListLogsInput {
            dates: vec![],
            levels: vec![LogLevel::INFO],
            collection: None,
            request: None,
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
