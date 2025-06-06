pub mod snapshot;

use anyhow::Context;
use moss_common::api::OperationError;
use moss_file::toml::EditableInPlaceFileHandle;
use moss_fs::FileSystem;

use snapshot::{Entry, Snapshot, UnloadedEntry};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use thiserror::Error;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    configuration::{ConfigurationModel, DirConfigurationModel, SpecificationMetadata},
    models::{primitives::ChangesDiffSet, types::PathChangeKind},
    worktree::{constants::*, snapshot::UnloadedParentId},
};

pub mod constants {
    use uuid::Uuid;

    use crate::worktree::snapshot::UnloadedId;

    pub(crate) const CONFIG_FILE_NAME_ITEM: &str = "config.toml";
    pub(crate) const CONFIG_FILE_NAME_DIR: &str = "config-folder.toml";

    pub(crate) const ROOT_PATH: &str = "";
    pub(crate) const ROOT_ID: Uuid = Uuid::nil();
    pub(crate) const ROOT_UNLOADED_ID: UnloadedId = 0;
}

#[derive(Error, Debug)]
pub enum WorktreeError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("worktree entry already exists: {0}")]
    AlreadyExists(String),

    #[error("worktree entry is not found: {0}")]
    NotFound(String),

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl From<WorktreeError> for OperationError {
    fn from(error: WorktreeError) -> Self {
        match error {
            WorktreeError::InvalidInput(err) => OperationError::InvalidInput(err),
            WorktreeError::AlreadyExists(err) => OperationError::AlreadyExists(err),
            WorktreeError::NotFound(err) => OperationError::NotFound(err),
            WorktreeError::Unknown(err) => OperationError::Unknown(err),
        }
    }
}

pub type WorktreeResult<T> = Result<T, WorktreeError>;

struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    parent_unloaded_id: Option<usize>,
    scan_queue: mpsc::UnboundedSender<ScanJob>,
}

pub struct Worktree {
    abs_path: Arc<Path>,
    snapshot: Snapshot,
    fs: Arc<dyn FileSystem>,
}

impl Worktree {
    pub async fn new(fs: Arc<dyn FileSystem>, abs_path: Arc<Path>) -> Result<Self, WorktreeError> {
        debug_assert!(abs_path.is_absolute());

        let next_unloaded_id = Arc::new(AtomicUsize::new(ROOT_UNLOADED_ID + 1));
        let mut unloaded_entries = vec![(
            UnloadedEntry::Dir {
                id: ROOT_UNLOADED_ID,
                path: Path::new(ROOT_PATH).into(),
                abs_path: Arc::clone(&abs_path),
            },
            None,
        )];
        unloaded_entries.extend(
            Self::scan(
                fs.clone(),
                &abs_path,
                &Path::new(ROOT_PATH),
                next_unloaded_id.clone(),
            )
            .await?,
        );

        let snapshot = Snapshot::from(unloaded_entries);

        Ok(Self {
            abs_path,
            fs,
            snapshot,
        })
    }

