pub mod common;
pub mod snapshot;

use crate::models::{
    primitives::EntryId,
    types::{EntryKind, PathChangeKind},
};
use anyhow::{Context, Result};
use common::ROOT_PATH;
use moss_common::api::{OperationError, OperationResult};
use moss_fs::{CreateOptions, FileSystem, RemoveOptions, RenameOptions};
use std::{
    collections::HashMap,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use tokio::sync::{RwLock, mpsc};

use self::snapshot::{Entry, Snapshot};

pub(crate) type ChangesDiffSet = Arc<[(Arc<Path>, EntryId, PathChangeKind)]>;

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

impl Deref for Worktree {
    type Target = Arc<RwLock<Snapshot>>;

    fn deref(&self) -> &Self::Target {
        &self.snapshot
    }
}

impl Worktree {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        abs_path: Arc<Path>,
        next_entry_id: Arc<AtomicUsize>,
    ) -> Self {
        debug_assert!(abs_path.is_absolute());

        let initial_snapshot = Snapshot::new(abs_path);
        Self {
            fs,
            next_entry_id,
            snapshot: Arc::new(RwLock::new(initial_snapshot.clone())),
        }
    }

    pub async fn snapshot(&self) -> &Arc<RwLock<Snapshot>> {
        &self.snapshot
    }

    pub async fn absolutize(&self, root_abs_path: &Path, path: &Path) -> Result<PathBuf> {
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

        if path.file_name().is_some() {
            Ok(root_abs_path.join(path))
        } else {
            Ok(root_abs_path.to_path_buf())
        }
    }

    pub async fn relativize(&self, root_abs_path: &Path, abs_path: &Path) -> Result<PathBuf> {
        debug_assert!(abs_path.is_absolute());

        if !abs_path.starts_with(root_abs_path) {
            return Err(anyhow::anyhow!(
                "Path {} is outside of the worktree root {}",
                abs_path.display(),
                root_abs_path.display()
            ));
        }

        Ok(abs_path
            .strip_prefix(root_abs_path)
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|_| PathBuf::from(ROOT_PATH)))
    }

    pub async fn create_entry(
        &self,
        path: impl AsRef<Path>,
        is_dir: bool,
        content: Option<Vec<u8>>,
    ) -> OperationResult<ChangesDiffSet> {
        let path: Arc<Path> = path.as_ref().into();
        debug_assert!(path.is_relative());
        let root_abs_path = self.snapshot.read().await.abs_path().clone();
        let abs_path = self.absolutize(&root_abs_path, &path).await?;
        if abs_path.exists() {
            return OperationResult::Err(OperationError::AlreadyExists {
                name: path
                    .file_name()
                    .context("Entry name should not be empty")?
                    .to_string_lossy()
                    .to_string(),
                path: abs_path,
            });
        }
        let changes = if is_dir {
            let lowest_ancestor_path = self.snapshot.read().await.lowest_ancestor_path(&path);

            // Scanning from the highest entry that is newly created
            // Extract part of the path that's one component more than the lowest ancestor path
            // Example LAP: \requests, path: \requests\1\1.request => scan: \requests\1

            let new_level = path
                .strip_prefix(&lowest_ancestor_path)
                .expect("Lowest ancestor path must be a prefix of path")
                .components()
                .next()
                .expect("The input path must contain a unique new level");
            let scan_path = lowest_ancestor_path.join(new_level);

            self.fs.create_dir(&abs_path).await?;

            let entries = self.scan(&root_abs_path, scan_path).await?;
            let mut changes = vec![];

            {
                let mut snapshot_lock = self.snapshot.write().await;
                for e in entries.into_iter().map(Arc::new) {
                    changes.push((Arc::clone(&e.path), e.id, PathChangeKind::Created));
                    snapshot_lock.create_entry(e);
                }
            }

            changes
        } else {
            self.fs
                .create_file_with(
                    &abs_path,
                    String::from_utf8(content.as_deref().unwrap_or(&[]).to_vec())
                        .context("Content is not valid utf8 bytes.")?,
                    CreateOptions {
                        overwrite: true,
                        ignore_if_exists: false,
                    },
                )
                .await?;

            let metadata = tokio::fs::metadata(&abs_path).await.context(format!(
                "Unable to get metadata for path {}",
                abs_path.display()
            ))?;
            let entry = Arc::new(Entry {
                id: EntryId::new(&self.next_entry_id),
                path: Arc::clone(&path),
                kind: EntryKind::File,
                unit_type: None,
                mtime: metadata.modified().ok(),
                file_id: file_id::get_file_id(&abs_path).context(format!(
                    "Unable to get file id for path {}",
                    abs_path.display()
                ))?,
            });

            {
                let mut snapshot_lock = self.snapshot.write().await;
                snapshot_lock.create_entry(Arc::clone(&entry));
            }

            vec![(path.into(), entry.id, PathChangeKind::Created)]
        };

        Ok(ChangesDiffSet::from(changes))
    }

    pub async fn remove_entry(&self, path: impl AsRef<Path>) -> Result<ChangesDiffSet> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        let root_abs_path = self.snapshot.read().await.abs_path().clone();
        let abs_path = self.absolutize(&root_abs_path, &path).await?;

        let mut snapshot_lock = self.snapshot.write().await;

        // Skip fs operation if it's already deleted
        if abs_path.exists() {
            if abs_path.is_dir() {
                let mut temp_dir = abs_path.clone();
                let original_name = temp_dir.file_name().unwrap().to_string_lossy().to_string();
                let temp_name = format!("{}.deleted.{}", original_name, std::process::id());
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
        }

        let removed_entries = snapshot_lock.remove_entry(path);
        drop(snapshot_lock);

        let changes = removed_entries
            .into_iter()
            .map(|entry| (entry.path.clone(), entry.id, PathChangeKind::Removed))
            .collect::<Vec<_>>();

        Ok(ChangesDiffSet::from(changes))
    }

    pub async fn rename_entry(
        &self,
        old_path: impl AsRef<Path>,
        new_path: impl AsRef<Path>,
    ) -> OperationResult<ChangesDiffSet> {
        let old_path = old_path.as_ref();
        let new_path = new_path.as_ref();
        debug_assert!(old_path.is_relative());
        debug_assert!(new_path.is_relative());

        let root_abs_path = self.snapshot.read().await.abs_path().clone();
        let abs_old_path = self.absolutize(&root_abs_path, &old_path).await?;
        let abs_new_path = self.absolutize(&root_abs_path, &new_path).await?;

        if abs_new_path.exists() {
            return OperationResult::Err(OperationError::AlreadyExists {
                name: new_path.file_name().unwrap().to_string_lossy().to_string(),
                path: abs_new_path,
            });
        }
        if !abs_old_path.exists() {
            return OperationResult::Err(OperationError::NotFound {
                name: old_path.file_name().unwrap().to_string_lossy().to_string(),
                path: abs_old_path,
            });
        }
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

        let mut changes = Vec::new();
        let mut removed_entries_by_file_id = snapshot_lock
            .remove_entry(old_path)
            .into_iter()
            .map(|e| (e.file_id, e))
            .collect::<HashMap<_, _>>();

        let changed_entries = self.scan(&root_abs_path, new_path).await?;

        for entry in changed_entries {
            let (entry, change) =
                if let Some(removed_entry) = removed_entries_by_file_id.remove(&entry.file_id) {
                    let entry = reuse_id(&removed_entry, entry);
                    (Arc::new(entry), PathChangeKind::Updated)
                } else {
                    (Arc::new(entry), PathChangeKind::Created)
                };

            changes.push((entry.path.clone(), entry.id, change));
            snapshot_lock.create_entry(entry);
        }

        Ok(ChangesDiffSet::from(changes))
    }

    pub async fn initial_scan(&self) -> Result<()> {
        let root_abs_path = self.snapshot.read().await.abs_path().clone();
        let entries = self.scan(&root_abs_path, ROOT_PATH).await?;
        {
            let mut snapshot_lock = self.snapshot.write().await;
            for entry in entries {
                snapshot_lock.create_entry(entry.into());
            }
        }

        Ok(())
    }

    pub async fn scan(&self, root_abs_path: &Path, path: impl AsRef<Path>) -> Result<Vec<Entry>> {
        let path: Arc<Path> = path.as_ref().into();
        debug_assert!(path.is_relative());

        let abs_path: Arc<Path> = self.absolutize(&root_abs_path, &path).await?.into();
        let (scan_job_tx, mut scan_job_rx) = mpsc::unbounded_channel();

        let initial_job = ScanJob {
            abs_path: Arc::clone(&abs_path),
            path: Arc::clone(&path),
            scan_queue: scan_job_tx.clone(),
        };
        scan_job_tx.send(initial_job).unwrap();

        drop(scan_job_tx);

        let mut handles = Vec::new();
        while let Some(job) = scan_job_rx.recv().await {
            let fs_clone = self.fs.clone();
            let next_entry_id = self.next_entry_id.clone();

            let handle = tokio::spawn(async move {
                let mut new_entries = vec![];
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

        let metadata = tokio::fs::metadata(&abs_path)
            .await
            .expect("Failed to get scan job abs path metadata");
        let file_id =
            file_id::get_file_id(&abs_path).expect("Failed to get scan job abs path file id");

        let next_entry_id = self.next_entry_id.clone();
        let entry = Entry {
            id: EntryId::new(&next_entry_id),
            path,
            kind: if metadata.is_dir() {
                EntryKind::Dir
            } else {
                EntryKind::File
            },
            unit_type: None,
            mtime: metadata.modified().ok(),
            file_id,
        };

        Ok(std::iter::once(entry)
            .chain(
                futures::future::join_all(handles)
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .flatten(),
            )
            .collect())
    }
}

fn reuse_id(old_entry: &Entry, mut new_entry: Entry) -> Entry {
    new_entry.id = old_entry.id;
    new_entry
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

        let worktree = Worktree::new(fs, abs_path, Arc::new(AtomicUsize::new(0)));
        worktree.initial_scan().await.unwrap();

        let snapshot = worktree.snapshot.read().await;

        for (_, entry) in snapshot.iter_entries_by_prefix("") {
            println!("{}", entry.path.to_path_buf().display());
        }
    }
}
