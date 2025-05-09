mod common;
pub mod snapshot;

use anyhow::{Context, Result};
use common::ROOT_PATH;
use moss_fs::{CreateOptions, FileSystem, RemoveOptions, RenameOptions};
use std::{
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use tokio::sync::{Mutex, mpsc};

use crate::models::{
    primitives::EntryId,
    types::{EntryKind, PathChangeKind},
};

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
    snapshot: Arc<Mutex<Snapshot>>,
    prev_snapshot: Arc<Mutex<Snapshot>>,
}

impl Deref for Worktree {
    type Target = Arc<Mutex<Snapshot>>;

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
            snapshot: Arc::new(Mutex::new(initial_snapshot.clone())),
            prev_snapshot: Arc::new(Mutex::new(initial_snapshot)),
        }
    }

    pub async fn snapshot(&self) -> &Arc<Mutex<Snapshot>> {
        &self.snapshot
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

        let snapshot = self.snapshot.lock().await;
        if path.file_name().is_some() {
            Ok(snapshot.abs_path().join(path))
        } else {
            Ok(snapshot.abs_path().to_path_buf())
        }
    }

    pub async fn relativize(&self, abs_path: &Path) -> Result<PathBuf> {
        debug_assert!(abs_path.is_absolute());

        let snapshot = self.snapshot.lock().await;
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
            .unwrap_or_else(|_| PathBuf::from(ROOT_PATH)))
    }

    pub async fn create_entry(
        &self,
        path: impl AsRef<Path>,
        is_dir: bool,
        content: Option<Vec<u8>>,
    ) -> Result<ChangesDiffSet> {
        let path: Arc<Path> = path.as_ref().into();
        debug_assert!(path.is_relative());

        let abs_path = self.absolutize(&path).await?;
        let changes = if is_dir {
            let first_segment = path.components().next().context("Path is empty")?;
            let lowest_ancestor_path = self.snapshot.lock().await.lowest_ancestor_path(&path);

            self.fs.create_dir(&abs_path).await?;

            let first_segment_path: Arc<Path> = lowest_ancestor_path.join(first_segment).into();
            let first_segment_abs_path = self.absolutize(&first_segment_path).await?;
            let first_segment_metadata = tokio::fs::metadata(&first_segment_abs_path)
                .await
                .context("Failed to get metadata for first segment")?;
            let first_segment_entry = Arc::new(Entry {
                id: EntryId::new(&self.next_entry_id),
                path: Arc::clone(&first_segment_path),
                kind: EntryKind::Dir,
                unit_type: None,
                mtime: first_segment_metadata.modified().ok(),
                file_id: file_id::get_file_id(&first_segment_abs_path)?,
            });

            {
                let mut snapshot_lock = self.snapshot.lock().await;
                snapshot_lock.create_entry(Arc::clone(&first_segment_entry));
            }

            let entries = self.scan(lowest_ancestor_path.join(first_segment)).await?;
            let mut changes = vec![(
                first_segment_path,
                first_segment_entry.id,
                PathChangeKind::Created,
            )];

            let mut snapshot_lock = self.snapshot.lock().await;
            for e in entries.into_iter().map(Arc::new) {
                changes.push((Arc::clone(&e.path), e.id, PathChangeKind::Created));
                snapshot_lock.create_entry(e);
            }

            changes
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

            let metadata = tokio::fs::metadata(&abs_path).await?;
            let entry = Arc::new(Entry {
                id: EntryId::new(&self.next_entry_id),
                path: Arc::clone(&path),
                kind: EntryKind::File,
                unit_type: None,
                mtime: metadata.modified().ok(),
                file_id: file_id::get_file_id(&abs_path)?,
            });

            let mut snapshot_lock = self.snapshot.lock().await;
            snapshot_lock.create_entry(Arc::clone(&entry));

            vec![(path.into(), entry.id, PathChangeKind::Created)]
        };

        Ok(ChangesDiffSet::from(changes))
    }

    pub async fn remove_entry(&self, path: impl AsRef<Path>) -> Result<ChangesDiffSet> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        let abs_path = self.absolutize(&path).await?;

        let mut snapshot_lock = self.snapshot.lock().await;

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
    ) -> Result<ChangesDiffSet> {
        let old_path = old_path.as_ref();
        let new_path = new_path.as_ref();
        debug_assert!(old_path.is_relative());
        debug_assert!(new_path.is_relative());

        let abs_old_path = self.absolutize(old_path).await?;
        let abs_new_path = self.absolutize(new_path).await?;

        let mut snapshot_lock = self.snapshot.lock().await;

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

        let removed_entries = snapshot_lock.remove_entry(old_path);

        // Scan new path and add the entries with Created kind
        // let new_entries = self.scan(new_path).await?;
        // for entry in new_entries {
        //     let entry_ref = Arc::new(entry);
        //     changes.push((
        //         entry_ref.path.clone(),
        //         entry_ref.id,
        //         PathChangeKind::Created,
        //     ));
        //     snapshot_lock.create_entry(entry_ref);
        // }

        // Ok(ChangesDiffSet::from(changes))

        todo!()
    }

    pub async fn initial_scan(&self) -> Result<()> {
        let entries = self.scan(ROOT_PATH).await?;
        let mut snapshot_lock = self.snapshot.lock().await;
        for entry in entries {
            snapshot_lock.create_entry(entry.into());
        }

        Ok(())
    }

    pub async fn scan(&self, path: impl AsRef<Path>) -> Result<Vec<Entry>> {
        let path: Arc<Path> = path.as_ref().into();
        debug_assert!(path.is_relative());

        let abs_path: Arc<Path> = self.absolutize(&path).await?.into();
        let (scan_job_tx, mut scan_job_rx) = mpsc::unbounded_channel();

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

        let worktree = Worktree::new(fs, abs_path, Arc::new(AtomicUsize::new(0)));
        worktree.initial_scan().await.unwrap();

        let snapshot = worktree.snapshot.lock().await;

        for (_, entry) in snapshot.iter_entries_by_prefix("") {
            println!("{}", entry.path.to_path_buf().display());
        }
    }
}
