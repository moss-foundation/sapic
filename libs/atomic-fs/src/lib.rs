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
    pub fn new(temp: impl AsRef<Path>) -> Result<Self> {
        std::fs::create_dir_all(temp.as_ref())?;
        Ok(Self {
            temp: temp.as_ref().to_path_buf(),
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
    let undo_stack = remove_dir_action(path, &rb.temp, options).await?;
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

pub async fn remove_file(
    rb: &mut Rollback,
    path: impl AsRef<Path>,
    options: RemoveOptions,
) -> Result<()> {
    let undo_stack = remove_file_action(path, &rb.temp, options).await?;
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
            Undo::CreateDir(path) => tokio::fs::create_dir(&path).await,
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
    temp: impl AsRef<Path>,
    options: RemoveOptions,
) -> Result<Vec<Undo>> {
    // Try moving the directory to be removed to a new temporary directory
    // This should have the same failure conditions as remove_dir
    let path = path.as_ref();
    let temp = temp.as_ref();

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

async fn remove_file_action(
    path: impl AsRef<Path>,
    temp: impl AsRef<Path>,
    options: RemoveOptions,
) -> Result<Vec<Undo>> {
    let path = path.as_ref();
    let temp = temp.as_ref();

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
        rename_dir_impl(from, to, options, temp).await
    } else {
        rename_file_impl(from, to, options, temp).await
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
    temp: &Path,
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
    temp: &Path,
) -> Result<Vec<Undo>> {
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

#[cfg(any(test, feature = "integration-tests"))]
mod tests {
    use super::*;
    fn setup_rollback() -> (Rollback, PathBuf) {
        let test_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("data")
            .join(nanoid!(10));
        std::fs::create_dir_all(&test_path).unwrap();
        (Rollback::new(test_path.join("temp")).unwrap(), test_path)
    }
    /// -------------------------------------------
    ///               Basic Operations
    /// -------------------------------------------

    /// ---------------create_dir()----------------
    #[tokio::test]
    pub async fn test_create_dir_success() {
        let (mut rb, test_path) = setup_rollback();
        let target = test_path.join("1");

        create_dir(&mut rb, &target).await.unwrap();

        assert!(target.exists());
        assert!(target.is_dir());
        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }
    #[tokio::test]
    pub async fn test_create_dir_missing_parent() {
        let (mut rb, test_path) = setup_rollback();

        // Missing parent folder
        assert!(
            create_dir(&mut rb, test_path.join("missing").join("1"))
                .await
                .is_err()
        );

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }
    #[tokio::test]
    pub async fn test_create_dir_all_success() {
        let (mut rb, test_path) = setup_rollback();

        let outer = test_path.join("1");
        let inner = outer.join("2");

        create_dir_all(&mut rb, &inner).await.unwrap();

        assert!(outer.is_dir());
        assert!(inner.is_dir());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    /// ---------------remove_dir()----------------
    #[tokio::test]
    pub async fn test_remove_dir_success() {
        let (mut rb, test_path) = setup_rollback();

        let target = test_path.join("1");
        tokio::fs::create_dir(&target).await.unwrap();

        remove_dir(
            &mut rb,
            &target,
            RemoveOptions {
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        assert!(!target.exists());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_remove_dir_with_content() {
        let (mut rb, test_path) = setup_rollback();

        let target = test_path.join("1");
        let file = target.join("file.txt");
        tokio::fs::create_dir(&target).await.unwrap();
        tokio::fs::File::create(&file).await.unwrap();

        remove_dir(
            &mut rb,
            &target,
            RemoveOptions {
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        assert!(!target.exists());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_remove_dir_ignore_when_not_exist() {
        let (mut rb, test_path) = setup_rollback();

        // Removing non-existent directory
        let target = test_path.join("1");
        assert!(
            remove_dir(
                &mut rb,
                &target,
                RemoveOptions {
                    ignore_if_not_exists: true,
                }
            )
            .await
            .is_ok()
        );

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_remove_dir_not_ignore_when_not_exist() {
        let (mut rb, test_path) = setup_rollback();

        // Removing non-existent directory
        let target = test_path.join("1");
        assert!(
            remove_dir(
                &mut rb,
                &target,
                RemoveOptions {
                    ignore_if_not_exists: false,
                }
            )
            .await
            .is_err()
        );

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    /// ---------------create_file()----------------
    #[tokio::test]
    pub async fn test_create_file_success() {
        let (mut rb, test_path) = setup_rollback();

        let target = test_path.join("file.txt");

        create_file(
            &mut rb,
            &target,
            CreateOptions {
                overwrite: false,
                create_new: true,
            },
        )
        .await
        .unwrap();

        assert!(target.is_file());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_create_file_create_new_when_exists() {
        let (mut rb, test_path) = setup_rollback();

        let target = test_path.join("file.txt");

        tokio::fs::File::create(&target).await.unwrap();

        assert!(
            create_file(
                &mut rb,
                &target,
                CreateOptions {
                    overwrite: false,
                    create_new: true,
                }
            )
            .await
            .is_err()
        );

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_create_file_overwrite_truncate_existing_file() {
        let (mut rb, test_path) = setup_rollback();

        let data = "Hello World".as_bytes();
        let target = test_path.join("file.txt");

        tokio::fs::write(&target, data).await.unwrap();

        // create_file with overwrite will truncate existing content
        create_file(
            &mut rb,
            &target,
            CreateOptions {
                overwrite: true,
                create_new: false,
            },
        )
        .await
        .unwrap();

        assert!(target.is_file());
        let new_data = tokio::fs::read(&target).await.unwrap();
        assert!(new_data.is_empty());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_create_file_non_overwrite_preserve_existing_file() {
        let (mut rb, test_path) = setup_rollback();

        let data = "Hello World".as_bytes();
        let target = test_path.join("file.txt");

        tokio::fs::write(&target, data).await.unwrap();

        // create_file without overwrite will preserve existing content
        create_file(
            &mut rb,
            &target,
            CreateOptions {
                overwrite: false,
                create_new: false,
            },
        )
        .await
        .unwrap();

        assert!(target.is_file());
        let new_data = tokio::fs::read(&target).await.unwrap();
        assert_eq!(new_data, data);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    /// -------------create_file_with()----------------

    #[tokio::test]
    pub async fn test_create_file_with_success() {
        let (mut rb, test_path) = setup_rollback();

        let data = "Hello World".as_bytes();
        let target = test_path.join("file.txt");

        create_file_with(
            &mut rb,
            &target,
            CreateOptions {
                overwrite: false,
                create_new: false,
            },
            data,
        )
        .await
        .unwrap();

        assert!(target.is_file());
        let content = tokio::fs::read(&target).await.unwrap();
        assert_eq!(content, data);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_create_file_with_create_new_when_exists() {
        let (mut rb, test_path) = setup_rollback();

        let data = "Hello World".as_bytes();
        let target = test_path.join("file.txt");
        tokio::fs::File::create(&target).await.unwrap();

        // Try creating a file that already exists
        assert!(
            create_file_with(
                &mut rb,
                &target,
                CreateOptions {
                    overwrite: false,
                    create_new: true,
                },
                data
            )
            .await
            .is_err()
        );

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_create_file_with_overwrite_existing_file() {
        let (mut rb, test_path) = setup_rollback();
        let old_data = "Hello World".as_bytes();
        let new_data = "42".as_bytes();
        let target = test_path.join("file.txt");

        tokio::fs::write(&target, old_data).await.unwrap();

        create_file_with(
            &mut rb,
            &target,
            CreateOptions {
                overwrite: true,
                create_new: false,
            },
            new_data,
        )
        .await
        .unwrap();

        let content = tokio::fs::read(&target).await.unwrap();
        assert_eq!(content, new_data);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_create_file_with_append_to_existing_file() {
        let (mut rb, test_path) = setup_rollback();
        let old_data = "Hello World".as_bytes();
        let new_data = "42".as_bytes();
        let target = test_path.join("file.txt");

        tokio::fs::write(&target, old_data).await.unwrap();

        create_file_with(
            &mut rb,
            &target,
            CreateOptions {
                overwrite: false,
                create_new: false,
            },
            new_data,
        )
        .await
        .unwrap();

        let content = tokio::fs::read(&target).await.unwrap();
        let complete_data = old_data
            .into_iter()
            .chain(new_data.into_iter())
            .cloned()
            .collect::<Vec<_>>();

        assert_eq!(content, complete_data);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    /// ---------------remove_file()----------------
    #[tokio::test]
    pub async fn test_remove_file_success() {
        let (mut rb, test_path) = setup_rollback();

        let target = test_path.join("file.txt");
        tokio::fs::File::create(&target).await.unwrap();

        remove_file(
            &mut rb,
            &target,
            RemoveOptions {
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();

        assert!(!target.exists());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_remove_file_ignore_when_not_exist() {
        let (mut rb, test_path) = setup_rollback();

        let target = test_path.join("file.txt");

        assert!(
            remove_file(
                &mut rb,
                &target,
                RemoveOptions {
                    ignore_if_not_exists: true,
                }
            )
            .await
            .is_ok()
        );

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_remove_file_not_ignore_when_not_exist() {
        let (mut rb, test_path) = setup_rollback();

        let target = test_path.join("file.txt");

        assert!(
            remove_file(
                &mut rb,
                &target,
                RemoveOptions {
                    ignore_if_not_exists: false,
                }
            )
            .await
            .is_err()
        );

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    /// ------------------rename()-----------------
    #[tokio::test]
    pub async fn test_rename_success() {
        let (mut rb, test_path) = setup_rollback();

        let data = "Hello World".as_bytes();
        let source = test_path.join("old.txt");
        let dest = test_path.join("new.txt");

        tokio::fs::write(&source, data).await.unwrap();

        rename(
            &mut rb,
            &source,
            &dest,
            RenameOptions {
                overwrite: false,
                ignore_if_exists: false,
            },
        )
        .await
        .unwrap();

        assert!(!source.exists());
        assert!(dest.exists());
        let content = tokio::fs::read(&dest).await.unwrap();
        assert_eq!(content, data);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rename_different_types() {
        let (mut rb, test_path) = setup_rollback();

        let source = test_path.join("old.txt");
        let dest = test_path.join("new");

        tokio::fs::File::create(&source).await.unwrap();
        tokio::fs::create_dir(&dest).await.unwrap();

        assert!(
            rename(
                &mut rb,
                &source,
                &dest,
                RenameOptions {
                    overwrite: true,
                    ignore_if_exists: false
                }
            )
            .await
            .is_err()
        );

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rename_ignore_when_exists() {
        let (mut rb, test_path) = setup_rollback();

        let source = test_path.join("old.txt");
        let dest = test_path.join("new.txt");

        tokio::fs::File::create(&source).await.unwrap();
        tokio::fs::File::create(&dest).await.unwrap();

        // Since the destination already exists, this will be a no op

        rename(
            &mut rb,
            &source,
            &dest,
            RenameOptions {
                overwrite: false,
                ignore_if_exists: true,
            },
        )
        .await
        .unwrap();

        assert!(source.exists());
        assert!(dest.exists());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rename_overwrite_existing_file() {
        let (mut rb, test_path) = setup_rollback();

        let source = test_path.join("old.txt");
        let source_content = "Source".as_bytes();
        let dest = test_path.join("new.txt");
        let dest_content = "Destination".as_bytes();

        tokio::fs::write(&source, source_content).await.unwrap();
        tokio::fs::write(&dest, dest_content).await.unwrap();

        rename(
            &mut rb,
            &source,
            &dest,
            RenameOptions {
                overwrite: true,
                ignore_if_exists: false,
            },
        )
        .await
        .unwrap();

        assert!(!source.exists());
        assert!(dest.exists());
        let content = tokio::fs::read(&dest).await.unwrap();
        assert_eq!(content, source_content);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rename_non_overwrite_already_exists() {
        let (mut rb, test_path) = setup_rollback();

        let source = test_path.join("old.txt");
        let dest = test_path.join("new.txt");

        tokio::fs::File::create(&source).await.unwrap();
        tokio::fs::create_dir(&dest).await.unwrap();

        assert!(
            rename(
                &mut rb,
                &source,
                &dest,
                RenameOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                }
            )
            .await
            .is_err()
        );

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rename_dir_nonempty_destination() {
        let (mut rb, test_path) = setup_rollback();

        let source = test_path.join("dir");
        let dest = test_path.join("new_dir");
        let file = dest.join("file.txt");
        tokio::fs::create_dir(&dest).await.unwrap();
        tokio::fs::write(&file, "Hello World".as_bytes())
            .await
            .unwrap();

        assert!(
            rename(
                &mut rb,
                &source,
                &dest,
                RenameOptions {
                    overwrite: true,
                    ignore_if_exists: false
                }
            )
            .await
            .is_err()
        );

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    /// -------------------------------------------
    ///          Simple Rollback Scenarios
    /// -------------------------------------------

    #[tokio::test]
    pub async fn test_rollback_create_dir() {
        let (mut rb, test_path) = setup_rollback();

        let target = test_path.join("1");

        create_dir(&mut rb, &target).await.unwrap();
        assert!(target.is_dir());

        rollback(&mut rb).await.unwrap();
        assert!(!target.exists());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_create_dir_all() {
        let (mut rb, test_path) = setup_rollback();

        let outer = test_path.join("1");
        let inner = outer.join("2");

        create_dir_all(&mut rb, &inner).await.unwrap();
        assert!(outer.is_dir());
        assert!(inner.is_dir());

        rollback(&mut rb).await.unwrap();

        assert!(!outer.exists());
        assert!(!inner.exists());
        // Should remove only directories created during the operation
        assert!(test_path.exists());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_remove_dir() {
        let (mut rb, test_path) = setup_rollback();

        let target = test_path.join("1");
        tokio::fs::create_dir(&target).await.unwrap();

        remove_dir(
            &mut rb,
            &target,
            RemoveOptions {
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        assert!(!target.exists());

        rollback(&mut rb).await.unwrap();

        // Should restore deleted directory
        assert!(target.exists());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_remove_dir_with_content() {
        let (mut rb, test_path) = setup_rollback();

        let target = test_path.join("1");
        let file = target.join("file.txt");
        tokio::fs::create_dir(&target).await.unwrap();
        tokio::fs::File::create(&file).await.unwrap();

        remove_dir(
            &mut rb,
            &target,
            RemoveOptions {
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();
        assert!(!target.exists());

        rollback(&mut rb).await.unwrap();

        // Should restore deleted directory and its content
        assert!(target.exists());
        assert!(file.exists());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_create_file() {
        let (mut rb, test_path) = setup_rollback();

        let file = test_path.join("file.txt");

        create_file(
            &mut rb,
            &file,
            CreateOptions {
                overwrite: false,
                create_new: true,
            },
        )
        .await
        .unwrap();
        assert!(file.is_file());

        rollback(&mut rb).await.unwrap();

        assert!(!file.exists());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_create_file_truncate() {
        let (mut rb, test_path) = setup_rollback();

        let file = test_path.join("file.txt");

        let data = "Hello World!".as_bytes();
        // Create a test file with content
        tokio::fs::write(&file, data).await.unwrap();

        // Create a file with overwrite option will truncate it
        create_file(
            &mut rb,
            &file,
            CreateOptions {
                overwrite: true,
                create_new: false,
            },
        )
        .await
        .unwrap();
        let data_after_truncation = tokio::fs::read(&file).await.unwrap();
        assert!(data_after_truncation.is_empty());

        rollback(&mut rb).await.unwrap();
        // Should restore the original content before truncation
        let data_after_rollback = tokio::fs::read(&file).await.unwrap();
        assert_eq!(data_after_rollback, data);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_create_file_with_content() {
        let (mut rb, test_path) = setup_rollback();

        let file = test_path.join("file.txt");

        let data = "Hello World!".as_bytes();

        create_file_with(
            &mut rb,
            &file,
            CreateOptions {
                overwrite: true,
                create_new: true,
            },
            data,
        )
        .await
        .unwrap();

        rollback(&mut rb).await.unwrap();
        // Should remove the newly created file
        assert!(!file.exists());
        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_create_file_with_overwrite_existing() {
        let (mut rb, test_path) = setup_rollback();

        let file = test_path.join("file.txt");

        let old_data = "Old".as_bytes();
        let new_data = "New".as_bytes();

        // Create the file with old data first
        tokio::fs::write(&file, old_data).await.unwrap();
        // Overwrite the file with new data
        create_file_with(
            &mut rb,
            &file,
            CreateOptions {
                overwrite: true,
                create_new: false,
            },
            new_data,
        )
        .await
        .unwrap();

        rollback(&mut rb).await.unwrap();

        // Should restore the old content of this file
        assert!(file.exists());
        let restored_data = tokio::fs::read(&file).await.unwrap();

        assert_eq!(old_data, restored_data);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_create_file_with_append_to_existing() {
        let (mut rb, test_path) = setup_rollback();

        let file = test_path.join("file.txt");
        let old_data = "Old".as_bytes();
        let new_data = "New".as_bytes();
        tokio::fs::write(&file, old_data).await.unwrap();

        // Create the file with old data first
        tokio::fs::write(&file, old_data).await.unwrap();
        // Append the new data at the end of old data
        create_file_with(
            &mut rb,
            &file,
            CreateOptions {
                overwrite: false,
                create_new: false,
            },
            new_data,
        )
        .await
        .unwrap();

        rollback(&mut rb).await.unwrap();
        // Should restore the old content of this file
        assert!(file.exists());
        let restored_data = tokio::fs::read(&file).await.unwrap();

        assert_eq!(old_data, restored_data);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_remove_file() {
        let (mut rb, test_path) = setup_rollback();
        let file = test_path.join("file.txt");
        let content = "Hello World!".as_bytes();

        tokio::fs::write(&file, content).await.unwrap();

        remove_file(
            &mut rb,
            &file,
            RemoveOptions {
                ignore_if_not_exists: false,
            },
        )
        .await
        .unwrap();

        rollback(&mut rb).await.unwrap();
        // Should restore the file with the original content
        assert!(file.exists());
        let restored_data = tokio::fs::read(&file).await.unwrap();
        assert_eq!(content, restored_data);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_rename_file() {
        let (mut rb, test_path) = setup_rollback();
        let source = test_path.join("source.txt");
        let dest = test_path.join("dest.txt");
        let content = "Hello World!".as_bytes();

        tokio::fs::write(&source, content).await.unwrap();

        rename(
            &mut rb,
            &source,
            &dest,
            RenameOptions {
                overwrite: false,
                ignore_if_exists: false,
            },
        )
        .await
        .unwrap();

        rollback(&mut rb).await.unwrap();
        assert!(source.exists());
        assert!(!dest.exists());
        let restored_data = tokio::fs::read(&source).await.unwrap();
        assert_eq!(content, restored_data);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_rename_file_overwrite() {
        let (mut rb, test_path) = setup_rollback();
        let source = test_path.join("source.txt");
        let source_data = "Source".as_bytes();
        let dest = test_path.join("dest.txt");
        let dest_data = "Dest".as_bytes();

        tokio::fs::write(&source, source_data).await.unwrap();
        tokio::fs::write(&dest, dest_data).await.unwrap();

        rename(
            &mut rb,
            &source,
            &dest,
            RenameOptions {
                overwrite: true,
                ignore_if_exists: false,
            },
        )
        .await
        .unwrap();

        rollback(&mut rb).await.unwrap();
        // Both source and dest should be restored to the previous state
        assert!(source.exists());
        let restored_source_data = tokio::fs::read(&source).await.unwrap();
        assert_eq!(source_data, restored_source_data);
        assert!(dest.exists());
        let restored_dest_data = tokio::fs::read(&dest).await.unwrap();
        assert_eq!(dest_data, restored_dest_data);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_rename_dir_with_content() {
        let (mut rb, test_path) = setup_rollback();
        let source = test_path.join("dir");
        let file = source.join("file.txt");
        let file_content = "Hello World!".as_bytes();
        let dest = test_path.join("new_dir");

        tokio::fs::create_dir(&source).await.unwrap();
        tokio::fs::write(&file, file_content).await.unwrap();

        rename(
            &mut rb,
            &source,
            &dest,
            RenameOptions {
                overwrite: false,
                ignore_if_exists: false,
            },
        )
        .await
        .unwrap();

        rollback(&mut rb).await.unwrap();
        assert!(source.exists());
        assert!(file.exists());
        let restored_file_content = tokio::fs::read(&file).await.unwrap();
        assert_eq!(file_content, restored_file_content);
        assert!(!dest.exists());

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_rollback_rename_dir_empty_dest_exists() {
        let (mut rb, test_path) = setup_rollback();
        let source = test_path.join("dir");
        let file = source.join("file.txt");
        let file_content = "Hello World!".as_bytes();
        let dest = test_path.join("new_dir");

        tokio::fs::create_dir(&source).await.unwrap();
        tokio::fs::write(&file, file_content).await.unwrap();
        tokio::fs::create_dir(&dest).await.unwrap();

        rename(
            &mut rb,
            &source,
            &dest,
            RenameOptions {
                overwrite: true,
                ignore_if_exists: false,
            },
        )
        .await
        .unwrap();

        rollback(&mut rb).await.unwrap();
        assert!(source.exists());
        assert!(file.exists());
        let restored_file_content = tokio::fs::read(&file).await.unwrap();
        assert_eq!(file_content, restored_file_content);

        // The initial empty destination directory should be restored
        assert!(dest.exists());
        let mut entries = tokio::fs::read_dir(&dest).await.unwrap();
        assert!(entries.next_entry().await.unwrap().is_none());

        tokio::fs::remove_dir_all(&dest).await.unwrap();
    }

    /// -------------------------------------------
    ///          Complex Rollback Scenarios
    /// -------------------------------------------

    #[tokio::test]
    pub async fn test_rollback_complex() {
        // Base state:
        // folder
        // folder/inner.txt ("inner")
        // outer.txt ("outer")

        let (mut rb, test_path) = setup_rollback();
        let folder = test_path.join("folder");
        let inner_file = folder.join("inner.txt");
        let inner_content = "inner".as_bytes();
        let outer_file = test_path.join("outer.txt");
        let outer_content = "outer".as_bytes();

        tokio::fs::create_dir(&folder).await.unwrap();
        tokio::fs::write(&inner_file, inner_content).await.unwrap();
        tokio::fs::write(&outer_file, outer_content).await.unwrap();

        rename(
            &mut rb,
            &inner_file,
            &outer_file,
            RenameOptions {
                overwrite: true,
                ignore_if_exists: false,
            },
        )
        .await
        .unwrap();

        let inner_new_content = "new_content".as_bytes();
        create_file_with(
            &mut rb,
            &inner_file,
            CreateOptions {
                overwrite: false,
                create_new: true,
            },
            inner_new_content,
        )
        .await
        .unwrap();

        rollback(&mut rb).await.unwrap();
        assert!(folder.exists());
        assert!(inner_file.exists());
        assert!(outer_file.exists());
        let restored_inner_content = tokio::fs::read(&inner_file).await.unwrap();
        assert_eq!(inner_content, restored_inner_content);
        let restored_outer_content = tokio::fs::read(&outer_file).await.unwrap();
        assert_eq!(outer_content, restored_outer_content);

        tokio::fs::remove_dir_all(&test_path).await.unwrap();
    }
}
