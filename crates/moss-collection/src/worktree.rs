//! # Worktree Module
//!
//! Manages a hierarchical file system structure with lazy loading capabilities.
//! The `Worktree` represents a tree-like structure of entries that can be loaded on-demand
//! from the file system.
//!
//! The worktree consists of two types of entries:
//! - **Unloaded entries**: Lightweight representations discovered during scanning
//! - **Loaded entries**: Full objects with configuration data loaded from disk
//!
//! Entries are initially discovered as "unloaded" during file system scanning. They are only
//! fully loaded (with configuration data) when explicitly requested, enabling efficient handling
//! of large directory structures.
//!
//! The worktree is internally represented as a directed graph where:
//! - **Root node**: Every worktree has exactly one root node (with UUID `Uuid::nil()`) that serves as the starting point
//! - **Parent-child relationships**: All loaded nodes in the graph must have a parent, except for the root node
//! - **Single parent constraint**: Each node can have at most one parent (no multiple inheritance)
//! - **Acyclic property**: The graph must not contain cycles - no node can be its own ancestor
//! - **Graph invariant**: If a non-root node appears in the loaded graph without a parent, this indicates a bug in the implementation
//! - **Tree property**: The graph maintains a tree structure where each node has at most one parent
//!
//! This graph structure ensures proper hierarchical relationships and enables efficient tree traversal operations.
//! All operations return `ChangesDiffSetNew` objects that describe what changes occurred,
//! enabling UI updates and synchronization.

pub mod snapshot;
pub mod walker;

use anyhow::Context;
use moss_common::api::OperationError;
use moss_file::toml::EditableInPlaceFileHandle;
use moss_fs::{FileSystem, RenameOptions};
use snapshot::{Entry, Snapshot, UnloadedEntry};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use thiserror::Error;
use tokio::{sync::mpsc, task::JoinHandle};
use uuid::Uuid;

use crate::{
    configuration::{ConfigurationModel, DirConfigurationModel, SpecificationMetadata},
    models::primitives::{WorktreeChange, WorktreeDiff},
    worktree::{
        constants::*,
        snapshot::{ParentId, UnloadedParentId},
    },
};

pub mod constants {
    use uuid::Uuid;

    use crate::worktree::snapshot::UnloadedId;

    pub const CONFIG_FILE_NAME_ITEM: &str = "config.toml";
    pub const CONFIG_FILE_NAME_DIR: &str = "config-folder.toml";

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

/// Internal structure for managing file system scanning jobs.
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
    background_tasks: Vec<JoinHandle<()>>,
}

impl Worktree {
    /// Creates a new worktree by scanning the specified directory.
    ///
    /// This performs an initial scan of the file system to discover all entries
    /// with configuration files, but doesn't load their full configuration data.
    pub async fn new(fs: Arc<dyn FileSystem>, abs_path: Arc<Path>) -> WorktreeResult<Self> {
        debug_assert!(abs_path.is_absolute());

        let next_unloaded_id = Arc::new(AtomicUsize::new(ROOT_UNLOADED_ID + 1));

        let unloaded_entries = Self::scan(
            fs.clone(),
            &abs_path,
            &Path::new(ROOT_PATH),
            next_unloaded_id.clone(),
        )
        .await?;

        let snapshot = Snapshot::from(unloaded_entries);

        let mut worktree = Self {
            abs_path,
            fs,
            snapshot,
            background_tasks: Vec::new(),
        };

        // Automatically load the root entry
        worktree
            .load_single_entry(Path::new(ROOT_PATH).into())
            .await?;

        Ok(worktree)
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

        let mut result = Vec::new();
        let current_entry_id = if path.as_os_str().is_empty() {
            ROOT_UNLOADED_ID
        } else {
            next_unloaded_id.fetch_add(1, Ordering::Relaxed)
        };

        let current_entry = UnloadedEntry::Dir {
            id: current_entry_id,
            path: Arc::clone(&path),
        };

        // Root entry has no parent, others will get parent assigned by caller
        result.push((current_entry, None));

        let (scan_job_tx, mut scan_job_rx) = mpsc::unbounded_channel();

        let initial_job = ScanJob {
            abs_path: Arc::clone(&scanned_abs_path),
            path: Arc::clone(&path),
            parent_unloaded_id: Some(current_entry_id),
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
                        };
                    } else if child_abs_path.join(CONFIG_FILE_NAME_ITEM).exists() {
                        unloaded_entry = UnloadedEntry::Item {
                            id: next_unloaded_id.fetch_add(1, Ordering::Relaxed),
                            path: Arc::clone(&child_path),
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

        result.extend(
            futures::future::join_all(handles)
                .await
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
                .map_err(anyhow::Error::from)?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        );

        Ok(result)
    }

    // pub fn entry_by_path(&self, path: &Path) -> Option<&Entry> {}

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

    pub fn entry(&self, id: Uuid) -> Option<&Entry> {
        self.snapshot.entry_by_id(id)
    }

    /// Loads an entry and optionally its children from the file system.
    ///
    /// This method loads the full configuration data for the specified entry and,
    /// depending on the depth parameter, its descendants. If ancestors of the target
    /// entry are not yet loaded, they will be loaded automatically.
    pub async fn load_entry(&mut self, path: &Path, depth: u8) -> WorktreeResult<WorktreeDiff> {
        let sanitized_path = moss_fs::utils::sanitize_path(path, None)?;
        let changes = self.load_entry_internal(&sanitized_path, depth).await?;
        Ok(WorktreeDiff::from(changes))
    }

    async fn load_entry_internal(
        &mut self,
        path: &Path,
        depth: u8,
    ) -> WorktreeResult<Vec<WorktreeChange>> {
        debug_assert!(path.is_relative());

        let mut changes = Vec::new();

        // Load ancestors if they are not loaded
        for ancestor_path in path
            .ancestors()
            .skip(1)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            if self.snapshot.is_loaded(ancestor_path) {
                continue;
            }

            let ancestor_id = self.load_single_entry(ancestor_path.into()).await?;
            changes.push(WorktreeChange::Loaded {
                id: ancestor_id,
                path: ancestor_path.into(),
            });
        }

        // Load the target entry if it's not loaded
        if !self.snapshot.is_loaded(&path) {
            let id = self.load_single_entry(path.into()).await?;
            changes.push(WorktreeChange::Loaded {
                id,
                path: path.into(),
            });
        }

        // Load children recursively
        if depth > 0 {
            self.load_children_recursive(path.into(), depth, &mut changes)
                .await?;
        }

        Ok(changes)
    }

