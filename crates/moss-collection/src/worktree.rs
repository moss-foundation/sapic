use anyhow::Result;
use moss_common::{api::OperationError, continue_if_err, continue_if_none};
use moss_fs::{CreateOptions, FileSystem, RemoveOptions, desanitize_path};
use moss_text::sanitized::{desanitize, sanitize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use thiserror::Error;
use tokio::{fs, sync::mpsc};
use uuid::Uuid;

use crate::models::{
    primitives::{EntryClass, EntryKind, EntryProtocol},
    types::configuration::{CompositeDirConfigurationModel, CompositeItemConfigurationModel},
};

pub mod constants {
    pub(crate) const CONFIG_FILE_NAME_ITEM: &str = "config.toml";
    pub(crate) const CONFIG_FILE_NAME_DIR: &str = "config-folder.toml";
}

#[derive(Error, Debug)]
pub enum WorktreeError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("worktree entry already exists: {0}")]
    AlreadyExists(String),

    #[error("worktree entry is not found: {0}")]
    NotFound(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl From<WorktreeError> for OperationError {
    fn from(error: WorktreeError) -> Self {
        match error {
            WorktreeError::InvalidInput(err) => OperationError::InvalidInput(err),
            WorktreeError::AlreadyExists(err) => OperationError::AlreadyExists(err),
            WorktreeError::NotFound(err) => OperationError::NotFound(err),
            WorktreeError::Unknown(err) => OperationError::Unknown(err),
            WorktreeError::Io(err) => OperationError::Internal(err.to_string()),
        }
    }
}

pub type WorktreeResult<T> = Result<T, WorktreeError>;

#[derive(Debug)]
pub struct WorktreeEntry {
    pub id: Uuid,
    pub name: String,
    pub path: Arc<Path>,
    pub class: EntryClass,
    pub kind: EntryKind,
    pub protocol: Option<EntryProtocol>,
}

pub struct Worktree {
    fs: Arc<dyn FileSystem>,
    abs_path: Arc<Path>,
}

struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    scan_queue: mpsc::UnboundedSender<ScanJob>,
}

impl Worktree {
    pub fn new(fs: Arc<dyn FileSystem>, abs_path: Arc<Path>) -> Self {
        Self { fs, abs_path }
    }

    pub fn absolutize(&self, path: &Path) -> WorktreeResult<PathBuf> {
        debug_assert!(path.is_relative());

        if path
            .components()
            .any(|c| c == std::path::Component::ParentDir)
        {
            return Err(WorktreeError::InvalidInput(format!(
                "Path cannot contain '..' components: {}",
                path.display()
            )));
        }

        if path.file_name().is_some() {
            Ok(self.abs_path.join(path))
        } else {
            Ok(self.abs_path.to_path_buf())
        }
    }

    pub async fn create_entry(
        &self,
        path: impl AsRef<Path>,
        name: &str,
        is_dir: bool,
        content: &[u8],
    ) -> WorktreeResult<()> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        let encoded_path = moss_fs::utils::sanitize_path(path, None)?.join(sanitize(name));
        let abs_path = self.absolutize(&encoded_path)?;

        if abs_path.exists() {
            return Err(WorktreeError::AlreadyExists(format!(
                "Entry already exists: {}",
                abs_path.display()
            )));
        }

        self.fs.create_dir(&abs_path).await?;

        if is_dir {
            let file_path = abs_path.join(constants::CONFIG_FILE_NAME_DIR);
            self.fs
                .create_file_with(
                    &file_path,
                    content,
                    CreateOptions {
                        overwrite: false,
                        ignore_if_exists: false,
                    },
                )
                .await?;
        } else {
            let file_path = abs_path.join(constants::CONFIG_FILE_NAME_ITEM);
            self.fs
                .create_file_with(
                    &file_path,
                    content,
                    CreateOptions {
                        overwrite: false,
                        ignore_if_exists: false,
                    },
                )
                .await?;
        }

