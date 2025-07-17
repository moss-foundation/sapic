use anyhow::{Result, anyhow};
use nanoid::nanoid;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

#[derive(Copy, Clone)]
pub struct CreateOptions {
    pub overwrite: bool,
    // If true, the operation will fail when the file already exists
    pub create_new: bool,
}

#[derive(Copy, Clone)]
pub struct RenameOptions {
    pub overwrite: bool,
    pub ignore_if_exists: bool,
}

impl Default for CreateOptions {
    fn default() -> Self {
        Self {
            overwrite: true,
            create_new: false,
        }
    }
}

/// For every filesystem operation that succeeds, we push a reverse action
/// Once an operation fails, we go back through the sequence of actions
/// Which will reverse all the changes so far

/// Also, it's not easy to find which folders are created by `create_dir_all`,
/// So I will not implement it at first
pub enum Action {
    RemoveDir(PathBuf),
    RemoveFile(PathBuf),
    Restore { path: PathBuf, original: PathBuf },
}

pub struct FileSystemTransaction {
    temp_folder: PathBuf,
    rollback_sequence: Vec<Action>,
}

impl FileSystemTransaction {
    fn temp_path(&self) -> PathBuf {
        self.temp_folder.join(nanoid!(10))
    }

    /// We assume that if we can do a filesystem operation
    /// We can also do its reverse action
    /// For example, if we create an entry at a path,
    /// We should be able to delete it
    /// Similarly, if we are able to delete a path,
    /// We should be able to create at that path
    /// I'm not sure if there's a better strategy
    pub async fn rollback(&mut self) {
        while let Some(action) = self.rollback_sequence.pop() {
            match action {
                Action::RemoveDir(path) => {
                    let _ = tokio::fs::remove_dir(&path).await;
                }
                Action::RemoveFile(path) => {
                    let _ = tokio::fs::remove_file(&path).await;
                }
                Action::Restore { path, original } => {
                    let _ = tokio::fs::rename(&original, &path).await;
                }
            }
        }
    }

    pub async fn create_dir(&mut self, path: &Path) -> Result<()> {
        match tokio::fs::create_dir(path).await {
            Ok(_) => {
                self.rollback_sequence
                    .push(Action::RemoveDir(path.to_path_buf()));

                Ok(())
            }
            Err(e) => {
                self.rollback().await;
                Err(anyhow!(
                    "failed to create directory at {}: {e}",
                    path.display()
                ))
            }
        }
    }

    // FIXME: It's easier to support recursive directory deletion only here
    // Which only means that you can remove both empty and non-empty directories
    // It should be fine for our purpose,
    pub async fn remove_dir(&mut self, path: &Path) -> Result<()> {
        // Try moving the directory to be removed to a new temporary directory
        // This should have the same failure conditions as remove_dir

        // The process of backing up files and directories to be deleted is actually the same
        if !path.is_dir() {
            return Err(anyhow!("not a directory: {}", path.display()));
        }

        let temp_path = self.temp_path();
        if let Err(e) = tokio::fs::rename(path, &temp_path).await {
            self.rollback().await;
            return Err(anyhow!(
                "failed to remove a directory at {}: {e}",
                path.display()
            ));
        }
        self.rollback_sequence.push(Action::Restore {
            path: path.to_path_buf(),
            original: temp_path,
        });

        Ok(())
    }

