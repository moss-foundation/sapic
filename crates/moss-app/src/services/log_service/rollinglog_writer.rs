use chrono::DateTime;
use moss_applib::AppRuntime;
use std::{
    collections::VecDeque,
    fs::OpenOptions,
    io::BufWriter,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::{
    models::types::LogEntryInfo,
    services::{
        log_service::constants::{FILE_TIMESTAMP_FORMAT, TIMESTAMP_FORMAT},
        storage_service::StorageService,
    },
};
// log:{log_id}: log_entry_path

pub struct RollingLogWriter<R: AppRuntime> {
    pub log_path: PathBuf,
    pub dump_threshold: usize,
    pub log_queue: Arc<Mutex<VecDeque<LogEntryInfo>>>,
    pub storage: Arc<StorageService<R>>,
}

impl<R: AppRuntime> RollingLogWriter<R> {
    pub fn new(
        log_path: PathBuf,
        dump_threshold: usize,
        log_queue: Arc<Mutex<VecDeque<LogEntryInfo>>>,
        storage: Arc<StorageService<R>>,
    ) -> Self {
        Self {
            log_path,
            dump_threshold,
            log_queue,
            storage,
        }
    }
}

impl<'a, R: AppRuntime> std::io::Write for RollingLogWriter<R> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let log_entry: LogEntryInfo = serde_json::from_str(String::from_utf8_lossy(buf).as_ref())?;

        let mut queue_lock = self
            .log_queue
            .lock()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Mutex poisoned"))?;
        while queue_lock.len() >= self.dump_threshold {
            // Use the timestamp of the oldest entry for filename
            if let Ok(datetime) =
                DateTime::parse_from_str(queue_lock[0].timestamp.as_ref(), TIMESTAMP_FORMAT)
            {
                let file_name = datetime.format(FILE_TIMESTAMP_FORMAT).to_string();
                let file_path = self.log_path.join(file_name).with_extension("log");

                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file_path)?;
                let mut writer = BufWriter::new(file);

                let mut txn = self.storage.begin_write()?;

                while let Some(entry) = queue_lock.pop_front() {
                    serde_json::to_writer(&mut writer, &entry)?;
                    writer.write(b"\n")?;
                    writer.flush()?;
                    // Record the file to which the log entry is written
                    self.storage
                        .put_log_path_txn(&mut txn, &entry.id, file_path.clone())?;
                }

                txn.commit()?;
            } else {
                // Skip the first entry since its timestamp is invalid
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
