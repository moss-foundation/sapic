use joinerror::Error;
use nanoid::nanoid;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

#[derive(Copy, Clone)]
pub struct CreateOptions {
    /// If true, both `create_file` and `create_file_with` will truncate the old content,
    /// If false, "create_file" will preserve the current content,
    /// While `create_file_with` will append the content at the end of the file.
    pub overwrite: bool,

    pub ignore_if_exists: bool,
}

#[derive(Copy, Clone)]
pub struct RenameOptions {
    /// If true, the destination will be overwritten by the source content
    /// If false, rename to an existing path will return an error if `ignore_if_exists` is false
    pub overwrite: bool,

    pub ignore_if_exists: bool,
}

#[derive(Copy, Clone)]
pub struct RemoveOptions {
    /// If false, `remove_` options will fail if the target does not exist
    pub ignore_if_not_exists: bool,
}

/// For every filesystem operation that succeeds, we push a reverse action
/// Once an operation fails, we go back through the sequence of actions
/// Which will reverse all the changes so far
#[derive(Debug)]
pub enum Undo {
    CreateDir(PathBuf),
    RemoveDir(PathBuf),
    RemoveDirContent(PathBuf),
    RemoveFile(PathBuf),
    Restore { path: PathBuf, original: PathBuf },
}

pub struct Rollback {
    tmp: PathBuf,
    pub(crate) undo_stack: Vec<Undo>,
}

impl Drop for Rollback {
    fn drop(&mut self) {
        let tmp = self.tmp.clone();
        tokio::spawn(async move { tokio::fs::remove_dir_all(tmp).await });
    }
}

impl Rollback {
    pub async fn new(tmp: impl AsRef<Path>) -> joinerror::Result<Self> {
        tokio::fs::create_dir_all(tmp.as_ref()).await?;
        Ok(Self {
            tmp: tmp.as_ref().to_path_buf(),
            undo_stack: Vec::new(),
        })
    }

    fn backup_path(&self) -> PathBuf {
        self.tmp.join(nanoid!(10))
    }

    pub async fn rollback(&mut self) -> joinerror::Result<()> {
        while let Some(undo) = self.undo_stack.pop() {
            let result = match undo {
                Undo::RemoveDir(path) => tokio::fs::remove_dir(&path).await,
                Undo::RemoveFile(path) => tokio::fs::remove_file(&path).await,
                Undo::Restore { path, original } => tokio::fs::rename(&original, &path).await,
                Undo::CreateDir(path) => tokio::fs::create_dir(&path).await,
                Undo::RemoveDirContent(path) => {
                    // Equivalent to remove the dir and create it again
                    tokio::fs::remove_dir_all(&path).await?;
                    tokio::fs::create_dir(&path).await
                }
            };
            if let Err(e) = result {
                return Err(Error::new::<()>(format!("failed to rollback: {}", e)));
            }
        }
        Ok(())
    }
}

pub async fn create_dir(rb: &mut Rollback, path: &Path) -> joinerror::Result<()> {
    tokio::fs::create_dir(path).await?;
    rb.undo_stack.push(Undo::RemoveDir(path.to_path_buf()));
    Ok(())
}

pub async fn create_dir_all(rb: &mut Rollback, path: &Path) -> joinerror::Result<()> {
    let mut missing_paths = Vec::new();
    for p in path.ancestors() {
        if p.exists() {
            continue;
        }
        missing_paths.push(p);
    }

    tokio::fs::create_dir_all(path).await?;
    // Deleting from inner to outer
    rb.undo_stack.extend(
        missing_paths
            .into_iter()
            .map(|p| Undo::RemoveDir(p.to_path_buf()))
            .rev(),
    );

    Ok(())
}

/// Right now we are always recursively deleting directories
pub async fn remove_dir(
    rb: &mut Rollback,
    path: &Path,
    options: RemoveOptions,
) -> joinerror::Result<()> {
    // Try moving the directory to be removed to a new tmporary directory
    if !path.exists() {
        return if options.ignore_if_not_exists {
            Ok(())
        } else {
            Err(Error::new::<()>(format!(
                "cannot remove non-existent directory: {}",
                path.display()
            )))
        };
    }

    if !path.is_dir() {
        return Err(Error::new::<()>(format!(
            "not a directory: {}",
            path.display()
        )));
    }

    let backup = rb.backup_path();
    if let Err(e) = tokio::fs::rename(path, &backup).await {
        return Err(Error::new::<()>(format!(
            "failed to remove directory at {}: {e}",
            path.display()
        )));
    }

    rb.undo_stack.push(Undo::Restore {
        path: path.to_path_buf(),
        original: backup,
    });
    Ok(())
}

