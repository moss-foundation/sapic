mod tokens;

use crate::tokens::*;
use anyhow::Result;
use chrono::{NaiveDate, Utc};
use serde_json::Value as JSONValue;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use tracing::{event, instrument, Level};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::field::MakeExt;
use tracing_subscriber::fmt::{
    format::{FmtSpan, JsonFields},
    FormatFields,
};
use tracing_subscriber::prelude::*;

type LogEntry = JSONValue;
// Empty field means that no filter will be applied
#[derive(Default)]
struct LogFilter {
    // TODO: Should we use `DateTime` objects as range?
    dates: HashSet<NaiveDate>,
    levels: HashSet<Level>,
    collections: HashSet<PathBuf>,
    requests: HashSet<PathBuf>,
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
    pub fn add_collections(self, collections: impl IntoIterator<Item = PathBuf>) -> Self {
        Self {
            collections: self
                .collections
                .into_iter()
                .chain(collections.into_iter())
                .collect(),
            ..self
        }
    }
    pub fn add_requests(self, requests: impl IntoIterator<Item = PathBuf>) -> Self {
        Self {
            requests: self
                .requests
                .into_iter()
                .chain(requests.into_iter())
                .collect(),
            ..self
        }
    }
}

struct LoggingService {
    session_path: PathBuf,
    _guard: WorkerGuard,
}

impl LoggingService {
    pub fn init(path: &Path) -> Result<LoggingService> {
        let log_format = tracing_subscriber::fmt::format()
            .with_file(false)
            .with_line_number(false)
            .with_target(false)
            .json()
            .flatten_event(true)
            .with_current_span(true);

        let session_id = Utc::now().timestamp().to_string();
        let session_path = path.join(session_id);
        let file_appender = tracing_appender::rolling::minutely(&session_path, LOG_PREFIX);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        let subscriber = tracing_subscriber::registry().with(
            tracing_subscriber::fmt::layer()
                .event_format(log_format)
                .with_span_events(FmtSpan::CLOSE)
                .with_writer(non_blocking)
                .fmt_fields(JsonFields::default()),
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

            if !filter.collections.is_empty() {
                if let Some(collection) = value
                    .get(COLLECTION_LIT)
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                {
                    if !filter.collections.contains(&collection) {
                        continue;
                    }
                } else {
                    continue;
                }
            }

            if !filter.requests.is_empty() {
                if let Some(request) = value
                    .get(REQUEST_LIT)
                    .and_then(|v| v.as_str())
                    .map(PathBuf::from)
                {
                    if !filter.requests.contains(&request) {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            records.push(value);
        }

        Ok(())
    }

    pub fn query_with_filter(&self, filter: &LogFilter) -> Result<Vec<LogEntry>> {
        let mut result = Vec::new();
        for entry in fs::read_dir(&self.session_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                continue;
            }

            if !filter.dates.is_empty() {
                let file_date = path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .strip_prefix(format!("{}.", LOG_PREFIX).as_str())
                    .and_then(|f| NaiveDate::parse_from_str(f, "%Y-%m-%d-%H-%M").ok());

                dbg!(&file_date);
                // Parse only files with correctly formatted dates specifie in the filter
                if let Some(file_date) = file_date {
                    if !filter.dates.contains(&file_date) {
                        continue;
                    }
                } else {
                    continue;
                }
            }
            LoggingService::parse_file_with_filter(&mut result, &path, filter)?
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    #[instrument(level = "trace")]
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

    #[instrument(level = "trace")]
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

    #[instrument(level = "trace")]
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
    const TEST_MAX_FILE_SIZE: u64 = 1024 * 1024; // 1mb
    const TEST_MAX_FILE_COUNTS: usize = 10;
    #[test]
    fn test() {
        let service = LoggingService::init(Path::new(TEST_LOG_FOLDER)).unwrap();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let collection_path = Path::new("").join("TestCollection");
        let request_path = Path::new("").join("TestCollection").join("TestRequest");

        runtime.block_on(async {
            create_collection(Path::new(""), "TestCollection").await;
            create_request(&collection_path, "TestRequest").await;
            something_terrible(&collection_path, &request_path).await;
        });

        let filter = LogFilter::new()
            .add_dates(vec![Utc::now().naive_utc().into()])
            .add_levels(vec![Level::WARN, Level::ERROR])
            .add_collections(vec![collection_path.clone()])
            .add_requests(vec![request_path.clone()]);
        dbg!(service.query_with_filter(&filter).unwrap());
    }
}
