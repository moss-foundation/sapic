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
pub enum Undo {
    RemoveDir(PathBuf),
    RemoveFile(PathBuf),
    Restore { path: PathBuf, original: PathBuf },
}

pub enum Action {
    CreateDir(PathBuf),
    RemoveDir(PathBuf),
    CreateFileWith {
        path: PathBuf,
        content: Vec<u8>,
        options: CreateOptions,
    },
    RemoveFile(PathBuf),
    Rename {
        from: PathBuf,
        to: PathBuf,
        options: RenameOptions,
    },
}
pub struct FileSystemTransaction {
    temp_folder: PathBuf,
    actions: Vec<Action>,
}

// TODO: Allow only one transaction at a time?

pub struct AtomicFileSystem {}

impl AtomicFileSystem {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_dir_txn(&self, txn: &mut FileSystemTransaction, path: &Path) {
        txn.actions.push(Action::CreateDir(path.to_path_buf()));
    }

    pub fn remove_dir_txn(&self, txn: &mut FileSystemTransaction, path: &Path) {
        txn.actions.push(Action::RemoveDir(path.to_path_buf()));
    }

    pub fn create_file_with_txn(
        &mut self,
        txn: &mut FileSystemTransaction,
        path: &Path,
        content: &[u8],
        options: CreateOptions,
    ) {
        txn.actions.push(Action::CreateFileWith {
            path: path.to_path_buf(),
            content: content.to_vec(),
            options,
        })
    }

    pub fn remove_file_txn(&self, txn: &mut FileSystemTransaction, path: &Path) {
        txn.actions.push(Action::RemoveFile(path.to_path_buf()));
    }

    pub fn rename_txn(
        &self,
        txn: &mut FileSystemTransaction,
        from: &Path,
        to: &Path,
        options: RenameOptions,
    ) {
        txn.actions.push(Action::Rename {
            from: from.to_path_buf(),
            to: to.to_path_buf(),
            options,
        });
    }
}

