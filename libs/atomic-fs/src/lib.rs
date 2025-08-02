use anyhow::{Result, anyhow};
use nanoid::nanoid;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

#[derive(Copy, Clone)]
pub struct CreateOptions {
    /// If true, both `create_file` and `create_file_with` will truncate the old content,
    /// If false, "create_file" will preserve the current content,
    /// While `create_file_with` will append the content at the end of the file.
    pub overwrite: bool,

    /// If true, both `create_file` and `create_file_with` will fail if the file already exists.
    pub create_new: bool,
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
    RemoveFile(PathBuf),
    Restore { path: PathBuf, original: PathBuf },
}

pub struct Rollback {
    tmp: PathBuf,
    undo_stack: Vec<Undo>,
}

impl Drop for Rollback {
    fn drop(&mut self) {
        let tmp = self.tmp.clone();
        tokio::spawn(async move { tokio::fs::remove_dir_all(tmp).await });
    }
}

impl Rollback {
    pub fn new(tmp: impl AsRef<Path>) -> Result<Self> {
        std::fs::create_dir_all(tmp.as_ref())?;
        Ok(Self {
            tmp: tmp.as_ref().to_path_buf(),
            undo_stack: Vec::new(),
        })
    }
}

pub async fn create_dir(rb: &mut Rollback, path: impl AsRef<Path>) -> Result<()> {
    let undo_stack = create_dir_action(path).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

pub async fn create_dir_all(rb: &mut Rollback, path: impl AsRef<Path>) -> Result<()> {
    let undo_stack = create_dir_all_action(path).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

pub async fn remove_dir(
    rb: &mut Rollback,
    path: impl AsRef<Path>,
    options: RemoveOptions,
) -> Result<()> {
    let undo_stack = remove_dir_action(path, &rb.tmp, options).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

pub async fn create_file(
    rb: &mut Rollback,
    path: impl AsRef<Path>,
    options: CreateOptions,
) -> Result<()> {
    let undo_stack = create_file_action(path, options, &rb.tmp).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

pub async fn create_file_with(
    rb: &mut Rollback,
    path: impl AsRef<Path>,
    options: CreateOptions,
    content: &[u8],
) -> Result<()> {
    let undo_stack = create_file_with_action(path, options, content, &rb.tmp).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

pub async fn remove_file(
    rb: &mut Rollback,
    path: impl AsRef<Path>,
    options: RemoveOptions,
) -> Result<()> {
    let undo_stack = remove_file_action(path, &rb.tmp, options).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

/// To keep the semantics consistent with std::fs::rename,
/// We are also using `rename` for storing backups.
/// This means that it will fail if the tmporary folder
/// Is on a different mount point than the source/destination
pub async fn rename(
    rb: &mut Rollback,
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    options: RenameOptions,
) -> Result<()> {
    let undo_stack = rename_action(from, to, options, &rb.tmp).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

pub async fn rollback(rb: &mut Rollback) -> Result<()> {
    while let Some(undo) = rb.undo_stack.pop() {
        let result = match undo {
            Undo::RemoveDir(path) => tokio::fs::remove_dir(&path).await,
            Undo::RemoveFile(path) => tokio::fs::remove_file(&path).await,
            Undo::Restore { path, original } => tokio::fs::rename(&original, &path).await,
            Undo::CreateDir(path) => tokio::fs::create_dir(&path).await,
        };
        if let Err(err) = result {
            return Err(anyhow!("failed to rollback: {}", err));
        }
    }
    Ok(())
}

fn tmp_path(tmp_dir: impl AsRef<Path>) -> PathBuf {
    tmp_dir.as_ref().join(nanoid!(10))
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

    let mut missing_paths = Vec::new();
    for p in path.ancestors() {
        if p.exists() {
            continue;
        }
        missing_paths.push(p.to_path_buf());
    }

    if let Err(e) = tokio::fs::create_dir_all(path).await {
        return Err(anyhow!(
            "failed to create directory up to {}: {e}",
            path.display()
        ));
    } else {
        Ok(missing_paths
            .into_iter()
            .rev()
            .map(|p| Undo::RemoveDir(p.to_path_buf()))
            .collect())
    }
}

async fn remove_dir_action(
    path: impl AsRef<Path>,
    tmp: impl AsRef<Path>,
    options: RemoveOptions,
) -> Result<Vec<Undo>> {
    // Try moving the directory to be removed to a new tmporary directory
    // This should have the same failure conditions as remove_dir
    let path = path.as_ref();
    let tmp = tmp.as_ref();

    if !path.exists() {
        return if options.ignore_if_not_exists {
            Ok(vec![])
        } else {
            Err(anyhow!(
                "cannot remove non-existing directory: {}",
                path.display()
            ))
        };
    }

    if !path.is_dir() {
        return Err(anyhow!("not a directory: {}", path.display()));
    }

    let backup = tmp_path(tmp);
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
async fn create_file_action(
    path: impl AsRef<Path>,
    options: CreateOptions,
    tmp: impl AsRef<Path>,
) -> Result<Vec<Undo>> {
    let path = path.as_ref();
    let tmp = tmp.as_ref();

    let file_exists = path.exists();
    if file_exists && options.create_new {
        return Err(anyhow!("file already exists at {}", path.display()));
    }

    let backup = tmp_path(tmp);
    match (file_exists, options.overwrite) {
        (true, true) => {
            // Backup and truncate existing file
            if let Err(e) = tokio::fs::copy(path, &backup).await {
                return Err(anyhow!(
                    "failed to create a backup for {}: {e}",
                    path.display()
                ));
            }
            if let Err(e) = tokio::fs::File::options()
                .write(true)
                .truncate(true)
                .open(&path)
                .await
            {
                return Err(anyhow!(
                    "failed to truncate file at {}: {e}",
                    path.display()
                ));
            }
            Ok(vec![Undo::Restore {
                path: path.to_path_buf(),
                original: backup,
            }])
        }
        (true, false) => {
            // Skip since the file already exists
            Ok(vec![])
        }
        (false, _) => {
            if let Err(e) = tokio::fs::File::create_new(path).await {
                return Err(anyhow!(
                    "failed to create a file at {}: {e}",
                    path.display()
                ));
            }
            Ok(vec![Undo::RemoveFile(path.to_path_buf())])
        }
    }
}

async fn create_file_with_action(
    path: impl AsRef<Path>,
    options: CreateOptions,
    content: &[u8],
    tmp: impl AsRef<Path>,
) -> Result<Vec<Undo>> {
    let path = path.as_ref();
    let tmp = tmp.as_ref();

    let file_exists = path.exists();
    if file_exists && options.create_new {
        return Err(anyhow!("File already exists at {}", path.display()));
    }

    let backup = tmp_path(tmp);
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
                return Err(anyhow!(
                    "failed to create a file with content at {}: {e}",
                    path.display()
                ));
            }
            Ok(vec![Undo::RemoveFile(path.to_path_buf())])
        }
    }
}

async fn remove_file_action(
    path: impl AsRef<Path>,
    tmp: impl AsRef<Path>,
    options: RemoveOptions,
) -> Result<Vec<Undo>> {
    let path = path.as_ref();
    let tmp = tmp.as_ref();

    if !path.exists() {
        return if options.ignore_if_not_exists {
            Ok(vec![])
        } else {
            Err(anyhow!(
                "cannot remove nonexistent file: {}",
                path.display()
            ))
        };
    }
    if !path.is_file() {
        return Err(anyhow!("not a file: {}", path.display()));
    }

    let backup = tmp_path(tmp);
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
    tmp: impl AsRef<Path>,
) -> Result<Vec<Undo>> {
    let from = from.as_ref();
    let to = to.as_ref();
    let tmp = tmp.as_ref();
    if !from.exists() {
        return Err(anyhow!(
            "cannot rename nonexistent file/directory: {}",
            from.display()
        ));
    }

    if from.is_file() && to.is_dir() {
        return Err(anyhow!(
            "cannot rename a file into a directory: {} -> {}",
            from.display(),
            to.display()
        ));
    }

    if from.is_dir() && to.is_file() {
        return Err(anyhow!(
            "cannot rename a directory into a file: {} -> {}",
            from.display(),
            to.display()
        ));
    }

    if from.is_dir() {
        rename_dir_impl(from, to, options, tmp).await
    } else {
        rename_file_impl(from, to, options, tmp).await
    }
}

// FIXME: Not sure if we should actually support renaming into a non-empty directory
// It would be ambiguous whether `overwrite` means truncating existing content at the destination
// Or if it means overwriting on a per file basis, similar to batch move
// The standard library does not support renaming into a non-empty directory
// on both Unix-like systems and Windows

async fn rename_dir_impl(
    from: &Path,
    to: &Path,
    options: RenameOptions,
    _tmp: &Path,
) -> Result<Vec<Undo>> {
    let mut undo_stack = Vec::new();

    match (options.overwrite, to.exists()) {
        (true, _) | (false, false) => {
            // If the dest already exists as an empty directory before rename
            // undoing rename will restore an empty directory
            if to.exists() {
                let mut entries = tokio::fs::read_dir(to).await?;
                if entries.next_entry().await?.is_some() {
                    return Err(anyhow!(
                        "cannot rename to a non-empty directory {}",
                        to.display()
                    ));
                }
                undo_stack.push(Undo::CreateDir(to.to_path_buf()));
            }

            if let Err(e) = tokio::fs::rename(from, to).await {
                return Err(anyhow!(
                    "failed to rename {} to {}: {e}",
                    from.display(),
                    to.display()
                ));
            }
            undo_stack.push(Undo::Restore {
                path: from.to_path_buf(),
                original: to.to_path_buf(),
            });

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

async fn rename_file_impl(
    from: &Path,
    to: &Path,
    options: RenameOptions,
    tmp: &Path,
) -> Result<Vec<Undo>> {
    let mut undo_stack = Vec::new();
    match (options.overwrite, to.exists()) {
        (true, _) | (false, false) => {
            // Backup the source
            let from_backup = tmp_path(tmp);
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

            if to.exists() {
                let to_backup = tmp_path(tmp);
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
            } else {
                undo_stack.push(Undo::RemoveFile(to.to_path_buf()));
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
