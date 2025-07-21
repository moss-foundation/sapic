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

/// For every filesystem operation that succeeds, we push a reverse action
/// Once an operation fails, we go back through the sequence of actions
/// Which will reverse all the changes so far
pub enum Undo {
    RemoveDir(PathBuf),
    RemoveFile(PathBuf),
    Restore { path: PathBuf, original: PathBuf },
}

pub struct Rollback {
    temp: PathBuf,
    undo_stack: Vec<Undo>,
}

impl Drop for Rollback {
    fn drop(&mut self) {
        let temp = self.temp.clone();
        tokio::spawn(async move { tokio::fs::remove_dir_all(temp).await });
    }
}

impl Rollback {
    pub fn new(temp: impl AsRef<Path>) -> Self {
        Self {
            temp: temp.as_ref().to_path_buf(),
            undo_stack: Vec::new(),
        }
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

pub async fn remove_dir(rb: &mut Rollback, path: impl AsRef<Path>) -> Result<()> {
    let undo_stack = remove_dir_action(path, &rb.temp).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

pub async fn create_file(
    rb: &mut Rollback,
    path: impl AsRef<Path>,
    options: CreateOptions,
) -> Result<()> {
    let undo_stack = create_file_action(path, options, &rb.temp).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

pub async fn create_file_with(
    rb: &mut Rollback,
    path: impl AsRef<Path>,
    options: CreateOptions,
    content: &[u8],
) -> Result<()> {
    let undo_stack = create_file_with_action(path, options, content, &rb.temp).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

pub async fn remove_file(rb: &mut Rollback, path: impl AsRef<Path>) -> Result<()> {
    let undo_stack = remove_file_action(path, &rb.temp).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

/// To keep the semantics consistent with std::fs::rename,
/// We are also using `rename` for storing backups.
/// This means that it will fail if the temporary folder
/// Is on a different mount point than the source/destination
pub async fn rename(
    rb: &mut Rollback,
    from: impl AsRef<Path>,
    to: impl AsRef<Path>,
    options: RenameOptions,
) -> Result<()> {
    let undo_stack = rename_action(from, to, options, &rb.temp).await?;
    rb.undo_stack.extend(undo_stack);
    Ok(())
}

pub async fn rollback(rb: &mut Rollback) -> Result<()> {
    while let Some(undo) = rb.undo_stack.pop() {
        let result = match undo {
            Undo::RemoveDir(path) => tokio::fs::remove_dir(&path).await,
            Undo::RemoveFile(path) => tokio::fs::remove_file(&path).await,
            Undo::Restore { path, original } => tokio::fs::rename(&original, &path).await,
        };
        if let Err(err) = result {
            return Err(anyhow!("failed to rollback: {}", err));
        }
    }
    Ok(())
}

fn temp_path(temp_dir: impl AsRef<Path>) -> PathBuf {
    temp_dir.as_ref().join(nanoid!(10))
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
            .map(|p| Undo::RemoveDir(p.to_path_buf()))
            .collect())
    }
}

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
async fn create_file_action(
    path: impl AsRef<Path>,
    options: CreateOptions,
    temp: impl AsRef<Path>,
) -> Result<Vec<Undo>> {
    let path = path.as_ref();
    let temp = temp.as_ref();

    let file_exists = path.exists();
    if file_exists && options.create_new {
        return Err(anyhow!("file already exists at {}", path.display()));
    }

    let backup = temp_path(temp);
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
    temp: impl AsRef<Path>,
) -> Result<Vec<Undo>> {
    let path = path.as_ref();
    let temp = temp.as_ref();

    let file_exists = path.exists();
    if file_exists && options.create_new {
        return Err(anyhow!("File already exists at {}", path.display()));
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
                return Err(anyhow!(
                    "failed to create a file with content at {}: {e}",
                    path.display()
                ));
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

#[cfg(any(test, feature = "integration-tests"))]
mod tests {
    use super::*;
    use std::fs::remove_dir_all;
    fn setup_rollback() -> (Rollback, PathBuf) {
        let test_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join(nanoid!(10));
        std::fs::create_dir_all(&test_path).unwrap();
        (Rollback::new(test_path.join("temp")), test_path)
    }
    /// -------------------------------------------
    ///               Basic Operations
    /// -------------------------------------------
    #[tokio::test]
    pub async fn test_create_dir_success() {
        let (mut rollback, test_path) = setup_rollback();

        create_dir(&mut rollback, test_path.join("1"))
            .await
            .unwrap();

        assert!(test_path.join("1").exists());
        assert!(test_path.join("1").is_dir());
        remove_dir_all(&test_path).unwrap();
    }
    #[tokio::test]
    pub async fn test_create_dir_failure() {
        let (mut rollback, test_path) = setup_rollback();

        // Missing parent folder

        assert!(
            create_dir(&mut rollback, test_path.join("missing").join("1"))
                .await
                .is_err()
        );

        remove_dir_all(&test_path).unwrap();
    }
}
