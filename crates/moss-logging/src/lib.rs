use anyhow::Result;
use chrono::NaiveDate;
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

use moss_app::service::AppService;
use moss_session::SessionService;

pub const LEVEL_LIT: &'static str = "level";
pub const COLLECTION_LIT: &'static str = "collection";
pub const REQUEST_LIT: &'static str = "request";

// Empty field means that no filter will be applied
#[derive(Default)]
pub struct LogFilter {
    dates: HashSet<NaiveDate>,
    levels: HashSet<Level>,
    collection: Option<PathBuf>,
    request: Option<PathBuf>,
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
    pub fn new(
        app_log_path: &Path,
        session_log_path: &Path,
        session_service: &SessionService,
    ) -> Result<LoggingService> {
        let standard_log_format = tracing_subscriber::fmt::format()
            .with_file(false)
            .with_line_number(false)
            .with_target(false)
            .with_timer(ChronoLocal::rfc_3339())
            .json()
            .flatten_event(true)
            .with_current_span(true);

        let instrument_log_format = tracing_subscriber::fmt::format()
            .with_file(true)
            .with_line_number(true)
            .with_target(false)
            .with_timer(ChronoLocal::rfc_3339())
            .compact()
            .with_ansi(true);

        let session_id = session_service.get_session_uuid();
        let session_path = session_log_path.join(session_id);
        // TODO: make `log.` suffix or get rid of it
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

        let timestamp_format = "%Y-%m-%dT%H:%M:%S%.3f%z".to_string();

        let subscriber = tracing_subscriber::registry()
            .with(
                // Session log subscriber
                tracing_subscriber::fmt::layer()
                    .event_format(standard_log_format.clone())
                    .with_timer(ChronoLocal::new(timestamp_format.clone()))
                    .with_writer(session_log_writer)
                    .fmt_fields(JsonFields::default())
                    .with_filter(filter_fn(|metadata| {
                        metadata.level() < &Level::TRACE && metadata.target() == "session"
                    })),
            )
            .with(
                // App log subscriber
                tracing_subscriber::fmt::layer()
                    .event_format(standard_log_format)
                    .with_timer(ChronoLocal::new(timestamp_format.clone()))
                    .with_writer(app_log_writer)
                    .fmt_fields(JsonFields::default())
                    .with_filter(filter_fn(|metadata| {
                        metadata.level() < &Level::TRACE && metadata.target() == "app"
                    })),
            )
            .with(
                // Showing all logs (including span events) to the console
                tracing_subscriber::fmt::layer()
                    .event_format(instrument_log_format)
                    .with_timer(ChronoLocal::new(timestamp_format))
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

    fn parse_file_with_filter(
        records: &mut Vec<JsonValue>,
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

            records.push(value);
        }

        Ok(())
    }

    // TODO: Add query app log, right now this is querying session log
    // Although it's better to do that when we understand what app log would entail
    // Maybe it will need a different set of filters

    pub fn query_with_filter(&self, filter: &LogFilter) -> Result<Vec<JsonValue>> {
        let mut result = Vec::new();
        let mut paths = Vec::new();
        for entry in fs::read_dir(&self.session_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() || path.extension() != Some(OsStr::new("log")) {
                continue;
            }

            let file_date = NaiveDate::parse_from_str(
                &path.file_stem().unwrap().to_string_lossy().to_string(),
                "%Y-%m-%d-%H-%M",
            )?;

            paths.push((path, file_date));
        }
        paths.sort_by_key(|p| p.1);

        for (path, date_time) in &paths {
            if filter.dates.contains(date_time) {
                LoggingService::parse_file_with_filter(&mut result, path, filter)?
            }
        }

        Ok(result)
    }

    // Tracing disallows non-constant value for `target`
    // So we have to manually match it
    pub fn trace(&self, scope: LogScope, payload: LogPayload) {
        match scope {
            LogScope::App => {
                trace!(
                    target: "app",
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
                    message = payload.message
                )
            }
            LogScope::Session => {
                trace!(
                    target: "session",
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
                    target: "app",
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
                    message = payload.message
                )
            }
            LogScope::Session => {
                debug!(
                    target: "session",
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
                    target: "app",
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
                    message = payload.message
                )
            }
            LogScope::Session => {
                info!(
                    target: "session",
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
                    target: "app",
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
                    message = payload.message
                )
            }
            LogScope::Session => {
                warn!(
                    target: "session",
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
                    target: "app",
                    collection = payload.collection.map(|p| p.display().to_string()).unwrap_or_default(),
                    request = payload.request.map(|p| p.display().to_string()).unwrap_or_default(),
                    message = payload.message
                )
            }
            LogScope::Session => {
                error!(
                    target: "session",
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

        let filter = LogFilter::new()
            .select_collection(&collection_path)
            .select_request(&request_path)
            .add_dates(vec![Utc::now().naive_utc().into()])
            .add_levels(vec![Level::WARN, Level::ERROR]);

        let output = logging_service
            .query_with_filter(&filter)
            .unwrap()
            .iter()
            .map(|entry| serde_json::to_string_pretty(entry).unwrap())
            .collect::<Vec<_>>()
            .join("\n");

        fs::write("logs/filtered", output).unwrap();
    }
}