    async fn load_children_recursive(
        &mut self,
        path: Arc<Path>,
        depth: u8,
        changes: &mut Vec<WorktreeChange>,
    ) -> WorktreeResult<()> {
        let mut current_level = vec![path];

        for _ in 0..depth {
            let mut next_level = Vec::new();

            for parent_path in current_level {
                let children = self.load_children(&parent_path).await?;
                for child_change in children {
                    if let WorktreeChange::Loaded { path, .. } = &child_change {
                        next_level.push(path.clone());
                    }
                    changes.push(child_change);
                }
            }

            if next_level.is_empty() {
                break;
            }

            current_level = next_level;
        }

        Ok(())
    }

    async fn load_children(&mut self, parent_path: &Path) -> WorktreeResult<Vec<WorktreeChange>> {
        let unloaded_children = self.snapshot.unloaded_entry_children_by_path(parent_path);
        let mut changes = Vec::with_capacity(unloaded_children.len());

        for unloaded_child in unloaded_children {
            let child_path = unloaded_child.path().clone();
            if self.snapshot.is_loaded(&child_path) {
                continue;
            }

            let child_id = self.load_single_entry(child_path.clone()).await?;
            changes.push(WorktreeChange::Loaded {
                id: child_id,
                path: child_path,
            });
        }

        Ok(changes)
    }

    async fn load_single_entry(&mut self, path: Arc<Path>) -> WorktreeResult<Uuid> {
        debug_assert!(path.is_relative());

        let unloaded_entry = self
            .snapshot
            .unloaded_entry_by_path(&path)
            .ok_or(WorktreeError::NotFound(path.display().to_string()))?;

        let entry = if unloaded_entry.is_root() {
            Entry {
                id: ROOT_ID,
                name: "".to_string(),
                path,
                is_dir: true,
                config: None,
            }
        } else {
            let config_path = match unloaded_entry {
                UnloadedEntry::Item { path, .. } => Self::absolutize(
                    self.abs_path.as_ref(),
                    path.join(CONFIG_FILE_NAME_ITEM).as_path(),
                )?,
                UnloadedEntry::Dir { path, .. } => Self::absolutize(
                    self.abs_path.as_ref(),
                    path.join(CONFIG_FILE_NAME_DIR).as_path(),
                )?,
            };

            let config = EditableInPlaceFileHandle::<ConfigurationModel>::load(
                self.fs.clone(),
                &config_path,
            )
            .await?;

            let model = config.model().await;
            let id = model.id();
            let name = path
                .file_name()
                .ok_or_else(|| {
                    WorktreeError::InvalidInput(format!("Path is not a valid: {}", path.display()))
                })?
                .to_string_lossy()
                .to_string();

            Entry {
                id,
                name,
                path,
                is_dir: matches!(model, ConfigurationModel::Dir(_)),
                config: Some(config),
            }
        };

        Ok(self.snapshot.load_entry(unloaded_entry.id(), entry)?)
    }