pub async fn create_file(
    rb: &mut Rollback,
    path: &Path,
    options: CreateOptions,
) -> joinerror::Result<()> {
    let file_exists = path.exists();
    if file_exists && options.ignore_if_exists {
        return Ok(());
    }

    let backup = rb.backup_path();

    match (file_exists, options.overwrite) {
        (true, true) => {
            // Backup and truncate existing file
            if let Err(e) = tokio::fs::copy(path, &backup).await {
                return Err(Error::new::<()>(format!(
                    "failed to create a backup for {}: {e}",
                    path.display()
                )));
            }
            if let Err(e) = tokio::fs::File::options()
                .write(true)
                .truncate(true)
                .open(&path)
                .await
            {
                return Err(Error::new::<()>(format!(
                    "failed to truncate file at {}, {e}",
                    path.display()
                )));
            }
            rb.undo_stack.push(Undo::Restore {
                path: path.to_path_buf(),
                original: backup,
            });
        }
        (true, false) => {
            // Skip since the file already exists
        }
        (false, _) => {
            // Create a new file
            if let Err(e) = tokio::fs::File::create_new(path).await {
                return Err(Error::new::<()>(format!(
                    "failed to create a file at {}: {e}",
                    path.display()
                )));
            }
            rb.undo_stack.push(Undo::RemoveFile(path.to_path_buf()));
        }
    }
    Ok(())
}

pub async fn create_file_with(
    rb: &mut Rollback,
    path: &Path,
    options: CreateOptions,
    content: &[u8],
) -> joinerror::Result<()> {
    let file_exists = path.exists();
    if file_exists && options.ignore_if_exists {
        return Ok(());
    }

    let backup = rb.backup_path();

    match (file_exists, options.overwrite) {
        (true, true) => {
            // Backup the existing file content and overwrite it
            if let Err(e) = tokio::fs::copy(path, &backup).await {
                return Err(Error::new::<()>(format!(
                    "failed to create a backup for {}: {e}",
                    path.display()
                )));
            }

            if let Err(e) = tokio::fs::write(path, content).await {
                return Err(Error::new::<()>(format!(
                    "failed to overwrite a file at {}, {e}",
                    path.display()
                )));
            }

            rb.undo_stack.push(Undo::Restore {
                path: path.to_path_buf(),
                original: backup,
            });
        }
        (true, false) => {
            // Backup the existing file content and append to it
            if let Err(e) = tokio::fs::copy(path, &backup).await {
                return Err(Error::new::<()>(format!(
                    "failed to create a backup for {}: {e}",
                    path.display()
                )));
            }

            let mut open_options = tokio::fs::OpenOptions::new();
            open_options.append(true);
            let mut file = match open_options.open(path).await {
                Ok(file) => file,
                Err(e) => {
                    return Err(Error::new::<()>(format!(
                        "failed to open `{}` for appending: {e}",
                        path.display()
                    )));
                }
            };
            if let Err(e) = file.write_all(content).await {
                return Err(Error::new::<()>(format!(
                    "failed to append content to {}: {e}",
                    path.display()
                )));
            }
            if let Err(e) = file.flush().await {
                return Err(Error::new::<()>(format!(
                    "failed to flush content to {}: {e}",
                    path.display()
                )));
            }
            rb.undo_stack.push(Undo::Restore {
                path: path.to_path_buf(),
                original: backup,
            });
        }
        (false, _) => {
            if let Err(e) = tokio::fs::write(&path, content).await {
                return Err(Error::new::<()>(format!(
                    "failed to create a file with content at {}: {e}",
                    path.display()
                )));
            }
            rb.undo_stack.push(Undo::RemoveFile(path.to_path_buf()));
        }
    }
    Ok(())
}

pub async fn remove_file(
    rb: &mut Rollback,
    path: &Path,
    options: RemoveOptions,
) -> joinerror::Result<()> {
    if !path.exists() {
        return if options.ignore_if_not_exists {
            Ok(())
        } else {
            Err(Error::new::<()>(format!(
                "cannot remove nonexistent file at {}",
                path.display()
            )))
        };
    }
    if !path.is_file() {
        return Err(Error::new::<()>(format!("not a file: {}", path.display())));
    }

    let backup = rb.backup_path();
    if let Err(e) = tokio::fs::rename(path, &backup).await {
        return Err(Error::new::<()>(format!(
            "failed to remove file {}: {e}",
            path.display()
        )));
    }
    rb.undo_stack.push(Undo::Restore {
        path: path.to_path_buf(),
        original: backup,
    });
    Ok(())
}

