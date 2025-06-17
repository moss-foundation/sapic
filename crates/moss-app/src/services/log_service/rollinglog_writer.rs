use chrono::DateTime;
use moss_db::{DatabaseError, primitives::AnyValue};
use moss_storage::{
    GlobalStorage, primitives::segkey::SegKey, storage::operations::TransactionalPutItem,
};
use parking_lot::Mutex;
use std::{
    collections::VecDeque,
    fs::OpenOptions,
    io::{BufWriter, ErrorKind},
    path::PathBuf,
    sync::Arc,
};

use crate::{
    models::types::LogEntryInfo,
    services::log_service::constants::{FILE_TIMESTAMP_FORMAT, TIMESTAMP_FORMAT},
};
// log:{log_id}: log_entry_path

pub struct RollingLogWriter {
    pub log_path: PathBuf,
    pub dump_threshold: usize,
    pub log_queue: Arc<Mutex<VecDeque<LogEntryInfo>>>,
    pub storage: Arc<dyn GlobalStorage>,
}

impl RollingLogWriter {
    pub fn new(
        log_path: PathBuf,
        dump_threshold: usize,
        log_queue: Arc<Mutex<VecDeque<LogEntryInfo>>>,
        storage: Arc<dyn GlobalStorage>,
    ) -> Self {
        Self {
            log_path,
            dump_threshold,
            log_queue,
            storage,
        }
    }
}

fn map_database_error_to_io_error(error: DatabaseError) -> std::io::Error {
    std::io::Error::new(
        ErrorKind::Other,
        format!("Database operation failed: {}", error),
    )
}

impl<'a> std::io::Write for RollingLogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let log_entry: LogEntryInfo = serde_json::from_str(String::from_utf8_lossy(buf).as_ref())?;

        let mut queue_lock = self.log_queue.lock();
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

                let mut write_txn = self
                    .storage
                    .begin_write()
                    .map_err(map_database_error_to_io_error)?;

                let log_store = self.storage.log_store();

                while let Some(entry) = queue_lock.pop_front() {
                    serde_json::to_writer(&mut writer, &entry)?;
                    writer.write(b"\n")?;
                    writer.flush()?;
                    // Record the file to which the log entry is written
                    let segkey = SegKey::new(&entry.id).to_segkey_buf();
                    let value = AnyValue::serialize(&file_path)?;

                    TransactionalPutItem::put(log_store.as_ref(), &mut write_txn, segkey, value)
                        .map_err(map_database_error_to_io_error)?;
                }
                write_txn.commit().map_err(map_database_error_to_io_error)?;
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
