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
pub enum Undo {
    RemoveDir(PathBuf),
    RemoveFile(PathBuf),
    Restore { path: PathBuf, original: PathBuf },
}

pub enum Action {
    CreateDir(PathBuf),
    CreateDirAll(PathBuf),
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

pub struct Rollback {
    temp: PathBuf,
    actions: Vec<Action>,
}

impl Drop for Rollback {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.temp);
    }
}

impl Rollback {
    pub fn new(temp: impl AsRef<Path>) -> Self {
        Self {
            temp: temp.as_ref().to_path_buf(),
            actions: Vec::new(),
        }
    }
}

pub fn create_dir(rb: &mut Rollback, path: impl AsRef<Path>) {
    rb.actions
        .push(Action::CreateDir(path.as_ref().to_path_buf()));
}

pub fn remove_dir(rb: &mut Rollback, path: impl AsRef<Path>) {
    rb.actions
        .push(Action::RemoveDir(path.as_ref().to_path_buf()));
}

pub fn create_file_with(
    rb: &mut Rollback,
    path: impl AsRef<Path>,
    content: &[u8],
    options: CreateOptions,
) {
    rb.actions.push(Action::CreateFileWith {
        path: path.as_ref().to_path_buf(),
        content: content.to_vec(),
        options,
    })
}

pub fn remove_file(rb: &mut Rollback, path: impl AsRef<Path>) {
    rb.actions
        .push(Action::RemoveFile(path.as_ref().to_path_buf()));
}

pub fn rename(
    rb: &mut Rollback,
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    options: RenameOptions,
) {
    rb.actions.push(Action::Rename {
        from: from.as_ref().to_path_buf(),
        to: to.as_ref().to_path_buf(),
        options,
    })
}

pub async fn apply(rb: Rollback) -> Result<()> {
    let mut undo_stack = Vec::new();
    for action in &rb.actions {
        let action_result = match action {
            Action::CreateDir(path) => create_dir_action(path).await,
            Action::CreateDirAll(path) => create_dir_all_action(path).await,
            Action::RemoveDir(path) => remove_dir_action(path, &rb.temp).await,
            Action::CreateFileWith {
                path,
                content,
                options,
            } => create_file_with_action(path, content, *options, &rb.temp).await,
            Action::RemoveFile(path) => remove_file_action(path, &rb.temp).await,
            Action::Rename { from, to, options } => {
                rename_action(from, to, *options, &rb.temp).await
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

fn temp_path(temp_dir: impl AsRef<Path>) -> PathBuf {
    temp_dir.as_ref().join(nanoid!(10))
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

async fn create_dir_action(path: impl AsRef<Path>) -> Result<Vec<Undo>> {
    let path = path.as_ref();
    if let Err(e) = tokio::fs::create_dir(path).await {
        return Err(anyhow!(
            "failed to create directory at {}: {e}",
            path.display()
        ));
    } else {
        Ok(vec![Undo::RemoveDir(path.to_path_buf())])
    }
}

async fn create_dir_all_action(path: impl AsRef<Path>) -> Result<Vec<Undo>> {
    let path = path.as_ref();
    let mut result = Vec::new();
    let missing_paths = path
        .ancestors()
        .skip_while(|p| p.exists())
        .collect::<Vec<_>>();

    if let Err(e) = tokio::fs::create_dir_all(path).await {
        return Err(anyhow!(
            "failed to create directory up to {}: {e}",
            path.display()
        ));
    } else {
        Ok(missing_paths
            .into_iter()
            .map(|p| Undo::RemoveFile(p.to_path_buf()))
            .collect())
    }
}

// FIXME: It's easier to support recursive directory deletion only here
// Which only means that you can remove both empty and non-empty directories
// It should be fine for our purpose,

async fn remove_dir_action(path: impl AsRef<Path>, temp: impl AsRef<Path>) -> Result<Vec<Undo>> {
    // Try moving the directory to be removed to a new temporary directory
    // This should have the same failure conditions as remove_dir

    // The process of backing up files and directories to be deleted is actually the same
    let path = path.as_ref();
    let temp = temp.as_ref();
    if !path.is_dir() {
        return Err(anyhow!("not a directory: {}", path.display()));
    }

    let backup = temp_path(temp);
    if let Err(e) = tokio::fs::rename(path, &backup).await {
        return Err(anyhow!(
            "failed to remove a directory at {}: {e}",
            path.display()
        ));
    }
    Ok(vec![Undo::Restore {
        path: path.to_path_buf(),
        original: backup,
    }])
}

async fn create_file_with_action(
    path: impl AsRef<Path>,
    content: &[u8],
    options: CreateOptions,
    temp: impl AsRef<Path>,
) -> Result<Vec<Undo>> {
    let path = path.as_ref();
    let temp = temp.as_ref();

    let file_exists = path.exists();
    if file_exists && options.create_new {
        return Err(anyhow!("File already exists at {}", path.display()));
    }

    if !file_exists && !options.create_new {
        return Err(anyhow!("File does not exist at {}", path.display()));
    }

    let backup = temp_path(temp);
    match (file_exists, options.overwrite) {
        (true, true) => {
            // Backup the existing file content and overwrite it
            if let Err(e) = tokio::fs::copy(path, &backup).await {
                return Err(anyhow!(
                    "Failed to create a backup for {}: {e}",
                    path.display()
                ));
            }

            if let Err(e) = tokio::fs::write(path, &content).await {
                return Err(anyhow!(
                    "failed to overwrite a file at {}: {e}",
                    path.display()
                ));
            }
            Ok(vec![Undo::Restore {
                path: path.to_path_buf(),
                original: backup.to_path_buf(),
            }])
        }
        (true, false) => {
            // Backup the existing file content and append to it
            if let Err(e) = tokio::fs::copy(path, &backup).await {
                return Err(anyhow!(
                    "Failed to create a backup for {}: {e}",
                    path.display()
                ));
            }

            let mut open_options = tokio::fs::OpenOptions::new();
            open_options.append(true);
            let mut file = match open_options.open(path).await {
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
                original: backup.to_path_buf(),
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

async fn remove_file_action(path: impl AsRef<Path>, temp: impl AsRef<Path>) -> Result<Vec<Undo>> {
    let path = path.as_ref();
    let temp = temp.as_ref();
    if !path.is_file() {
        return Err(anyhow!("not a file: {}", path.display()));
    }

    let backup = temp_path(temp);
    if let Err(e) = tokio::fs::rename(path, &backup).await {
        return Err(anyhow!("failed to remove {}: {e}", path.display()));
    }
    Ok(vec![Undo::Restore {
        path: path.to_path_buf(),
        original: backup.to_path_buf(),
    }])
}

async fn rename_action(
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    options: RenameOptions,
    temp: impl AsRef<Path>,
) -> Result<Vec<Undo>> {
    let from = from.as_ref();
    let to = to.as_ref();
    let temp = temp.as_ref();
    let mut undo_stack = Vec::new();
    match (options.overwrite, to.exists()) {
        (true, _) | (false, false) => {
            // Backup the source
            let from_backup = temp_path(temp);
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
                let to_backup = temp_path(temp);
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
