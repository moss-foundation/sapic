use crate::{FILE_DATE_FORMAT, TIMESTAMP_FORMAT, models::types::LogEntry};
use chrono::DateTime;
use std::{
    collections::VecDeque,
    fs::OpenOptions,
    io::BufWriter,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tracing_subscriber::fmt::MakeWriter;

pub struct SessionLogMakeWriter {
    pub sessionlog_path: PathBuf,
    pub dump_threshold: usize,
    pub sessionlog_queue: Arc<Mutex<VecDeque<LogEntry>>>,
}

impl SessionLogMakeWriter {
    pub fn new(
        sessionlog_path: &Path,
        dump_threshold: usize,
        sessionlog_queue: Arc<Mutex<VecDeque<LogEntry>>>,
    ) -> SessionLogMakeWriter {
        Self {
            sessionlog_path: sessionlog_path.to_owned(),
            dump_threshold,
            sessionlog_queue,
        }
    }
}

pub struct SessionLogWriter {
    pub sessionlog_path: PathBuf,
    pub dump_threshold: usize,
    pub sessionlog_queue: Arc<Mutex<VecDeque<LogEntry>>>,
}

impl<'a> std::io::Write for SessionLogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let log_entry: LogEntry = serde_json::from_str(String::from_utf8_lossy(buf).as_ref())?;

        let mut queue_lock = self.sessionlog_queue.lock().unwrap();
        while queue_lock.len() >= self.dump_threshold {
            // Use the timestamp of the oldest entry for filename
            if let Ok(datetime) =
                DateTime::parse_from_str(queue_lock[0].timestamp.as_ref(), TIMESTAMP_FORMAT)
            {
                let file_name = datetime.format(FILE_DATE_FORMAT).to_string();

                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.sessionlog_path.join(file_name))?;
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
        queue_lock.push_back(log_entry);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for SessionLogMakeWriter {
    type Writer = SessionLogWriter;

    fn make_writer(&'a self) -> Self::Writer {
        SessionLogWriter {
            sessionlog_path: self.sessionlog_path.clone(),
            dump_threshold: self.dump_threshold,
            sessionlog_queue: self.sessionlog_queue.clone(),
        }
    }
}