    async fn scan(
        fs: Arc<dyn FileSystem>,
        abs_path: &Path,
        path: &Path,
        next_unloaded_id: Arc<AtomicUsize>,
    ) -> WorktreeResult<Vec<(UnloadedEntry, Option<UnloadedParentId>)>> {
        debug_assert!(path.is_relative());
        debug_assert!(abs_path.is_absolute());

        let path: Arc<Path> = path.into();
        let scanned_abs_path: Arc<Path> = Self::absolutize(&abs_path, &path)?.into();
        let (scan_job_tx, mut scan_job_rx) = mpsc::unbounded_channel();

        let initial_job = ScanJob {
            abs_path: Arc::clone(&scanned_abs_path),
            path: Arc::clone(&path),
            parent_unloaded_id: None,
            scan_queue: scan_job_tx.clone(),
        };
        scan_job_tx.send(initial_job).unwrap();

        drop(scan_job_tx);

        let mut handles = Vec::new();
        while let Some(job) = scan_job_rx.recv().await {
            let fs_clone = fs.clone();
            let next_unloaded_id = next_unloaded_id.clone();

            let handle = tokio::spawn(async move {
                let mut new_jobs = Vec::new();
                let mut new_entries = Vec::new();

                let mut read_dir = fs_clone.read_dir(&job.abs_path).await.unwrap();

                let mut child_paths = Vec::new();
                while let Some(dir_entry) = read_dir.next_entry().await.unwrap_or(None) {
                    child_paths.push(dir_entry);
                }

                for child_entry in child_paths {
                    let child_file_type = match child_entry.file_type().await {
                        Ok(file_type) => file_type,
                        Err(err) => {
                            println!("Error reading file type: {}", err);
                            continue;
                        }
                    };

                    if !child_file_type.is_dir() {
                        continue;
                    }

                    let child_abs_path: Arc<Path> = child_entry.path().into();
                    let child_name = child_abs_path.file_name().unwrap();
                    let child_path: Arc<Path> = job.path.join(child_name).into();

                    let unloaded_entry: UnloadedEntry;
                    if child_abs_path.join(CONFIG_FILE_NAME_DIR).exists() {
                        unloaded_entry = UnloadedEntry::Dir {
                            id: next_unloaded_id.fetch_add(1, Ordering::Relaxed),
                            path: Arc::clone(&child_path),
                            abs_path: Arc::clone(&child_abs_path),
                        };
                    } else if child_abs_path.join(CONFIG_FILE_NAME_ITEM).exists() {
                        unloaded_entry = UnloadedEntry::Item {
                            id: next_unloaded_id.fetch_add(1, Ordering::Relaxed),
                            path: Arc::clone(&child_path),
                            abs_path: Arc::clone(&child_abs_path),
                        };
                    } else {
                        continue;
                    }

                    let unloaded_entry_id = unloaded_entry.id();
                    new_entries.push((unloaded_entry, job.parent_unloaded_id));

                    new_jobs.push(ScanJob {
                        abs_path: Arc::clone(&child_abs_path),
                        path: child_path,
                        parent_unloaded_id: Some(unloaded_entry_id),
                        scan_queue: job.scan_queue.clone(),
                    });
                }

                for new_job in new_jobs {
                    job.scan_queue.send(new_job).unwrap();
                }

                new_entries
            });

            handles.push(handle);
        }

        Ok(futures::future::join_all(handles)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .map_err(anyhow::Error::from)?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>())
    }

    pub fn absolutize(abs_path: &Path, path: &Path) -> WorktreeResult<PathBuf> {
        debug_assert!(abs_path.is_absolute());
        debug_assert!(path.is_relative());

        if path
            .components()
            .any(|c| c == std::path::Component::ParentDir)
        {
            return Err(WorktreeError::InvalidInput(format!(
                "Path cannot contain '..' components: {}",
                path.display()
            )));
        }

        if path.file_name().is_some() {
            Ok(abs_path.join(path))
        } else {
            Ok(abs_path.to_path_buf())
        }
    }

    pub async fn load_entry(&mut self, path: &Path, depth: u8) -> WorktreeResult<ChangesDiffSet> {
        let sanitized_path = moss_fs::utils::sanitize_path(path, None)?;
        let changes = self.load_entry_internal(&sanitized_path, depth).await?;
        Ok(ChangesDiffSet::from(changes))
    }

    async fn load_entry_internal(
        &mut self,
        path: &Path,
        depth: u8,
    ) -> WorktreeResult<Vec<(Arc<Path>, Uuid, PathChangeKind)>> {
        let mut changes = Vec::new();

        let ancestors_to_load = self.collect_ancestors_to_load(&path);
        for ancestor_path in &ancestors_to_load {
            if !self.snapshot.is_loaded(ancestor_path) {
                let ancestor_id = self.load_single_entry(ancestor_path.clone().into()).await?;
                changes.push((
                    ancestor_path.clone().into(),
                    ancestor_id,
                    PathChangeKind::Loaded,
                ));
            }
        }

        for ancestor_path in ancestors_to_load.iter().filter(|p| p != &path) {
            changes.extend(self.load_entry_children(ancestor_path).await?);
        }

        let path: Arc<Path> = path.into();
        if !self.snapshot.is_loaded(&path) {
            let id = self.load_single_entry(path.clone()).await?;
            changes.push((path.clone(), id, PathChangeKind::Loaded));
        }

        if depth > 0 {
            self.load_children_at_depth(path, depth, &mut changes)
                .await?;
        }

        Ok(changes)
    }

    fn collect_ancestors_to_load(&self, path: &Path) -> Vec<PathBuf> {
        let mut ancestors_to_load = Vec::new();
        let mut current_path = path;

        // Collect unloaded ancestors
        while !current_path.as_os_str().is_empty() {
            if !self.snapshot.is_loaded(current_path) {
                ancestors_to_load.push(current_path.to_path_buf());
            }
            current_path = current_path.parent().unwrap_or(Path::new(""));
        }

        // Ensure root is included if not loaded
        if !self.snapshot.is_loaded(Path::new("")) {
            ancestors_to_load.push(PathBuf::from(""));
        }

        ancestors_to_load.reverse();
        ancestors_to_load
    }