    // We want to restore file to its previous state when undoing
    // Since create_file_with can overwrite or append to existing files
    // We need to keep their previous version to a temporary file
    pub async fn create_file_with(
        &mut self,
        path: &Path,
        content: &[u8],
        options: CreateOptions,
    ) -> Result<()> {
        let file_exists = path.exists();
        if file_exists && options.create_new {
            self.rollback().await;
            return Err(anyhow!("File already exists at {}", path.display()));
        }

        if !file_exists && !options.create_new {
            self.rollback().await;
            return Err(anyhow!("File does not exist at {}", path.display()));
        }

        let temp_path = self.temp_path();
        match (file_exists, options.overwrite) {
            (true, true) => {
                if let Err(e) = tokio::fs::copy(path, &temp_path).await {
                    self.rollback().await;
                    return Err(anyhow!(
                        "Failed to create a backup for {}: {e}",
                        path.display()
                    ));
                }

                if let Err(e) = tokio::fs::write(&path, &content).await {
                    self.rollback().await;
                    return Err(anyhow!(
                        "failed to overwrite a file at {}: {e}",
                        path.display()
                    ));
                }
                self.rollback_sequence.push(Action::Restore {
                    path: path.to_path_buf(),
                    original: temp_path.to_path_buf(),
                });
                Ok(())
            }
            (true, false) => {
                if let Err(e) = tokio::fs::copy(path, &temp_path).await {
                    self.rollback().await;
                    return Err(anyhow!(
                        "Failed to create a backup for {}: {e}",
                        path.display()
                    ));
                }

                let mut open_options = tokio::fs::OpenOptions::new();
                open_options.append(true);
                let mut file = match open_options.open(&path).await {
                    Ok(file) => file,
                    Err(e) => {
                        self.rollback().await;
                        return Err(anyhow!(
                            "failed to open {} for appending: {e}",
                            path.display()
                        ));
                    }
                };
                if let Err(e) = file.write_all(content).await {
                    self.rollback().await;
                    return Err(anyhow!(
                        "failed to append content to {}: {e}",
                        path.display()
                    ));
                }
                if let Err(e) = file.flush().await {
                    self.rollback().await;
                    return Err(anyhow!(
                        "failed to flush the content to {}: {e}",
                        path.display()
                    ));
                }
                self.rollback_sequence.push(Action::Restore {
                    path: path.to_path_buf(),
                    original: temp_path.to_path_buf(),
                });
                Ok(())
            }
            (false, _) => {
                if let Err(e) = tokio::fs::write(&path, &content).await {
                    self.rollback().await;
                    return Err(anyhow!("failed to overwrite {}: {e}", path.display()));
                }

                self.rollback_sequence
                    .push(Action::RemoveFile(path.to_path_buf()));
                Ok(())
            }
        }
    }

    // Move the file to be removed to the temporary folder
    pub async fn remove_file(&mut self, path: &Path) -> Result<()> {
        if !path.is_file() {
            return Err(anyhow!("not a file: {}", path.display()));
        }

        let temp_path = self.temp_path();
        if let Err(e) = tokio::fs::rename(path, &temp_path).await {
            self.rollback().await;
            return Err(anyhow!("failed to remove {}: {e}", path.display()));
        }
        self.rollback_sequence.push(Action::Restore {
            path: path.to_path_buf(),
            original: temp_path.to_path_buf(),
        });
        Ok(())
    }

    pub async fn rename(&mut self, from: &Path, to: &Path, options: RenameOptions) -> Result<()> {
        match (options.overwrite, to.exists()) {
            (true, _) | (false, false) => {
                // Backup the source (and destination if possible)
                let from_backup = self.temp_path();
                if let Err(e) = tokio::fs::copy(from, &from_backup).await {
                    self.rollback().await;
                    return Err(anyhow!(
                        "failed to backup the source {}: {e}",
                        from.display()
                    ));
                }
                self.rollback_sequence.push(Action::Restore {
                    path: from.to_path_buf(),
                    original: from_backup,
                });

                if to.exists() {
                    let to_backup = self.temp_path();
                    if let Err(e) = tokio::fs::rename(to, &to_backup).await {
                        self.rollback().await;
                        return Err(anyhow!(
                            "failed to backup the destination {}: {e}",
                            to.display()
                        ));
                    }
                    self.rollback_sequence.push(Action::Restore {
                        path: to.to_path_buf(),
                        original: to_backup,
                    });
                }

                // We don't need a particular rollback action for this
                // We just need to restore the backups
                if let Err(e) = tokio::fs::rename(from, to).await {
                    self.rollback().await;
                    return Err(anyhow!(
                        "failed to rename {} to {}: {e}",
                        from.display(),
                        to.display()
                    ));
                }

                Ok(())
            }
            (false, true) => {
                if options.ignore_if_exists {
                    Ok(())
                } else {
                    self.rollback().await;
                    Err(anyhow!("path {} already exists", to.display()))
                }
            }
        }
    }
}

// TODO: Allow only one transaction at a time?

pub struct AtomicFileSystem {
    temp_folder: PathBuf,
}

impl AtomicFileSystem {
    pub fn begin_transaction(&self) -> FileSystemTransaction {
        FileSystemTransaction {
            temp_folder: self.temp_folder.clone(),
            rollback_sequence: vec![],
        }
    }
}
