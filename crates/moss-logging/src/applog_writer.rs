use chrono::DateTime;
use moss_db::primitives::AnyValue;
use moss_storage::{
    global_storage::stores::AppLogCache,
    primitives::segkey::SegKey,
    storage::operations::{PutItem, Scan, Truncate},
};
use std::{
    fs::OpenOptions,
    io::{BufWriter, ErrorKind},
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use tracing_subscriber::fmt::MakeWriter;

use crate::{FILE_DATE_FORMAT, TIMESTAMP_FORMAT, models::types::LogEntry};

pub struct AppLogMakeWriter {
    pub applog_cache: Arc<dyn AppLogCache>,
    pub applog_path: PathBuf,
    pub cache_counter: Arc<AtomicUsize>,
    pub dump_threshold: usize, // Dump the cached logs to a file
}

pub struct AppLogWriter {
    pub applog_cache: Arc<dyn AppLogCache>,
    pub applog_path: PathBuf,
    pub cache_counter: Arc<AtomicUsize>,
    pub dump_threshold: usize, // Dump the cached logs to a file
}

impl<'a> std::io::Write for AppLogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let log_entry: LogEntry = serde_json::from_str(String::from_utf8_lossy(buf).as_ref())?;
        let cache = self.applog_cache.clone();
        if self.cache_counter.fetch_add(1, Ordering::SeqCst) >= self.dump_threshold {
            dbg!("Dumping");
            // Reset the counter and dump all cached logs to a file
            self.cache_counter.store(0, Ordering::SeqCst);
            let mut cached_logs = Scan::scan(cache.as_ref())
                .map_err(|_| {
                    std::io::Error::new(ErrorKind::Other, "Failed to scan AppLog cache".to_string())
                })?
                .into_iter()
                .filter_map(|(k, v)| {
                    let result: Result<LogEntry, _> = v.deserialize();
                    return if let Ok(entry) = result {
                        let timestamp =
                            DateTime::parse_from_str(&entry.timestamp, TIMESTAMP_FORMAT);
                        if let Ok(timestamp) = timestamp {
                            Some((timestamp, entry))
                        } else {
                            // Skip log with invalid timestamp
                            None
                        }
                    } else {
                        // Skip log that cannot be parsed
                        None
                    };
                })
                .collect::<Vec<(_, _)>>();
            Truncate::truncate(cache.as_ref()).map_err(|_| {
                std::io::Error::new(
                    ErrorKind::Other,
                    "Failed to truncate AppLog cache".to_string(),
                )
            })?;

            cached_logs.sort_by(|a, b| a.0.cmp(&b.0));

            // Write (or append) the json-serialized log entries to the file
            let file_name = cached_logs[0].0.format(FILE_DATE_FORMAT).to_string();
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.applog_path.join(file_name))?;
            let mut writer = BufWriter::new(file);
            for (_, entry) in cached_logs {
                serde_json::to_writer(&mut writer, &entry)?;
                writer.write(b"\n")?;
            }
            serde_json::to_writer(&mut writer, &log_entry)?;
        } else {
            dbg!("Inserting");
            // Insert log_entry into the database cache
            let key = SegKey::new(log_entry.id.as_str()).to_segkey_buf();
            PutItem::put(cache.as_ref(), key, AnyValue::serialize(&log_entry)?).map_err(|_| {
                std::io::Error::new(
                    ErrorKind::Other,
                    "Failed to put AppLog entry into database".to_string(),
                )
            })?;
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for AppLogMakeWriter {
    type Writer = AppLogWriter;

    fn make_writer(&'a self) -> Self::Writer {
        AppLogWriter {
            applog_cache: self.applog_cache.clone(),
            applog_path: self.applog_path.clone(),
            cache_counter: self.cache_counter.clone(),
            dump_threshold: self.dump_threshold,
        }
    }
}
