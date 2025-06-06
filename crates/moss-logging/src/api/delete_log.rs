use crate::{
    FILE_TIME_FORMAT, LoggingService, TIMESTAMP_FORMAT,
    models::{
        operations::{DeleteLogInput, DeleteLogOutput},
        types::LogEntryInfo,
    },
};
use anyhow::Result;
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use moss_common::api::{OperationError, OperationResult};
use moss_fs::CreateOptions;
use std::{
    fs::{File, read_to_string},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

impl LoggingService {
    pub async fn delete_log(&self, input: &DeleteLogInput) -> OperationResult<DeleteLogOutput> {
        let datetime =
            DateTime::parse_from_str(&input.timestamp, TIMESTAMP_FORMAT).map_err(|_| {
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
            let log_files = self.identify_log_file(&self.applog_path, datetime).await?;
            dbg!(&log_files);
            for file in log_files {
                let updated = self.update_log_file(&file, &input.id).await?;
                if updated {
                    return Ok(DeleteLogOutput {
                        id: input.id.clone(),
                        file_path: Some(file),
                    });
                }
            }
        }
        {
            let log_files = self
                .identify_log_file(&self.sessionlog_path, datetime)
                .await?;
            for file in log_files {
                let updated = self.update_log_file(&file, &input.id).await?;
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
    async fn identify_log_file(
        &self,
        path: &Path,
        datetime: DateTime<FixedOffset>,
    ) -> Result<Vec<PathBuf>> {
        // Use timestamp to identify which file to update
        // We just need to check the last two log files that starts before the log entry's timestamp
        // Two because of potential timestamp rounding issue

        // OPTIMIZE: We might use a binary search here but I'm not sure if it's necessary
        let mut file_list = Vec::new();

        let mut read_dir = self.fs.read_dir(path).await?;
        while let Some(entry) = read_dir.next_entry().await.unwrap_or(None) {
            dbg!(&entry);
            let path = entry.path();
            if !path.is_file() || path.file_stem().is_none() {
                continue;
            }
            let stem = path.file_stem().unwrap().to_string_lossy().to_string();
            // example: 2025-06-06-18-00.log
            if let Ok(dt) = NaiveDateTime::parse_from_str(&stem, FILE_TIME_FORMAT) {
                dbg!(&path, &dt);
                file_list.push((path, dt));
            }
            // Skip a log file if its name is not well-formatted
        }

        // Sort the log files chronologically and find the ones that might contain the entry
        file_list.sort_by(|a, b| a.1.cmp(&b.1));
        let files = file_list
            .into_iter()
            .filter(|item| item.1 <= datetime.naive_local())
            .rev()
            .take(2)
            .map(|item| item.0)
            .collect::<Vec<_>>();
        Ok(files)
    }

    async fn update_log_file(&self, path: &Path, id: &str) -> Result<bool> {
        // Try to delete the entry with given id
        // If deleted, return Ok(true)
        let mut new_content = String::new();
        let mut deleted = false;

        {
            let f = self.fs.open_file(path).await?;
            let reader = BufReader::new(f);
            for line in reader.lines() {
                let line = line?;
                let log_entry: LogEntryInfo = serde_json::from_str(&line)?;
                dbg!(&log_entry);
                if log_entry.id == id {
                    // Splice this line from the output content
                    deleted = true;
                } else {
                    new_content.push_str(&line);
                    new_content.push('\n');
                }
            }
        }

        // We don't need to update the file if no deletion is made
        if deleted {
            self.fs
                .create_file_with(
                    path,
                    new_content.as_bytes(),
                    CreateOptions {
                        overwrite: true,
                        ignore_if_exists: true,
                    },
                )
                .await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
