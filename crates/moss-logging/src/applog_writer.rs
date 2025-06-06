use chrono::DateTime;
use std::{
    collections::VecDeque,
    fs::OpenOptions,
    io::BufWriter,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tracing_subscriber::fmt::MakeWriter;

use crate::{FILE_DATE_FORMAT, TIMESTAMP_FORMAT, models::types::LogEntry};

pub struct AppLogMakeWriter {
    pub applog_path: PathBuf,
    pub dump_threshold: usize, // Dump the cached logs to a file
    pub applog_queue: Arc<Mutex<VecDeque<LogEntry>>>,
}

impl AppLogMakeWriter {
    pub fn new(applog_path: &Path, dump_threshold: usize) -> AppLogMakeWriter {
        Self {
            applog_path: applog_path.to_owned(),
            dump_threshold,
            applog_queue: Arc::new(Mutex::new(VecDeque::with_capacity(dump_threshold))),
        }
    }
}

pub struct AppLogWriter {
    pub applog_path: PathBuf,
    pub dump_threshold: usize, // Dump the cached logs to a file
    pub applog_queue: Arc<Mutex<VecDeque<LogEntry>>>,
}

impl<'a> std::io::Write for AppLogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let log_entry: LogEntry = serde_json::from_str(String::from_utf8_lossy(buf).as_ref())?;

        let mut queue_lock = self.applog_queue.lock().unwrap();
        while queue_lock.len() >= self.dump_threshold {
            dbg!("Dumping");
            // Use the timestamp of the oldest entry for filename
            if let Ok(datetime) =
                DateTime::parse_from_str(queue_lock[0].timestamp.as_ref(), TIMESTAMP_FORMAT)
            {
                let file_name = datetime.format(FILE_DATE_FORMAT).to_string();

                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.applog_path.join(file_name))?;
                let mut writer = BufWriter::new(file);
                while let Some(entry) = queue_lock.pop_front() {
                    serde_json::to_writer(&mut writer, &entry)?;
                    writer.write(b"\n")?;
                }
            } else {
                // Skip the first entry since its timestamp is invalid
                dbg!("Invalid timestamp");
                queue_lock.pop_front();
            }
        }
        dbg!("Pushing");

        queue_lock.push_back(log_entry);
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
            applog_path: self.applog_path.clone(),
            dump_threshold: self.dump_threshold,
            applog_queue: self.applog_queue.clone(),
        }
    }
}
