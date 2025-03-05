mod tokens;

use crate::tokens::*;
use anyhow::Result;
use chrono::{NaiveDate, Utc};
use serde_json::Value as JSONValue;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::{fs, io};
#[allow(unused_imports)]
use tracing::{event, instrument, Instrument, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::fmt::format::{FmtSpan, JsonFields};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::prelude::*;

use moss_session::SessionService;

type LogEntry = JSONValue;
// Empty field means that no filter will be applied
#[derive(Default)]
struct LogFilter {
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

// TODO: in-memory, session log, global log
struct LoggingService {
    session_path: PathBuf,
    _guard: WorkerGuard,
}

impl LoggingService {
    pub fn init(logs_path: &Path, session_service: Arc<SessionService>) -> Result<LoggingService> {
        let session_log_format = tracing_subscriber::fmt::format()
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
        let session_path = logs_path.join(session_id);
        // TODO: make `log.` suffix or get rid of it
        let file_appender = tracing_appender::rolling::Builder::new()
            .rotation(Rotation::MINUTELY)
            .filename_suffix("log")
            .build(&session_path)?;

        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        let subscriber = tracing_subscriber::registry()
            .with(
                // Session log subscriber
                tracing_subscriber::fmt::layer()
                    .event_format(session_log_format)
                    .with_timer(ChronoLocal::rfc_3339())
                    .with_writer(non_blocking)
                    .fmt_fields(JsonFields::default())
                    .with_current_span(true)
                    .with_filter(filter_fn(|metadata| metadata.level() < &Level::TRACE)),
            )
            .with(
                // Trace log subscriber
                tracing_subscriber::fmt::layer()
                    .event_format(instrument_log_format)
                    .with_timer(ChronoLocal::rfc_3339())
                    .with_span_events(FmtSpan::CLOSE)
                    .with_ansi(true)
                    .with_writer(io::stdout)
                    .with_filter(filter_fn(|metadata| metadata.level() == &Level::TRACE)),
            );

        tracing::subscriber::set_global_default(subscriber)?;
        Ok(Self {
            _guard,
            session_path,
        })
    }

    fn parse_file_with_filter(
        records: &mut Vec<LogEntry>,
        path: &Path,
        filter: &LogFilter,
    ) -> Result<()> {
        // In the log created by tracing-appender, each line is a JSON object for a LogEntry
        let file = File::open(path)?;

        for line in BufReader::new(file).lines() {
            let line = line?;
            let value: JSONValue = serde_json::from_str(&line)?;

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

    pub fn query_with_filter(&self, filter: &LogFilter) -> Result<Vec<LogEntry>> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    #[instrument(level = "trace", skip_all)]
    async fn create_collection(path: &Path, name: &str) {
        let collection_path = path.join(name);
        event!(
            Level::INFO,
            collection = collection_path.to_string_lossy().to_string(),
            message = format!(
                "Created collection {} at {}",
                name,
                collection_path.to_string_lossy().to_string()
            )
        );
    }

    #[instrument(level = "trace", skip_all)]
    async fn create_request(collection_path: &Path, name: &str) {
        let request_path = collection_path.join(name);
        event!(
            Level::INFO,
            collection = collection_path.to_string_lossy().to_string(),
            request = request_path.to_string_lossy().to_string(),
            message = format!(
                "Created request {} at {}",
                name,
                request_path.to_string_lossy().to_string()
            )
        );
    }

    #[instrument(level = "trace", skip_all)]
    async fn something_terrible(collection_path: &Path, request_path: &Path) {
        event!(
            Level::WARN,
            collection = collection_path.to_string_lossy().to_string(),
            request = request_path.to_string_lossy().to_string(),
            message = "Something bad!"
        );
        event!(
            Level::ERROR,
            collection = collection_path.to_string_lossy().to_string(),
            request = request_path.to_string_lossy().to_string(),
            message = "Something terrible!"
        )
    }

    const TEST_LOG_FOLDER: &'static str = "logs";
    #[test]
    fn test() {
        let session_service = SessionService::init();
        let logging_service =
            LoggingService::init(Path::new(TEST_LOG_FOLDER), Arc::new(session_service)).unwrap();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        // FIXME: Solve backslash issue
        let collection_path = Path::new("").join("TestCollection");
        let request_path = Path::new("").join("TestCollection").join("TestRequest");

        runtime.block_on(async {
            create_collection(Path::new(""), "TestCollection").await;
            create_request(&collection_path, "TestRequest").await;
            something_terrible(&collection_path, &request_path).await;
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