/// The behavior when both from and to exists will match that of tokio::fs::rename
/// On Unix-like systems, when from is a directory, to must be an empty directory
pub async fn rename(
    rb: &mut Rollback,
    from: &Path,
    to: &Path,
    options: RenameOptions,
) -> joinerror::Result<()> {
    if !from.exists() {
        return Err(Error::new::<()>(format!(
            "cannot rename nonexistent file/directory {}",
            from.display()
        )));
    }

    if from.is_file() && to.is_dir() {
        return Err(Error::new::<()>(format!(
            "cannot rename a file to a directory: {} -> {}",
            from.display(),
            to.display()
        )));
    }

    if from.is_dir() && to.is_file() {
        return Err(Error::new::<()>(format!(
            "cannot rename a directory to a file: {} -> {}",
            from.display(),
            to.display()
        )));
    }

    if from.is_dir() {
        rename_dir_impl(rb, from, to, options).await
    } else {
        rename_file_impl(rb, from, to, options).await
    }
}

async fn rename_dir_impl(
    rb: &mut Rollback,
    from: &Path,
    to: &Path,
    options: RenameOptions,
) -> joinerror::Result<()> {
    match (options.overwrite, to.exists()) {
        (false, true) => {
            return if options.ignore_if_exists {
                Ok(())
            } else {
                Err(Error::new::<()>(format!(
                    "path `{}` already exists",
                    to.display()
                )))
            };
        }
        (true, true) => {
            // If the dest is already an empty directory
            // Undoing rename will restore an empty directory
            if let Err(e) = tokio::fs::rename(from, to).await {
                return Err(Error::new::<()>(format!(
                    "failed to rename dir {} to {}: {e}",
                    from.display(),
                    to.display()
                )));
            }

            rb.undo_stack.push(Undo::CreateDir(to.to_path_buf()));
            rb.undo_stack.push(Undo::Restore {
                path: from.to_path_buf(),
                original: to.to_path_buf(),
            });
        }

        (_, false) => {
            if let Err(e) = tokio::fs::rename(from, to).await {
                return Err(Error::new::<()>(format!(
                    "failed to rename dir {} to {}: {e}",
                    from.display(),
                    to.display()
                )));
            }
            rb.undo_stack.push(Undo::Restore {
                path: from.to_path_buf(),
                original: to.to_path_buf(),
            });
        }
    }
    Ok(())
}
async fn rename_file_impl(
    rb: &mut Rollback,
    from: &Path,
    to: &Path,
    options: RenameOptions,
) -> joinerror::Result<()> {
    match (options.overwrite, to.exists()) {
        (false, true) => {
            return if options.ignore_if_exists {
                // Skip when file exists
                Ok(())
            } else {
                Err(Error::new::<()>(format!(
                    "path `{}` already exists",
                    to.display()
                )))
            };
        }
        (true, true) => {
            let to_backup = rb.backup_path();
            if let Err(e) = tokio::fs::rename(to, &to_backup).await {
                return Err(Error::new::<()>(format!(
                    "failed to backup the destination `{}`: {e}",
                    to.display()
                )));
            }

            if let Err(e) = tokio::fs::rename(from, &to).await {
                return Err(Error::new::<()>(format!(
                    "failed to rename file `{}` to `{}`: {e}",
                    from.display(),
                    to.display()
                )));
            }

            rb.undo_stack.push(Undo::Restore {
                path: to.to_path_buf(),
                original: to_backup,
            });

            rb.undo_stack.push(Undo::Restore {
                path: from.to_path_buf(),
                original: to.to_path_buf(),
            });
        }

        (_, false) => {
            if let Err(e) = tokio::fs::rename(from, to).await {
                return Err(Error::new::<()>(format!(
                    "failed to rename file `{}` to `{}`: {e}",
                    from.display(),
                    to.display()
                )));
            };

            rb.undo_stack.push(Undo::Restore {
                path: from.to_path_buf(),
                original: to.to_path_buf(),
            });
        }
    }
    Ok(())
}