    /// Creates a new entry in the worktree.
    ///
    /// This method creates a new entry (item or directory) with the specified configuration.
    /// If parent directories don't exist, they will be created automatically as directory
    /// entries with generated UUIDs.
    pub async fn create_entry(
        &mut self,
        path: &Path,
        config: ConfigurationModel,
    ) -> WorktreeResult<WorktreeDiff> {
        assert!(path.is_relative());

        let name = path
            .file_name()
            .ok_or_else(|| {
                WorktreeError::InvalidInput(format!(
                    "Target name is not a valid: {}",
                    path.display()
                ))
            })?
            .to_string_lossy()
            .to_string();

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
            .id;

        let missing_part = encoded_path
            .strip_prefix(deepest_loaded_ancestor)
            .expect("Deepest loaded ancestor path must be a prefix of path");

        if missing_part.as_os_str().is_empty() {
            return Err(WorktreeError::AlreadyExists(
                encoded_path.display().to_string(),
            ));
        }

        let mut current_level = deepest_loaded_ancestor.to_path_buf();
        let mut next_parent_id = deepest_loaded_ancestor_id;
        let last_component = missing_part
            .components()
            .last()
            // An error here indicates a bug in the implementation logic.
            .context("Missing part should have at least one component")?;

        // We intentionally use a `Option` to track the input data
        // and guarantee that the input data is consumed only once.
        let mut config = Some(config);
        let mut name = Some(name);

        for component in missing_part.components() {
            current_level.push(component.as_os_str());

            // Check if this entry already exists
            if self.snapshot.is_loaded(&current_level) {
                // Entry already exists, just get its ID and continue
                let existing_entry = self
                    .snapshot
                    .entry_by_path(&current_level)
                    .expect("Entry should exist since is_loaded returned true");
                next_parent_id = existing_entry.id;
                continue;
            }

            let (current_config, current_name) = if component == last_component {
                // Should never happen. The input config should be consumed by the last component only.
                // A panic here indicates a bug in the implementation logic.
                debug_assert!(config.is_some());
                debug_assert!(name.is_some());
                (
                    config.take().context("The input config was consumed")?,
                    name.take().context("The input name was consumed")?,
                )
            } else {
                let id = Uuid::new_v4();
                let name = component.as_os_str().to_string_lossy().to_string();
                (
                    ConfigurationModel::Dir(DirConfigurationModel {
                        metadata: SpecificationMetadata { id },
                    }),
                    name,
                )
            };
            let id: Uuid = current_config.id();
            let is_dir = matches!(current_config, ConfigurationModel::Dir(_));
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
                name: current_name,
                path: entry_path.clone(),
                is_dir,
                config: Some(config_handle),
            };

            self.snapshot.create_entry(entry, Some(next_parent_id))?;
            changes.push(WorktreeChange::Created {
                id,
                path: entry_path.clone(),
            });

            next_parent_id = id;
        }

