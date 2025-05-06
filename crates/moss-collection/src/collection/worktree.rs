pub mod snapshot;

use anyhow::Result;
use file_id::FileId;
use moss_fs::{CreateOptions, FileSystem, RemoveOptions, RenameOptions};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use tokio::sync::{RwLock, mpsc};

use crate::models::primitives::EntryId;

use self::snapshot::{Entry, EntryKind, Snapshot};

struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    scan_queue: mpsc::UnboundedSender<ScanJob>,
}

pub struct Worktree {
    fs: Arc<dyn FileSystem>,
    next_entry_id: Arc<AtomicUsize>,
    snapshot: Arc<RwLock<Snapshot>>,
}

impl Worktree {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        next_entry_id: Arc<AtomicUsize>,
        abs_path: Arc<Path>,
    ) -> Self {
        debug_assert!(abs_path.is_absolute());

        let initial_snapshot = Snapshot::new(abs_path);

        Self {
            fs,
            next_entry_id,
            snapshot: Arc::new(RwLock::new(initial_snapshot)),
        }
    }

    pub async fn absolutize(&self, path: &Path) -> Result<PathBuf> {
        debug_assert!(path.is_relative());

        if path
            .components()
            .any(|c| c == std::path::Component::ParentDir)
        {
            return Err(anyhow::anyhow!(
                "Path cannot contain '..' components: {}",
                path.display()
            ));
        }

        let snapshot = self.snapshot.read().await;
        if path.file_name().is_some() {
            Ok(snapshot.abs_path().join(path))
        } else {
            Ok(snapshot.abs_path().to_path_buf())
        }
    }

    pub async fn relativize(&self, abs_path: &Path) -> Result<PathBuf> {
        debug_assert!(abs_path.is_absolute());

        let snapshot = self.snapshot.read().await;
        let root_path = snapshot.abs_path();

        if !abs_path.starts_with(root_path) {
            return Err(anyhow::anyhow!(
                "Path {} is outside of the worktree root {}",
                abs_path.display(),
                root_path.display()
            ));
        }

        Ok(abs_path
            .strip_prefix(root_path)
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|_| PathBuf::from("")))
    }

    pub async fn create_entry(
        &self,
        path: impl Into<Arc<Path>>,
        is_dir: bool,
        content: Option<Vec<u8>>,
    ) -> Result<()> {
        let path = path.into();
        debug_assert!(path.is_relative());

        let abs_path = self.absolutize(&path).await?;

        if is_dir {
            self.fs.create_dir(&abs_path).await?;
        } else {
            self.fs
                .create_file_with(
                    &abs_path,
                    String::from_utf8(content.as_deref().unwrap_or(&[]).to_vec())?,
                    CreateOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await?;
        }

        let file_id = file_id::get_file_id(&abs_path)?;

        let mut snapshot_lock = self.snapshot.write().await;
        snapshot_lock.create_entry(
            (Entry {
                id: EntryId::new(&self.next_entry_id),
                path,
                kind: if is_dir {
                    EntryKind::Dir
                } else {
                    EntryKind::File
                },
                unit_type: None,
                mtime: None,
                file_id,
            })
            .into(),
        );

        Ok(())
    }

    pub async fn remove_entry(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        let abs_path = self.absolutize(&path).await?;

        let mut snapshot_lock = self.snapshot.write().await;

        if abs_path.is_dir() {
            let mut temp_dir = abs_path.clone();
            let original_name = temp_dir.file_name().unwrap().to_string_lossy().to_string();
            let temp_name = format!(".{}.deleted.{}", original_name, std::process::id());
            temp_dir.set_file_name(temp_name);

            self.fs
                .rename(
                    &abs_path,
                    &temp_dir,
                    RenameOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await?;

            tokio::spawn(async move {
                match tokio::fs::remove_dir_all(&temp_dir).await {
                    Ok(_) => (),
                    Err(e) => eprintln!(
                        "Error removing temporary directory {}: {}",
                        temp_dir.display(),
                        e
                    ),
                }
            });
        } else {
            self.fs
                .remove_file(
                    &abs_path,
                    RemoveOptions {
                        recursive: false,
                        ignore_if_not_exists: false,
                    },
                )
                .await?;
        }

        snapshot_lock.remove_entry(path);

        Ok(())
    }

    pub async fn rename_entry(
        &self,
        old_path: impl AsRef<Path>,
        new_path: impl AsRef<Path>,
    ) -> Result<()> {
        let old_path = old_path.as_ref();
        let new_path = new_path.as_ref();
        debug_assert!(old_path.is_relative());
        debug_assert!(new_path.is_relative());

        let abs_old_path = self.absolutize(old_path).await?;
        let abs_new_path = self.absolutize(new_path).await?;

        let mut snapshot_lock = self.snapshot.write().await;

        self.fs
            .rename(
                &abs_old_path,
                &abs_new_path,
                RenameOptions {
                    overwrite: true,
                    ignore_if_exists: false,
                },
            )
            .await?;

        snapshot_lock.remove_entry(old_path);

        for entry in self.scan(abs_new_path.into()).await? {
            snapshot_lock.create_entry(entry.into());
        }

        Ok(())
    }

    pub async fn initial_scan(&self) -> Result<()> {
        let root_abs_path = {
            let snapshot = self.snapshot.read().await;
            snapshot.abs_path().clone()
        };

        let entries = self.scan(root_abs_path).await?;
        let mut snapshot_lock = self.snapshot.write().await;
        for entry in entries {
            snapshot_lock.create_entry(entry.into());
        }

        Ok(())
    }

    pub async fn scan(&self, abs_path: Arc<Path>) -> Result<Vec<Entry>> {
        debug_assert!(abs_path.is_absolute());

        let (scan_job_tx, mut scan_job_rx) = mpsc::unbounded_channel();

        let path: Arc<Path> = self.relativize(&abs_path).await?.into();
        let initial_job = ScanJob {
            abs_path,
            path,
            scan_queue: scan_job_tx.clone(),
        };
        scan_job_tx.send(initial_job).unwrap();

        drop(scan_job_tx);

        let mut handles = Vec::new();
        while let Some(job) = scan_job_rx.recv().await {
            let fs_clone = self.fs.clone();
            let next_entry_id = self.next_entry_id.clone();

            let handle = tokio::spawn(async move {
                let mut new_entries = Vec::new();
                let mut new_jobs: Vec<ScanJob> = Vec::new();

                let mut read_dir = fs_clone.read_dir(&job.abs_path).await.unwrap();

                let mut child_paths = Vec::new();
                while let Some(dir_entry) = read_dir.next_entry().await.unwrap_or(None) {
                    child_paths.push(dir_entry);
                }

                for child_entry in child_paths {
                    let child_abs_path: Arc<Path> = child_entry.path().into();
                    let child_name = child_abs_path.file_name().unwrap();
                    let child_path: Arc<Path> = job.path.join(child_name).into();

                    let child_metadata = match tokio::fs::metadata(&child_abs_path).await {
                        Ok(metadata) => metadata,
                        Err(_) => continue, // Skip if we can't get the metadata // TODO: handle errors?
                    };

                    let is_dir = child_metadata.is_dir();
                    let entry_kind = if is_dir {
                        EntryKind::Dir
                    } else {
                        EntryKind::File
                    };

                    let file_id = match file_id::get_file_id(&child_abs_path) {
                        Ok(id) => id,
                        Err(_) => continue, // Skip if we can't get the file ID // TODO: handle errors?
                    };

                    let child_entry = Entry {
                        id: EntryId::new(&next_entry_id),
                        path: child_path.clone(),
                        kind: entry_kind,
                        unit_type: None,
                        mtime: child_metadata.modified().ok(),
                        file_id,
                    };

                    if is_dir {
                        new_jobs.push(ScanJob {
                            abs_path: child_abs_path,
                            path: child_path,
                            scan_queue: job.scan_queue.clone(),
                        });
                    }

                    new_entries.push(child_entry);
                }

                for new_job in new_jobs.into_iter() {
                    job.scan_queue.send(new_job).unwrap(); // TODO: handle errors?
                }

                new_entries
            });

            handles.push(handle);
        }

        Ok(futures::future::join_all(handles)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use moss_fs::RealFileSystem;

    use super::*;

    #[tokio::test]
    async fn test_scan() {
        let fs = Arc::new(RealFileSystem::new());
        let abs_path = Arc::from(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("TestCollection"),
        );

        let worktree = Worktree::new(fs, Arc::new(AtomicUsize::new(0)), abs_path);
        worktree.initial_scan().await.unwrap();

        let snapshot = worktree.snapshot.read().await;

        for (_, entry) in snapshot.iter_entries_by_prefix("") {
            println!("{}", entry.path().to_path_buf().display());
        }
    }
}