    async fn load_children_at_depth(
        &mut self,
        path: Arc<Path>,
        depth: u8,
        changes: &mut Vec<(Arc<Path>, Uuid, PathChangeKind)>,
    ) -> WorktreeResult<()> {
        let mut current_level = vec![path];

        for _ in 0..depth {
            let mut next_level = Vec::new();

            for current_path in current_level {
                let children_changes = self.load_entry_children(&current_path).await?;
                next_level.extend(children_changes.iter().map(|(path, _, _)| path.clone()));
                changes.extend(children_changes);
            }

            current_level = next_level;
            if current_level.is_empty() {
                break;
            }
        }

        Ok(())
    }

    async fn load_entry_children(
        &mut self,
        parent_path: &Path,
    ) -> WorktreeResult<Vec<(Arc<Path>, Uuid, PathChangeKind)>> {
        let unloaded_children = self.snapshot.unloaded_entry_children_by_path(parent_path);
        let mut changes = Vec::with_capacity(unloaded_children.len());

        for child in unloaded_children {
            let child_path: Arc<Path> = Arc::clone(child.path());
            if !self.snapshot.is_loaded(&child_path) {
                let child_id = self.load_single_entry(child_path.clone()).await?;
                changes.push((child_path, child_id, PathChangeKind::Loaded));
            }
        }

        Ok(changes)
    }

    async fn load_single_entry(&mut self, path: Arc<Path>) -> WorktreeResult<Uuid> {
        let unloaded_entry = self
            .snapshot
            .unloaded_entry_by_path(&path)
            .ok_or(WorktreeError::NotFound(path.display().to_string()))?;

        let entry = if unloaded_entry.is_root() {
            Entry {
                id: ROOT_ID,
                path,
                config: None,
            }
        } else {
            let config_path = match unloaded_entry {
                UnloadedEntry::Item { abs_path, .. } => abs_path.join(CONFIG_FILE_NAME_ITEM),
                UnloadedEntry::Dir { abs_path, .. } => abs_path.join(CONFIG_FILE_NAME_DIR),
            };

            let config = EditableInPlaceFileHandle::<ConfigurationModel>::load(
                self.fs.clone(),
                &config_path,
            )
            .await?;

            let id = config.model().await.id();
            Entry {
                id,
                path,
                config: Some(config),
            }
        };

        Ok(self.snapshot.load_entry(unloaded_entry.id(), entry)?)
    }

    pub async fn create_entry(
        &mut self,
        path: &Path,
        config: ConfigurationModel,
    ) -> WorktreeResult<ChangesDiffSet> {
        assert!(path.is_relative());

        if path.file_name().is_none() {
            return Err(WorktreeError::InvalidInput(format!(
                "Target name is not a valid: {}",
                path.display()
            )));
        }

        let encoded_path = moss_fs::utils::sanitize_path(path, None)?;
        let lowest_ancestor_path = self.snapshot.lowest_ancestor_path(&encoded_path);

        let mut changes = vec![];
        changes.extend(self.load_entry_internal(&lowest_ancestor_path, 1).await?);

        let mut deepest_loaded_ancestor = Path::new("");
        for ancestor in encoded_path.ancestors() {
            if self.snapshot.is_loaded(ancestor) {
                deepest_loaded_ancestor = ancestor;
                break;
            }
        }

        let deepest_loaded_ancestor_id = self
            .snapshot
            .entry_by_path(deepest_loaded_ancestor)
            .expect("Deepest loaded ancestor path must be loaded")
            .id();

        let missing_part = encoded_path
            .strip_prefix(deepest_loaded_ancestor)
            .expect("Deepest loaded ancestor path must be a prefix of path");

        if missing_part.as_os_str().is_empty() {
            return Err(WorktreeError::AlreadyExists(
                encoded_path.display().to_string(),
            ));
        }

        dbg!(&missing_part);
        dbg!(&path);

        let mut current_level = deepest_loaded_ancestor.to_path_buf();
        let mut next_parent_id = deepest_loaded_ancestor_id;
        let last_component = missing_part
            .components()
            .last()
            // An error here indicates a bug in the implementation logic.
            .context("Missing part should have at least one component")?;

        // We intentionally use a `Option` to track the input config
        // and guarantee that the input config is consumed only once.
        let mut config = Some(config);

        for component in missing_part.components() {
            current_level.push(component.as_os_str());

            // Check if this entry already exists
            if self.snapshot.is_loaded(&current_level) {
                // Entry already exists, just get its ID and continue
                let existing_entry = self
                    .snapshot
                    .entry_by_path(&current_level)
                    .expect("Entry should exist since is_loaded returned true");
                next_parent_id = existing_entry.id();
                continue;
            }

            let current_config = if component == last_component {
                // Should never happen. The input config should be consumed by the last component only.
                // A panic here indicates a bug in the implementation logic.
                debug_assert!(config.is_some());

                config.take().context("The input config was consumed")?
            } else {
                let id = Uuid::new_v4();
                ConfigurationModel::Dir(DirConfigurationModel {
                    metadata: SpecificationMetadata { id },
                })
            };
            let id = current_config.id();

            let abs_path = Self::absolutize(&self.abs_path, &current_level)?;
            self.fs.create_dir(&abs_path).await?;

            let file_name = match current_config {
                ConfigurationModel::Item(_) => CONFIG_FILE_NAME_ITEM,
                ConfigurationModel::Dir(_) => CONFIG_FILE_NAME_DIR,
            };
            let config_handle = EditableInPlaceFileHandle::create(
                self.fs.clone(),
                &abs_path.join(file_name),
                current_config,
            )
            .await
            .context("Failed to create config file")?;

            let entry_path: Arc<Path> = current_level.clone().into();
            let entry = Entry {
                id,
                path: entry_path.clone(),
                config: Some(config_handle),
            };

            self.snapshot.create_entry(entry, Some(next_parent_id))?;
            changes.push((entry_path, id, PathChangeKind::Created));

            next_parent_id = id;
        }

        Ok(ChangesDiffSet::from(changes))
    }
}