        Ok(WorktreeDiff::from(changes))
    }

    /// Moves an entry to a new parent directory.
    ///
    /// This method moves an entry from its current location to a new parent directory.
    /// The file system operation is performed atomically, and the entry's configuration
    /// is updated to reflect the new path.
    pub async fn move_entry(
        &mut self,
        entry_id: Uuid,
        new_parent_id: Uuid,
    ) -> WorktreeResult<WorktreeDiff> {
        if entry_id == Uuid::nil() {
            return Err(WorktreeError::InvalidInput(
                "Root entry cannot be moved".to_string(),
            ));
        }

        let entry = self
            .snapshot
            .entry_by_id(entry_id)
            .ok_or_else(|| WorktreeError::NotFound(entry_id.to_string()))?;

        let from_path = entry.path.clone();

        // Get parent ID using the new method
        let from_parent_id = self
            .snapshot
            .entry_parent_id(entry_id)
            .unwrap_or(Uuid::nil());

        let new_parent = self
            .snapshot
            .entry_by_id(new_parent_id)
            .ok_or_else(|| WorktreeError::NotFound(new_parent_id.to_string()))?;

        let to_path = new_parent.path.join(entry.path.file_name().unwrap());
        let to_abs_path = self.abs_path.join(&to_path);
        if to_abs_path.exists() {
            return Err(WorktreeError::AlreadyExists(
                to_abs_path.to_string_lossy().to_string(),
            ));
        }
        let from_abs_path = self.abs_path.join(&from_path);

        let mut changes = vec![];

        // We need to load children of the parent since on the frontend we will expand the destination
        // folder and a user needs to see the children of the destination folder.
        changes.extend(self.load_children(new_parent.path.clone().as_ref()).await?);

        self.fs
            .rename(&from_abs_path, &to_abs_path, RenameOptions::default())
            .await?;
        self.snapshot
            .entry_by_id_mut_unchecked(entry_id)
            .config_mut()
            .reset_path(&to_abs_path);

        self.snapshot.move_entry(entry_id, new_parent_id)?;

        changes.push(WorktreeChange::Moved {
            id: entry_id,
            from_id: from_parent_id,
            to_id: new_parent_id,
            old_path: from_path,
            new_path: to_path.into(),
        });

        // TODO: collect children of the moved entry

        Ok(WorktreeDiff::from(changes))
    }

    /// Removes an entry from the worktree.
    ///
    /// This method removes an entry and all its children from both the file system
    /// and the worktree. The operation is performed recursively for directories.
    pub async fn remove_entry(&mut self, entry_id: Uuid) -> WorktreeResult<WorktreeDiff> {
        let entry = match self.snapshot.entry_by_id(entry_id) {
            Some(entry) => entry,
            None => {
                return Ok(vec![].into());
            }
        };

        let entry_path = entry.path.clone();
        let mut all_nodes_to_remove = self.snapshot.collect_loaded_descendants(entry_id);
        all_nodes_to_remove.push(entry_id); // Add the target entry itself

        // Collect all entry data before removal to avoid index shifting issues
        let mut entries_to_remove = Vec::new();
        for node_id in &all_nodes_to_remove {
            if let Some(entry) = self.snapshot.entry_by_id(*node_id) {
                if let Some(index) = self.snapshot.entry_node_index(*node_id) {
                    let path = entry.path.clone();
                    entries_to_remove.push((*node_id, path, index));
                }
            }
        }

        // Sort by index in descending order to avoid index shifting issues
        entries_to_remove.sort_by(|a, b| b.2.cmp(&a.2));

        let mut changes = Vec::new();

        // Move the target entry to a temporary directory for deletion
        let target_entry_abs_path = self.abs_path.join(&entry_path);
        dbg!(&target_entry_abs_path);
        let temp_dir = self.abs_path.join(format!(".{}.deleted", entry_id));
        let target_entry_name = entry.name.clone();

        if let Err(e) = self.fs.create_dir(&temp_dir).await {
            eprintln!("Failed to create temp directory: {}", e);
            return Err(WorktreeError::Unknown(e.into()));
        }

        if let Err(e) = self
            .fs
            .rename(
                &target_entry_abs_path,
                &temp_dir.join(&target_entry_name),
                RenameOptions::default(),
            )
            .await
        {
            eprintln!("Failed to move entry to temp directory: {}", e);
            return Err(WorktreeError::Unknown(e.into()));
        }

        // Schedule background task to delete the temporary directory
        let temp_dir_clone = temp_dir.clone();
        let fs_clone = self.fs.clone();
        let task = tokio::spawn(async move {
            if let Err(e) = fs_clone
                .remove_dir(
                    &temp_dir_clone,
                    moss_fs::RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
            {
                eprintln!("Failed to delete temporary directory: {}", e);
            }
        });
        self.background_tasks.push(task);

        // Remove entries from snapshot in reverse order of their indices
        for (entry_id, path, _index) in entries_to_remove {
            if let Some(_removed_entry) = self.snapshot.remove_entry(entry_id) {
                changes.push(WorktreeChange::Deleted {
                    id: entry_id,
                    path: path.into(),
                });
            }
        }

        // Remove all unloaded entries that are children of the removed entry
        self.snapshot.remove_unloaded_by_prefix(&entry_path);

        Ok(WorktreeDiff::from(changes))
    }

    pub fn spawn_background_task<F>(&mut self, future: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let handle = tokio::spawn(future);
        self.background_tasks.push(handle);
    }

    pub async fn wait_for_background_tasks(&mut self) -> Result<(), tokio::task::JoinError> {
        let tasks = std::mem::take(&mut self.background_tasks);

        for task in tasks {
            task.await?;
        }

        Ok(())
    }

    pub fn background_tasks_count(&self) -> usize {
        self.background_tasks.len()
    }

    pub async fn load_many<'a>(
        &mut self,
        paths: impl IntoIterator<Item = PathBuf>,
    ) -> WorktreeResult<WorktreeDiff> {
        let mut paths_to_load = paths
            .into_iter()
            .map(|p| p.to_path_buf())
            .collect::<Vec<_>>();

        paths_to_load.sort_by_key(|p| p.components().count());

        let mut changes = vec![];
        for path in find_maximal_paths(paths_to_load) {
            changes.extend(self.load_entry_internal(&path, 1).await?);
        }

        Ok(WorktreeDiff::from(changes))
    }

    /// Walks through the worktree starting from the specified entry ID,
    /// sending resolved entries with their parent IDs through the provided channel.
    /// The walk is performed in depth-first order and does not include the starting entry.
    pub async fn walk<T, F>(
        &self,
        from: Uuid,
        resolver: F,
        sender: mpsc::UnboundedSender<(ParentId, T)>,
    ) -> WorktreeResult<()>
    where
        F: Fn(&Entry) -> T,
        T: Send,
    {
        self.snapshot
            .entry_by_id(from)
            .ok_or_else(|| WorktreeError::NotFound(from.to_string()))?;

        // Use a stack that stores (node_id, parent_id) pairs
        let children = self.snapshot.entry_children(from);
        let mut stack: Vec<(Uuid, ParentId)> = children
            .into_iter()
            .map(|child_id| (child_id, from)) // child_id with parent from
            .collect();

        stack.sort_by_key(|(child_id, _)| {
            self.snapshot
                .entry_by_id(*child_id)
                .map(|entry| entry.path.clone())
                .unwrap_or_else(|| Path::new("").into())
        });
        stack.reverse(); // Reverse for correct depth-first order

        while let Some((current_id, parent_id)) = stack.pop() {
            let current_entry = self
                .snapshot
                .entry_by_id(current_id)
                .ok_or_else(|| WorktreeError::NotFound(current_id.to_string()))?;

            let resolved_entry = resolver(current_entry);
            if sender.send((parent_id, resolved_entry)).is_err() {
                return Ok(()); // Receiver dropped, stop walking
            }

            let mut children = self.snapshot.entry_children(current_id);
            children.sort_by_key(|&child_id| {
                self.snapshot
                    .entry_by_id(child_id)
                    .map(|entry| entry.path.clone())
                    .unwrap_or_else(|| Path::new("").into())
            });

            // Add children to stack with current_id as their parent
            for child_id in children.into_iter().rev() {
                stack.push((child_id, current_id));
            }
        }

        Ok(())
    }
}