        Ok(())
    }

    pub async fn rename_entry(&self, from: &Path, to: &Path) -> WorktreeResult<()> {
        let encoded_from = moss_fs::utils::sanitize_path(from, None)?;
        let encoded_to = moss_fs::utils::sanitize_path(to, None)?;

        let abs_from = self.absolutize(&encoded_from)?;
        let abs_to = self.absolutize(&encoded_to)?;

        if !abs_from.exists() {
            return Err(WorktreeError::NotFound(format!(
                "Entry not found: {}",
                from.display()
            )));
        }

        if abs_to.exists() {
            return Err(WorktreeError::AlreadyExists(format!(
                "Entry already exists: {}",
                to.display()
            )));
        }

        self.fs
            .rename(
                &abs_from,
                &abs_to,
                moss_fs::RenameOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        Ok(())
    }

    pub async fn remove_entry(&self, path: &Path) -> WorktreeResult<()> {
        let encoded_path = moss_fs::utils::sanitize_path(path, None)?;
        let abs_path = self.absolutize(&encoded_path)?;

        if !abs_path.exists() {
            return Err(WorktreeError::NotFound(format!(
                "Entry not found: {}",
                path.display()
            )));
        }

        self.fs
            .remove_dir(
                &abs_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        Ok(())
    }

    pub async fn scan(
        &self,
        path: &Path,
        sender: mpsc::UnboundedSender<WorktreeEntry>,
    ) -> WorktreeResult<()> {
        debug_assert!(path.is_relative());

        let path: Arc<Path> = path.into();
        let abs_path = self.absolutize(&path)?;

        let (job_tx, mut job_rx) = mpsc::unbounded_channel();

        let initial_job = ScanJob {
            abs_path: abs_path.into(),
            path: Arc::clone(&path),
            scan_queue: job_tx.clone(),
        };
        job_tx.send(initial_job).unwrap();

        drop(job_tx);

        let mut handles = Vec::new();
        while let Some(job) = job_rx.recv().await {
            let sender = sender.clone();
            let fs = self.fs.clone();
            let handle = tokio::spawn(async move {
                let mut new_jobs = Vec::new();

                let dir_name = job
                    .path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| job.path.to_string_lossy().to_string());

                match process_dir_entry(&dir_name, &job.path, &fs, &job.abs_path).await {
                    Ok(Some(dir_entry)) => {
                        let _ = sender.send(dir_entry);
                    }
                    Ok(None) => {
                        // TODO: log error
                        return;
                    }
                    Err(_err) => {
                        // TODO: log error
                        return;
                    }
                }

                let mut read_dir = match fs::read_dir(&job.abs_path).await {
                    Ok(dir) => dir,
                    Err(_) => return,
                };

                let mut child_paths = Vec::new();
                while let Ok(Some(dir_entry)) = read_dir.next_entry().await {
                    child_paths.push(dir_entry);
                }

                for child_entry in child_paths {
                    let child_file_type = continue_if_err!(child_entry.file_type().await);
                    let child_abs_path: Arc<Path> = child_entry.path().into();
                    let child_name = continue_if_none!(child_abs_path.file_name())
                        .to_string_lossy()
                        .to_string();
                    let child_path: Arc<Path> = job.path.join(&child_name).into();

                    let maybe_entry = if child_file_type.is_dir() {
                        continue_if_err!(
                            process_dir_entry(&child_name, &child_path, &fs, &child_abs_path).await
                        )
                    } else {
                        continue_if_err!(
                            process_file_entry(&child_name, &child_path, &fs, &child_abs_path)
                                .await
                        )
                    };

                    let entry = continue_if_none!(maybe_entry, || {
                        // TODO: Probably should log here since we should not be able to get here
                    });

                    if child_file_type.is_dir() {
                        // For directories, don't send here - they will be sent when their ScanJob is processed
                        // This avoids duplicate directory entries
                        new_jobs.push(ScanJob {
                            abs_path: Arc::clone(&child_abs_path),
                            path: child_path,
                            scan_queue: job.scan_queue.clone(),
                        });
                    } else {
                        // For files, send immediately since they won't have their own ScanJob
                        continue_if_err!(sender.send(entry), |_err| {
                            // TODO: log error
                        });
                    }
                }

                for new_job in new_jobs {
                    continue_if_err!(job.scan_queue.send(new_job), |_| {
                        // TODO: log error
                    });
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Err(_err) = handle.await {
                // TODO: log error
            }
        }

        Ok(())
    }
}

async fn process_dir_entry(
    name: &str,
    path: &Arc<Path>,
    fs: &Arc<dyn FileSystem>,
    abs_path: &Path,
) -> WorktreeResult<Option<WorktreeEntry>> {
    let dir_config_path = abs_path.join(constants::CONFIG_FILE_NAME_DIR);
    let item_config_path = abs_path.join(constants::CONFIG_FILE_NAME_ITEM);

    if dir_config_path.exists() {
        let config =
            parse_configuration::<CompositeDirConfigurationModel>(&fs, &dir_config_path).await?;

        return Ok(Some(WorktreeEntry {
            id: config.metadata.id,
            name: desanitize(name),
            path: desanitize_path(path, None)?.into(),
            class: config.classification(),
            kind: EntryKind::Dir,
            protocol: None,
        }));
    }

    if item_config_path.exists() {
        let config =
            parse_configuration::<CompositeItemConfigurationModel>(&fs, &item_config_path).await?;

        return Ok(Some(WorktreeEntry {
            id: config.metadata.id,
            name: desanitize(name),
            path: desanitize_path(path, None)?.into(),
            class: config.classification(),
            kind: EntryKind::Item,
            protocol: config.protocol(),
        }));
    }

    Ok(None)
}

async fn process_file_entry(
    _name: &str,
    _path: &Arc<Path>,
    _fs: &Arc<dyn FileSystem>,
    _abs_path: &Path,
) -> WorktreeResult<Option<WorktreeEntry>> {
    // TODO: implement
    Ok(None)
}

async fn parse_configuration<T>(fs: &Arc<dyn FileSystem>, path: &Path) -> WorktreeResult<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let mut reader = fs.open_file(path).await?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    Ok(toml::from_str(&buf).map_err(anyhow::Error::from)?)
}