impl AtomicFileSystem {
    pub fn begin_transaction(&self, temp_folder: &Path) -> FileSystemTransaction {
        FileSystemTransaction {
            temp_folder: temp_folder.to_path_buf(),
            actions: Vec::new(),
        }
    }
    pub async fn finish_transaction(&self, txn: FileSystemTransaction) -> Result<()> {
        let mut undo_stack = Vec::new();
        for action in &txn.actions {
            let action_result = match action {
                Action::CreateDir(path) => create_dir_action(path).await,

                Action::RemoveDir(path) => remove_dir_action(path, &txn.temp_folder).await,
                Action::CreateFileWith {
                    path,
                    content,
                    options,
                } => create_file_with_action(path, content, *options, &txn.temp_folder).await,
                Action::RemoveFile(path) => remove_file_action(path, &txn.temp_folder).await,
                Action::Rename { from, to, options } => {
                    rename_action(from, to, *options, &txn.temp_folder).await
                }
            };
            // Once an action failed, we roll back all the changes so far
            match action_result {
                Ok(undos) => {
                    undo_stack.extend(undos);
                }
                Err(e) => {
                    rollback(&undo_stack).await;
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}

fn temp_path(temp_dir: &Path) -> PathBuf {
    temp_dir.join(nanoid!(10))
}

async fn rollback(undo_stack: &[Undo]) {
    while let Some(action) = undo_stack.iter().rev().next() {
        match action {
            Undo::RemoveDir(path) => {
                let _ = tokio::fs::remove_dir(&path).await;
            }
            Undo::RemoveFile(path) => {
                let _ = tokio::fs::remove_file(&path).await;
            }
            Undo::Restore { path, original } => {
                let _ = tokio::fs::rename(&original, &path).await;
            }
        }
    }
}

async fn create_dir_action(path: &Path) -> Result<Vec<Undo>> {
    if let Err(e) = tokio::fs::create_dir(path).await {
        return Err(anyhow!(
            "failed to create directory at {}: {e}",
            path.display()
        ));
    } else {
        Ok(vec![Undo::RemoveDir(path.to_path_buf())])
    }
}

// FIXME: It's easier to support recursive directory deletion only here
// Which only means that you can remove both empty and non-empty directories
// It should be fine for our purpose,

async fn remove_dir_action(path: &Path, temp_dir: &Path) -> Result<Vec<Undo>> {
    // Try moving the directory to be removed to a new temporary directory
    // This should have the same failure conditions as remove_dir

    // The process of backing up files and directories to be deleted is actually the same
    if !path.is_dir() {
        return Err(anyhow!("not a directory: {}", path.display()));
    }

    let temp_path = temp_path(temp_dir);
    if let Err(e) = tokio::fs::rename(path, &temp_path).await {
        return Err(anyhow!(
            "failed to remove a directory at {}: {e}",
            path.display()
        ));
    }
    Ok(vec![Undo::Restore {
        path: path.to_path_buf(),
        original: temp_path,
    }])
}

async fn create_file_with_action(
    path: &Path,
    content: &[u8],
    options: CreateOptions,
    temp_dir: &Path,
) -> Result<Vec<Undo>> {
    let file_exists = path.exists();
    if file_exists && options.create_new {
        return Err(anyhow!("File already exists at {}", path.display()));
    }

    if !file_exists && !options.create_new {
        return Err(anyhow!("File does not exist at {}", path.display()));
    }

    let temp_path = temp_path(temp_dir);
    match (file_exists, options.overwrite) {
        (true, true) => {
            // Backup the existing file content and overwrite it
            if let Err(e) = tokio::fs::copy(path, &temp_path).await {
                return Err(anyhow!(
                    "Failed to create a backup for {}: {e}",
                    path.display()
                ));
            }

            if let Err(e) = tokio::fs::write(&path, &content).await {
                return Err(anyhow!(
                    "failed to overwrite a file at {}: {e}",
                    path.display()
                ));
            }
            Ok(vec![Undo::Restore {
                path: path.to_path_buf(),
                original: temp_path.to_path_buf(),
            }])
        }
        (true, false) => {
            // Backup the existing file content and append to it
            if let Err(e) = tokio::fs::copy(path, &temp_path).await {
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
                    return Err(anyhow!(
                        "failed to open {} for appending: {e}",
                        path.display()
                    ));
                }
            };
            if let Err(e) = file.write_all(content).await {
                return Err(anyhow!(
                    "failed to append content to {}: {e}",
                    path.display()
                ));
            }
            if let Err(e) = file.flush().await {
                return Err(anyhow!(
                    "failed to flush the content to {}: {e}",
                    path.display()
                ));
            }
            Ok(vec![Undo::Restore {
                path: path.to_path_buf(),
                original: temp_path.to_path_buf(),
            }])
        }
        (false, _) => {
            if let Err(e) = tokio::fs::write(&path, &content).await {
                return Err(anyhow!("failed to overwrite {}: {e}", path.display()));
            }
            Ok(vec![Undo::RemoveFile(path.to_path_buf())])
        }
    }
}

async fn remove_file_action(path: &Path, temp_dir: &Path) -> Result<Vec<Undo>> {
    if !path.is_file() {
        return Err(anyhow!("not a file: {}", path.display()));
    }

    let temp_path = temp_path(temp_dir);
    if let Err(e) = tokio::fs::rename(path, &temp_path).await {
        return Err(anyhow!("failed to remove {}: {e}", path.display()));
    }
    Ok(vec![Undo::Restore {
        path: path.to_path_buf(),
        original: temp_path.to_path_buf(),
    }])
}

async fn rename_action(
    from: &Path,
    to: &Path,
    options: RenameOptions,
    temp_dir: &Path,
) -> Result<Vec<Undo>> {
    let mut undo_stack = Vec::new();
    match (options.overwrite, to.exists()) {
        (true, _) | (false, false) => {
            // Backup the source
            let from_backup = temp_path(temp_dir);
            if let Err(e) = tokio::fs::copy(from, &from_backup).await {
                return Err(anyhow!(
                    "failed to backup the source {}: {e}",
                    from.display()
                ));
            }
            undo_stack.push(Undo::Restore {
                path: from.to_path_buf(),
                original: from_backup,
            });

            // Back up the destination if necessary
            if to.exists() {
                let to_backup = temp_path(temp_dir);
                if let Err(e) = tokio::fs::rename(to, &to_backup).await {
                    return Err(anyhow!(
                        "failed to backup the destination {}: {e}",
                        to.display()
                    ));
                }
                undo_stack.push(Undo::Restore {
                    path: to.to_path_buf(),
                    original: to_backup,
                });
            }

            // We don't need a particular undo action for this
            // We just need to restore the backups
            if let Err(e) = tokio::fs::rename(from, to).await {
                return Err(anyhow!(
                    "failed to rename {} to {}: {e}",
                    from.display(),
                    to.display()
                ));
            }

            Ok(undo_stack)
        }
        (false, true) => {
            if options.ignore_if_exists {
                // Skip when file exists
                Ok(Vec::new())
            } else {
                Err(anyhow!("path {} already exists", to.display()))
            }
        }
    }
}