#[cfg(test)]
mod tests {
    use moss_fs::RealFileSystem;

    use crate::configuration::ItemConfigurationModel;

    use super::*;

    #[tokio::test]
    async fn test_scan() {
        let fs = Arc::new(RealFileSystem::new());
        let mut worktree = Worktree::new(
            fs,
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("data/test")
                .into(),
        )
        .await
        .unwrap();

        // worktree.load_entry(Path::new(""), 1).await.unwrap();

        // worktree.load_entry(Path::new("qux"), 1).await.unwrap();

        let changes = worktree.load_entry(Path::new("foo"), 1).await.unwrap();

        println!("{}", worktree.snapshot);
        for change in changes.iter() {
            println!("{:?}", change);
        }
    }

    #[tokio::test]
    async fn test_create_entry() {
        let fs = Arc::new(RealFileSystem::new());
        let mut worktree = Worktree::new(
            fs,
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("data")
                .into(),
        )
        .await
        .unwrap();

        let changes = worktree
            .create_entry(
                Path::new("test/foo/bar"),
                ConfigurationModel::Item(ItemConfigurationModel {
                    metadata: SpecificationMetadata { id: Uuid::new_v4() },
                }),
            )
            .await
            .unwrap();

        dbg!(&changes);

        let changes = worktree
            .create_entry(
                Path::new("test/foo/baz"),
                ConfigurationModel::Item(ItemConfigurationModel {
                    metadata: SpecificationMetadata { id: Uuid::new_v4() },
                }),
            )
            .await
            .unwrap();

        let changes = worktree
            .create_entry(
                Path::new("test/foo/baz/pax"),
                ConfigurationModel::Item(ItemConfigurationModel {
                    metadata: SpecificationMetadata { id: Uuid::new_v4() },
                }),
            )
            .await
            .unwrap();

        let changes = worktree
            .create_entry(
                Path::new("test/qux"),
                ConfigurationModel::Item(ItemConfigurationModel {
                    metadata: SpecificationMetadata { id: Uuid::new_v4() },
                }),
            )
            .await
            .unwrap();

        println!("{}", worktree.snapshot);
    }

    #[tokio::test]
    async fn test_load_with_parent_children() {
        let fs = Arc::new(RealFileSystem::new());
        let mut worktree = Worktree::new(
            fs,
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests")
                .join("data/test")
                .into(),
        )
        .await
        .unwrap();

        // Load only foo/baz with depth 0 (no children of target)
        let changes = worktree.load_entry(Path::new("foo/baz"), 0).await.unwrap();

        println!("=== Snapshot after loading foo/baz ===");
        println!("{}", worktree.snapshot);

        println!("=== Changes ===");
        for change in changes.iter() {
            println!("{:?}", change);
        }

        // Verify that:
        // 1. Root is loaded
        // 2. All children of root are loaded (foo, qux)
        // 3. Target foo/baz is loaded
        // 4. Children of foo/baz are NOT loaded (depth=0)

        assert!(worktree.snapshot.is_loaded(Path::new(""))); // root
        assert!(worktree.snapshot.is_loaded(Path::new("foo"))); // child of root
        assert!(worktree.snapshot.is_loaded(Path::new("qux"))); // child of root
        assert!(worktree.snapshot.is_loaded(Path::new("foo/baz"))); // target
        assert!(!worktree.snapshot.is_loaded(Path::new("foo/baz/pax"))); // child of target (should not be loaded)
    }
}
