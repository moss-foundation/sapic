use anyhow::Result;
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use moss_common::api::{OperationError, OperationResult};
use std::{
    fs::{File, read_to_string},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{
    FILE_DATE_FORMAT, LoggingService, TIMESTAMP_FORMAT,
    models::{
        operations::{DeleteLogInput, DeleteLogOutput},
        types::LogEntryInfo,
    },
};

impl LoggingService {
    pub fn delete_log(&self, input: &DeleteLogInput) -> OperationResult<DeleteLogOutput> {
        let datetime =
            DateTime::parse_from_str(&input.timestamp, TIMESTAMP_FORMAT).map_err(|err| {
                OperationError::InvalidInput("The input timestamp is invalid".to_string())
            })?;
        {
            let mut applog_queue_lock = self.applog_queue.lock();
            let idx = applog_queue_lock.iter().position(|x| x.id == input.id);
            if let Some(idx) = idx {
                applog_queue_lock.remove(idx);
                return Ok(DeleteLogOutput {
                    id: input.id.clone(),
                    file_path: None,
                });
            }
        }
        {
            let mut sessionlog_queue_lock = self.sessionlog_queue.lock();
            let idx = sessionlog_queue_lock.iter().position(|x| x.id == input.id);
            if let Some(idx) = idx {
                sessionlog_queue_lock.remove(idx);
                return Ok(DeleteLogOutput {
                    id: input.id.clone(),
                    file_path: None,
                });
            }
        }
        {
            let log_files = Self::identify_log_file(&self.applog_path, datetime)?;
            for file in log_files {
                let updated = Self::update_log_file(&file, &input.id)?;
                if updated {
                    return Ok(DeleteLogOutput {
                        id: input.id.clone(),
                        file_path: Some(file),
                    });
                }
            }
        }
        {
            let log_files = Self::identify_log_file(&self.sessionlog_path, datetime)?;
            for file in log_files {
                let updated = Self::update_log_file(&file, &input.id)?;
                if updated {
                    return Ok(DeleteLogOutput {
                        id: input.id.clone(),
                        file_path: Some(file),
                    });
                }
            }
        }

        Err(OperationError::NotFound(format!(
            "Log id {} not found",
            input.id
        )))
    }
}

impl LoggingService {
    fn identify_log_file(path: &Path, datetime: DateTime<FixedOffset>) -> Result<Vec<PathBuf>> {
        // Use timestamp to identify which file to update
        // We just need to check the last two log files that starts before the log entry's timestamp
        // Two because of potential timestamp rounding issue

        // OPTIMIZE: We might use a binary search here but I'm not sure if it's necessary
        let mut file_list = Vec::new();
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(stem) = path.file_stem() {
                let stem = stem.to_string_lossy().to_string();
                let dt = NaiveDateTime::parse_from_str(&stem, FILE_DATE_FORMAT)?;
                file_list.push((stem, dt))
            }
        }
        file_list.sort_by(|a, b| a.1.cmp(&b.1));
        let files = file_list
            .into_iter()
            .filter(|item| item.1 <= datetime.naive_local())
            .rev()
            .take(2)
            .map(|item| path.join(item.0).with_extension("log"))
            .collect::<Vec<_>>();
        Ok(files)
    }

    fn update_log_file(path: &Path, id: &str) -> Result<bool> {
        // Try to delete the entry with given id
        // If deleted, return Ok(true)
        let old_content = read_to_string(&path)?;
        let mut new_content = String::with_capacity(old_content.len());
        let mut deleted = false;
        for line in old_content.lines() {
            let log_entry: LogEntryInfo = serde_json::from_str(line)?;
            if log_entry.id == id {
                deleted = true;
            } else {
                new_content.push_str(line);
                new_content.push('\n');
            }
        }
        // We don't need to update the file if no deletion is made
        if deleted {
            let mut f = File::options().write(true).truncate(true).open(path)?;
            f.write_all(new_content.as_bytes())?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