pub fn find_maximal_paths(paths: impl IntoIterator<Item = PathBuf>) -> Vec<PathBuf> {
    let paths: BTreeSet<PathBuf> = paths.into_iter().collect();
    let mut result = Vec::new();

    for path in &paths {
        let mut is_prefix = false;

        // Root path should always be included if present
        if path.as_os_str().is_empty() {
            result.push(path.clone());
            continue;
        }

        for other in paths.range(path.clone()..) {
            if other == path {
                continue;
            }

            if let Ok(rest) = other.strip_prefix(path) {
                if rest.components().next().is_some() {
                    is_prefix = true;
                    break;
                }
            } else {
                break;
            }
        }
        if !is_prefix {
            result.push(path.clone());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::configuration::{
        DirConfigurationModel, ItemConfigurationModel, SpecificationMetadata,
    };
    use moss_fs::RealFileSystem;
    use std::{env, path::PathBuf};
    use uuid::Uuid;

    #[test]
    fn test_find_maximal_paths_pathbuf() {
        let input = vec![
            PathBuf::from(""),
            PathBuf::from("a"),
            PathBuf::from("a/b"),
            PathBuf::from("a/b/c"),
            PathBuf::from("a/b/d"),
            PathBuf::from("a/b/d/l"),
            PathBuf::from("e"),
            PathBuf::from("e/c"),
            PathBuf::from("f"),
            PathBuf::from("f/b"),
            PathBuf::from("f/b/q"),
        ];
        let result = find_maximal_paths(input);
        let expected = vec![
            PathBuf::from(""),
            PathBuf::from("a/b/c"),
            PathBuf::from("a/b/d/l"),
            PathBuf::from("e/c"),
            PathBuf::from("f/b/q"),
        ];
        assert_eq!(result, expected);
    }

    async fn create_test_worktree() -> (Worktree, PathBuf) {
        let temp_dir = env::temp_dir().join(format!("moss_test_{}", Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir).unwrap();

        let fs = Arc::new(RealFileSystem::new());
        let worktree = Worktree::new(fs, temp_dir.clone().into()).await.unwrap();

        (worktree, temp_dir)
    }

    fn cleanup_test_dir(temp_dir: &PathBuf) {
        std::fs::remove_dir_all(temp_dir).ok();
    }

    #[tokio::test]
    async fn test_expand_entry() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        // Create some structure first for testing restore
        let config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree.create_entry(Path::new("a"), config).await.unwrap();

        let config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("a/b"), config)
            .await
            .unwrap();

        let _changes = worktree
            .load_many(vec![
                PathBuf::from(""),
                PathBuf::from("a"),
                PathBuf::from("a/b"),
            ])
            .await
            .unwrap();

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_new_worktree_creation() {
        let (worktree, temp_dir) = create_test_worktree().await;

        // Worktree should be created successfully and root should be loaded
        assert_eq!(worktree.background_tasks_count(), 0);
        assert!(worktree.snapshot.is_loaded(Path::new("")));

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_create_single_item() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        let config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });

        let changes = worktree
            .create_entry(Path::new("test_item"), config)
            .await
            .unwrap();

        // Should create one entry (root is already loaded, so no Loaded change for root)
        assert_eq!(changes.len(), 1);

        // Check that we have one Created change (item)
        let created_changes: Vec<_> = changes
            .iter()
            .filter(|change| matches!(change, WorktreeChange::Created { .. }))
            .collect();

        assert_eq!(created_changes.len(), 1); // item created

        // Entry should be loaded
        assert!(worktree.snapshot.is_loaded(Path::new("test_item")));

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_create_nested_structure() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        let config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });

        // Create deeply nested item - should auto-create parent directories
        let changes = worktree
            .create_entry(Path::new("level1/level2/level3/item"), config)
            .await
            .unwrap();

        // Should create 4 entries (root is already loaded, so no Loaded change for root)
        assert_eq!(changes.len(), 4);

        // Check that we have four Created changes
        let created_changes: Vec<_> = changes
            .iter()
            .filter(|change| matches!(change, WorktreeChange::Created { .. }))
            .collect();

        assert_eq!(created_changes.len(), 4); // 3 directories + 1 item created

        // All levels should be loaded
        assert!(worktree.snapshot.is_loaded(Path::new("level1")));
        assert!(worktree.snapshot.is_loaded(Path::new("level1/level2")));
        assert!(
            worktree
                .snapshot
                .is_loaded(Path::new("level1/level2/level3"))
        );
        assert!(
            worktree
                .snapshot
                .is_loaded(Path::new("level1/level2/level3/item"))
        );

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_create_duplicate_entry_fails() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        let config1 = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });

        let config2 = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });

        // Create first entry
        worktree
            .create_entry(Path::new("duplicate"), config1)
            .await
            .unwrap();

        // Creating second entry with same path should fail
        let result = worktree.create_entry(Path::new("duplicate"), config2).await;
        assert!(matches!(result, Err(WorktreeError::AlreadyExists(_))));

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_load_entry_basic() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        // Create some structure first
        let config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("parent"), config)
            .await
            .unwrap();

        let config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("parent/child"), config)
            .await
            .unwrap();

        // Create new worktree to test loading from scratch
        let fs = Arc::new(RealFileSystem::new());
        let mut fresh_worktree = Worktree::new(fs, temp_dir.clone().into()).await.unwrap();

        // Load parent with depth 0 (no children)
        let changes = fresh_worktree
            .load_entry(Path::new("parent"), 0)
            .await
            .unwrap();

        // Should load root and parent
        assert!(changes.len() >= 1);
        assert!(fresh_worktree.snapshot.is_loaded(Path::new("parent")));
        assert!(!fresh_worktree.snapshot.is_loaded(Path::new("parent/child")));

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_load_entry_with_depth() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        // Create structure
        let config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("parent"), config)
            .await
            .unwrap();

        let config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("parent/child1"), config)
            .await
            .unwrap();

        let config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("parent/child2"), config)
            .await
            .unwrap();

        // Create fresh worktree
        let fs = Arc::new(RealFileSystem::new());
        let mut fresh_worktree = Worktree::new(fs, temp_dir.clone().into()).await.unwrap();

        // Load parent with depth 1 (include children)
        let changes = fresh_worktree
            .load_entry(Path::new("parent"), 1)
            .await
            .unwrap();

        // Should load parent and its children
        assert!(fresh_worktree.snapshot.is_loaded(Path::new("parent")));
        assert!(
            fresh_worktree
                .snapshot
                .is_loaded(Path::new("parent/child1"))
        );
        assert!(
            fresh_worktree
                .snapshot
                .is_loaded(Path::new("parent/child2"))
        );

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_load_nonexistent_entry_fails() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        let result = worktree.load_entry(Path::new("nonexistent"), 0).await;
        assert!(matches!(result, Err(WorktreeError::NotFound(_))));

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_move_entry_basic() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        // Create source and target directories
        let source_config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("source"), source_config)
            .await
            .unwrap();

        let target_id = Uuid::new_v4();
        let target_config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: target_id },
        });
        worktree
            .create_entry(Path::new("target"), target_config)
            .await
            .unwrap();

        // Create item to move
        let item_id = Uuid::new_v4();
        let item_config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: item_id },
        });
        worktree
            .create_entry(Path::new("source/item"), item_config)
            .await
            .unwrap();

        // Move item from source to target
        let changes = worktree.move_entry(item_id, target_id).await.unwrap();

        // Verify move
        assert!(!worktree.snapshot.is_loaded(Path::new("source/item")));
        assert!(worktree.snapshot.is_loaded(Path::new("target/item")));

        // Check that move change is recorded
        let move_changes: Vec<_> = changes
            .iter()
            .filter(|change| matches!(change, WorktreeChange::Moved { .. }))
            .collect();
        assert_eq!(move_changes.len(), 1);

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_move_nonexistent_entry_fails() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        let target_id = Uuid::new_v4();
        let target_config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: target_id },
        });
        worktree
            .create_entry(Path::new("target"), target_config)
            .await
            .unwrap();

        let result = worktree.move_entry(Uuid::new_v4(), target_id).await;
        assert!(matches!(result, Err(WorktreeError::NotFound(_))));

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_move_root_entry_fails() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        let target_id = Uuid::new_v4();
        let target_config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: target_id },
        });
        worktree
            .create_entry(Path::new("target"), target_config)
            .await
            .unwrap();

        let result = worktree.move_entry(Uuid::nil(), target_id).await;
        assert!(matches!(result, Err(WorktreeError::InvalidInput(_))));

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_move_entry_should_load_children_of_destination() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        let item_id = Uuid::new_v4();
        let config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: item_id },
        });
        worktree
            .create_entry(Path::new("item"), config)
            .await
            .unwrap();

        let dest_id = Uuid::new_v4();
        let config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: dest_id },
        });
        worktree
            .create_entry(Path::new("dest"), config)
            .await
            .unwrap();

        let another_id = Uuid::new_v4();
        let config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: another_id },
        });
        worktree
            .create_entry(Path::new("dest/another"), config)
            .await
            .unwrap();

        drop(worktree);

        // Load a fresh worktree
        let fs = Arc::new(RealFileSystem::new());
        let mut worktree = Worktree::new(fs, temp_dir.as_path().into()).await.unwrap();

        // Load the children of root entry
        worktree.load_children(Path::new(ROOT_PATH)).await.unwrap();

        // Move `entry` to `dest\entry`
        let changes = worktree.move_entry(item_id, dest_id).await.unwrap();

        // Verify move
        assert!(!worktree.snapshot.is_loaded(Path::new("item")));
        assert!(worktree.snapshot.is_loaded(Path::new("dest/item")));

        // Verify that the children of `dest` are also loaded
        assert!(worktree.snapshot.is_loaded(Path::new("dest/another")));

        // Check that te move and load changes are recorded
        assert_eq!(changes.len(), 2);
        assert!(
            changes
                .iter()
                .any(|change| matches!(change, WorktreeChange::Moved { .. }))
        );
        assert!(
            changes
                .iter()
                .any(|change| matches!(change, WorktreeChange::Loaded { .. }))
        );

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_move_entry_already_exists() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        let item_id = Uuid::new_v4();
        let config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: item_id },
        });
        worktree
            .create_entry(Path::new("item"), config)
            .await
            .unwrap();

        let dest_id = Uuid::new_v4();
        let config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: dest_id },
        });
        worktree
            .create_entry(Path::new("dest"), config)
            .await
            .unwrap();

        let existing_id = Uuid::new_v4();
        let config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: existing_id },
        });
        worktree
            .create_entry(Path::new("dest/item"), config)
            .await
            .unwrap();

        let move_entry_result = worktree.move_entry(item_id, dest_id).await;
        assert!(matches!(
            move_entry_result,
            Err(WorktreeError::AlreadyExists { .. })
        ));

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_remove_entry_basic() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        let item_id = Uuid::new_v4();
        let config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: item_id },
        });
        worktree
            .create_entry(Path::new("to_remove"), config)
            .await
            .unwrap();

        // Verify entry exists
        assert!(worktree.snapshot.is_loaded(Path::new("to_remove")));

        // Remove entry
        let changes = worktree.remove_entry(item_id).await.unwrap();

        // Verify removal
        assert!(!worktree.snapshot.is_loaded(Path::new("to_remove")));

        // Check that delete change is recorded
        let delete_changes: Vec<_> = changes
            .iter()
            .filter(|change| matches!(change, WorktreeChange::Deleted { .. }))
            .collect();
        assert_eq!(delete_changes.len(), 1);

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_remove_entry_with_children() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        // Create parent directory
        let parent_id = Uuid::new_v4();
        let parent_config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: parent_id },
        });
        worktree
            .create_entry(Path::new("parent"), parent_config)
            .await
            .unwrap();

        // Create children
        let child1_config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("parent/child1"), child1_config)
            .await
            .unwrap();

        let child2_config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("parent/child2"), child2_config)
            .await
            .unwrap();

        // Debug: Print state before removal
        println!("Before removal:");
        println!(
            "  parent loaded: {}",
            worktree.snapshot.is_loaded(Path::new("parent"))
        );
        println!(
            "  child1 loaded: {}",
            worktree.snapshot.is_loaded(Path::new("parent/child1"))
        );
        println!(
            "  child2 loaded: {}",
            worktree.snapshot.is_loaded(Path::new("parent/child2"))
        );

        // Debug: Check parent-child relationships
        if let Some(parent_entry) = worktree.snapshot.entry_by_path(Path::new("parent")) {
            println!("  parent ID: {}", parent_entry.id);
        }
        if let Some(child1_entry) = worktree.snapshot.entry_by_path(Path::new("parent/child1")) {
            println!("  child1 ID: {}", child1_entry.id);
            if let Some(parent_id) = worktree.snapshot.entry_parent_id(child1_entry.id) {
                println!("  child1 parent ID: {}", parent_id);
            }
        }
        if let Some(child2_entry) = worktree.snapshot.entry_by_path(Path::new("parent/child2")) {
            println!("  child2 ID: {}", child2_entry.id);
            if let Some(parent_id) = worktree.snapshot.entry_parent_id(child2_entry.id) {
                println!("  child2 parent ID: {}", parent_id);
            }
        }

        // Remove parent (should remove children too)
        let changes = worktree.remove_entry(parent_id).await.unwrap();

        // Debug: Print state after removal
        println!("After removal:");
        println!(
            "  parent loaded: {}",
            worktree.snapshot.is_loaded(Path::new("parent"))
        );
        println!(
            "  child1 loaded: {}",
            worktree.snapshot.is_loaded(Path::new("parent/child1"))
        );
        println!(
            "  child2 loaded: {}",
            worktree.snapshot.is_loaded(Path::new("parent/child2"))
        );
        println!("  Changes count: {}", changes.len());
        for (i, change) in changes.iter().enumerate() {
            println!("  Change {}: {:?}", i, change);
        }

        // Verify all are removed
        assert!(!worktree.snapshot.is_loaded(Path::new("parent")));
        assert!(!worktree.snapshot.is_loaded(Path::new("parent/child1")));
        assert!(!worktree.snapshot.is_loaded(Path::new("parent/child2")));

        // Should have 3 delete changes (parent + 2 children)
        let delete_changes: Vec<_> = changes
            .iter()
            .filter(|change| matches!(change, WorktreeChange::Deleted { .. }))
            .collect();
        assert_eq!(delete_changes.len(), 3);

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_remove_nonexistent_entry() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        // Removing non-existent entry should return empty changes (not fail)
        let changes = worktree.remove_entry(Uuid::new_v4()).await.unwrap();
        assert_eq!(changes.len(), 0);

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_background_tasks_management() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        // Initially no background tasks
        assert_eq!(worktree.background_tasks_count(), 0);

        // Spawn some background tasks
        worktree.spawn_background_task(async {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        });

        worktree.spawn_background_task(async {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        });

        // Should have 2 background tasks
        assert_eq!(worktree.background_tasks_count(), 2);

        // Wait for all tasks to complete
        worktree.wait_for_background_tasks().await.unwrap();

        // Should have no background tasks after waiting
        assert_eq!(worktree.background_tasks_count(), 0);

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_absolutize_path() {
        let abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let rel_path = Path::new("src").join("module");

        let result = Worktree::absolutize(&abs_path, &rel_path).unwrap();

        assert_eq!(
            result,
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src")
                .join("module")
        );
    }

    #[tokio::test]
    async fn test_absolutize_path_with_parent_dir_fails() {
        let abs_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let rel_path = Path::new("../outside");

        let result = Worktree::absolutize(&abs_path, rel_path);
        assert!(matches!(result, Err(WorktreeError::InvalidInput(_))));
    }

    #[tokio::test]
    async fn test_complex_workflow() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        // Create initial structure
        let src_config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("src"), src_config)
            .await
            .unwrap();

        let module_id = Uuid::new_v4();
        let module_config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: module_id },
        });
        worktree
            .create_entry(Path::new("src/module"), module_config)
            .await
            .unwrap();

        // Create target directory
        let target_id = Uuid::new_v4();
        let target_config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: target_id },
        });
        worktree
            .create_entry(Path::new("lib"), target_config)
            .await
            .unwrap();

        // Move module from src to lib
        worktree.move_entry(module_id, target_id).await.unwrap();

        // Verify move
        assert!(!worktree.snapshot.is_loaded(Path::new("src/module")));
        assert!(worktree.snapshot.is_loaded(Path::new("lib/module")));

        // Create new worktree and test loading
        let fs = Arc::new(RealFileSystem::new());
        let mut fresh_worktree = Worktree::new(fs, temp_dir.clone().into()).await.unwrap();

        // Load lib with children
        let _changes = fresh_worktree
            .load_entry(Path::new("lib"), 1)
            .await
            .unwrap();

        // Should load lib and module
        assert!(fresh_worktree.snapshot.is_loaded(Path::new("lib")));
        assert!(fresh_worktree.snapshot.is_loaded(Path::new("lib/module")));

        // Remove entire lib directory
        fresh_worktree.remove_entry(target_id).await.unwrap();

        // Wait for cleanup
        fresh_worktree.wait_for_background_tasks().await.unwrap();

        // Verify removal
        assert!(!fresh_worktree.snapshot.is_loaded(Path::new("lib")));
        assert!(!fresh_worktree.snapshot.is_loaded(Path::new("lib/module")));

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_walk_function() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        // Create a tree structure for testing
        let parent_config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("parent"), parent_config)
            .await
            .unwrap();

        let child1_config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("parent/child1"), child1_config)
            .await
            .unwrap();

        let child2_config = ConfigurationModel::Dir(DirConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("parent/child2"), child2_config)
            .await
            .unwrap();

        let grandchild_config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("parent/child2/grandchild"), grandchild_config)
            .await
            .unwrap();

        // Get the parent entry ID
        let parent_entry = worktree
            .snapshot
            .entry_by_path(Path::new("parent"))
            .unwrap();
        let parent_id = parent_entry.id;

        // Create channel for receiving walked entries
        let (sender, mut receiver) = mpsc::unbounded_channel();

        // Walk from parent with a resolver that extracts the path
        worktree
            .walk(
                parent_id,
                |entry| entry.path.to_string_lossy().to_string(),
                sender,
            )
            .await
            .unwrap();

        // Collect all received entries
        let mut received_data: Vec<(snapshot::ParentId, String)> = Vec::new();
        while let Ok((parent_id, path)) = receiver.try_recv() {
            // Make the test work on Windows by normalizing the path separator
            received_data.push((parent_id, path.replace('\\', "/")));
        }

        let received_paths: Vec<String> =
            received_data.iter().map(|(_, path)| path.clone()).collect();

        // Should receive all descendants of parent (but not parent itself)
        dbg!(&received_paths);
        assert_eq!(received_paths.len(), 3);
        assert!(received_paths.contains(&"parent/child1".to_string()));
        assert!(received_paths.contains(&"parent/child2".to_string()));
        assert!(received_paths.contains(&"parent/child2/grandchild".to_string()));

        // Check that parent IDs are correct
        let parent_entry = worktree
            .snapshot
            .entry_by_path(Path::new("parent"))
            .unwrap();
        let child2_entry = worktree
            .snapshot
            .entry_by_path(Path::new("parent/child2"))
            .unwrap();

        // Find entries with correct parent relationships
        let child1_data = received_data
            .iter()
            .find(|(_, path)| path == "parent/child1")
            .unwrap();
        let child2_data = received_data
            .iter()
            .find(|(_, path)| path == "parent/child2")
            .unwrap();
        let grandchild_data = received_data
            .iter()
            .find(|(_, path)| path == "parent/child2/grandchild")
            .unwrap();

        assert_eq!(child1_data.0, parent_entry.id); // child1's parent should be parent
        assert_eq!(child2_data.0, parent_entry.id); // child2's parent should be parent  
        assert_eq!(grandchild_data.0, child2_entry.id); // grandchild's parent should be child2

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_walk_from_root() {
        let (mut worktree, temp_dir) = create_test_worktree().await;

        // Create some entries
        let item_config = ConfigurationModel::Item(ItemConfigurationModel {
            metadata: SpecificationMetadata { id: Uuid::new_v4() },
        });
        worktree
            .create_entry(Path::new("item"), item_config)
            .await
            .unwrap();

        // Create channel for receiving walked entries
        let (sender, mut receiver) = mpsc::unbounded_channel();

        // Walk from root with a resolver that extracts the path
        worktree
            .walk(
                ROOT_ID,
                |entry| entry.path.to_string_lossy().to_string(),
                sender,
            )
            .await
            .unwrap();

        // Collect all received entries
        let mut received_data: Vec<(snapshot::ParentId, String)> = Vec::new();
        while let Ok((parent_id, path)) = receiver.try_recv() {
            received_data.push((parent_id, path));
        }

        let received_paths: Vec<String> =
            received_data.iter().map(|(_, path)| path.clone()).collect();

        // Should receive all descendants of root (but not root itself)
        assert!(received_paths.len() >= 1); // At least item
        assert!(received_paths.contains(&"item".to_string()));

        // Check that parent ID is correct (should be ROOT_ID)
        let item_data = received_data
            .iter()
            .find(|(_, path)| path == "item")
            .unwrap();
        assert_eq!(item_data.0, ROOT_ID);

        cleanup_test_dir(&temp_dir);
    }

    #[tokio::test]
    async fn test_walk_nonexistent_entry() {
        let (worktree, temp_dir) = create_test_worktree().await;

        let (sender, _receiver) = mpsc::unbounded_channel();

        // Try to walk from non-existent entry
        let result = worktree
            .walk(
                Uuid::new_v4(),
                |entry| entry.path.to_string_lossy().to_string(),
                sender,
            )
            .await;
        assert!(matches!(result, Err(WorktreeError::NotFound(_))));

        cleanup_test_dir(&temp_dir);
    }
}
