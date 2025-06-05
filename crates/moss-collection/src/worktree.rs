// pub mod physical_snapshot;
// pub mod physical_worktree;
pub mod snapshot;
pub mod util;
// pub mod virtual_snapshot;
// pub mod virtual_worktree;

use anyhow::Context;
use moss_common::api::OperationError;
use moss_file::toml::EditableInPlaceFileHandle;
use moss_fs::FileSystem;
use moss_text::sanitized;

use serde_json::Value as JsonValue;
use snapshot::{
    ConfigurationModel, DirConfigurationModel, Entry, Snapshot, SpecificationMetadata,
    UnloadedEntry,
};
use std::{
    collections::VecDeque,
    path::{Component, Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use thiserror::Error;
use tokio::sync::mpsc;
use util::names::dir_name_from_classification;
use uuid::Uuid;

use crate::models::{
    primitives::{ChangesDiffSet, EntryId},
    types::{Classification, RequestProtocol},
};

pub(crate) const CONFIG_FILE_NAME_ITEM: &str = "config.toml";
pub(crate) const CONFIG_FILE_NAME_DIR: &str = "config-folder.toml";

pub(crate) const ROOT_PATH: &str = "";

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

// pub struct WorktreeDiff {
//     pub physical_changes: ChangesDiffSet,
//     pub virtual_changes: ChangesDiffSet,
// }

// impl Default for WorktreeDiff {
//     fn default() -> Self {
//         Self {
//             physical_changes: Arc::new([]),
//             virtual_changes: Arc::new([]),
//         }
//     }
// }

struct ScanJob {
    abs_path: Arc<Path>,
    path: Arc<Path>,
    parent_unloaded_id: Option<usize>,
    scan_queue: mpsc::UnboundedSender<ScanJob>,
}

pub struct Worktree {
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    snapshot: Snapshot,
}

impl Worktree {
    pub async fn new(fs: Arc<dyn FileSystem>, abs_path: Arc<Path>) -> Result<Self, WorktreeError> {
        debug_assert!(abs_path.is_absolute());

        let next_unloaded_id = Arc::new(AtomicUsize::new(0));
        let mut unloaded_entries = vec![(
            UnloadedEntry::Dir {
                id: next_unloaded_id.fetch_add(1, Ordering::Relaxed),
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

        let snapshot = Snapshot::new(unloaded_entries);

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
    ) -> WorktreeResult<Vec<(UnloadedEntry, Option<usize>)>> {
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
                    let child_abs_path: Arc<Path> = child_entry.path().into();
                    let child_name = child_abs_path.file_name().unwrap();
                    let child_path: Arc<Path> = job.path.join(child_name).into();

                    let unloaded_child_entry: UnloadedEntry;
                    if child_abs_path.join(CONFIG_FILE_NAME_DIR).exists() {
                        unloaded_child_entry = UnloadedEntry::Dir {
                            id: next_unloaded_id.fetch_add(1, Ordering::Relaxed),
                            path: Arc::clone(&child_path),
                            abs_path: Arc::clone(&child_abs_path),
                        };
                    } else if child_abs_path.join(CONFIG_FILE_NAME_ITEM).exists() {
                        unloaded_child_entry = UnloadedEntry::Item {
                            id: next_unloaded_id.fetch_add(1, Ordering::Relaxed),
                            path: Arc::clone(&child_path),
                            abs_path: Arc::clone(&child_abs_path),
                        };
                    } else {
                        println!("{} is not a valid directory", child_abs_path.display());
                        continue;
                    }

                    new_entries.push((unloaded_child_entry.clone(), job.parent_unloaded_id));

                    let child_file_type = child_entry.file_type().await.unwrap();
                    if child_file_type.is_dir() {
                        new_jobs.push(ScanJob {
                            abs_path: Arc::clone(&child_abs_path),
                            path: child_path,
                            parent_unloaded_id: Some(unloaded_child_entry.id()),
                            scan_queue: job.scan_queue.clone(),
                        });
                    } else {
                        // TODO
                    }
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

    pub async fn load_entry(&mut self, path: &Path, depth: u8) -> WorktreeResult<()> {
        let mut ancestors_to_load = Vec::new();

        // Collect all ancestors that need to be loaded
        for ancestor in path.ancestors() {
            if !self.snapshot.is_loaded(ancestor) {
                ancestors_to_load.push(ancestor.to_path_buf());
            } else {
                // Stop if we found a loaded ancestor
                break;
            }
        }

        // Load ancestors from root to leaves (in reverse order)
        ancestors_to_load.reverse();
        for ancestor_path in ancestors_to_load {
            if !self.snapshot.is_loaded(&ancestor_path) {
                self.load_single_entry(&ancestor_path).await?;
            }
        }

        if !self.snapshot.is_loaded(path) {
            self.load_single_entry(path).await?;
        }

        // If depth > 0, load children on the specified depth
        if depth > 0 {
            let mut current_level = vec![path.to_path_buf()];

            for _ in 0..depth {
                let mut next_level = Vec::new();

                for current_path in current_level {
                    // Get unloaded children of this path
                    let unloaded_children = self
                        .snapshot
                        .unloaded_children_by_parent_path(&current_path);

                    for child in unloaded_children {
                        let child_path = child.path().to_path_buf();
                        if !self.snapshot.is_loaded(&child_path) {
                            self.load_single_entry(&child_path).await?;
                        }
                        next_level.push(child_path);
                    }
                }

                current_level = next_level;
                if current_level.is_empty() {
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn load_single_entry(&mut self, path: &Path) -> WorktreeResult<()> {
        let unloaded_entry = self
            .snapshot
            .unloaded_entry_by_path(path)
            .ok_or(WorktreeError::NotFound(path.display().to_string()))?
            .clone();

        let entry = if unloaded_entry.is_root() {
            Entry {
                id: Uuid::nil(),
                path: unloaded_entry.path().to_owned(),
                config: None,
            }
        } else {
            let config_path = match &unloaded_entry {
                UnloadedEntry::Item { abs_path, .. } => abs_path.join(CONFIG_FILE_NAME_ITEM),
                UnloadedEntry::Dir { abs_path, .. } => abs_path.join(CONFIG_FILE_NAME_DIR),
            };

            let config = EditableInPlaceFileHandle::<ConfigurationModel>::load(
                self.fs.clone(),
                &config_path,
            )
            .await?;
            Entry {
                id: config.model().await.id(),
                path: unloaded_entry.path().to_owned(),
                config: Some(config),
            }
        };

        self.snapshot.load_entry(unloaded_entry.id(), entry)?;

        Ok(())
    }

    pub async fn create_entry(
        &mut self,
        path: &Path,
        config: ConfigurationModel,
    ) -> Result<(), WorktreeError> {
        assert!(path.is_relative());

        if path.file_name().is_none() {
            return Err(WorktreeError::InvalidInput(format!(
                "Target name is not a valid: {}",
                path.display()
            )));
        }

        let encoded_path = moss_fs::utils::sanitize_path(path, None)?;

        let lowest_loaded_ancestor_ref = self.snapshot.lowest_loaded_ancestor_path(&encoded_path);
        let lowest_unloaded_ancestor_ref = self.snapshot.lowest_ancestor_path(&encoded_path);

        if let Some(p) = lowest_loaded_ancestor_ref {
            //
        } else {
            // Root was not expanded yet
            self.load_entry(Path::new(ROOT_PATH), 1).await?;
        };

        let lowest_loaded_ancestor_ref = self
            .snapshot
            .lowest_loaded_ancestor_path(&encoded_path)
            .expect("Root expanded to be loaded");

        {
            if self.snapshot.unloaded_entries_count() > 1 {
                let missing_part_to_load = lowest_unloaded_ancestor_ref
                    .path
                    .strip_prefix(&lowest_loaded_ancestor_ref.path)
                    .unwrap();

                let mut current_level = lowest_loaded_ancestor_ref.path.to_path_buf();

                for component in missing_part_to_load.components() {
                    current_level.push(component.as_os_str());
                    self.load_entry(&current_level, 0).await?;
                }
            }
        }

        let missing_part = encoded_path
            .strip_prefix(&lowest_loaded_ancestor_ref.path)
            .expect("Lowest ancestor path must be a prefix of path");

        let mut current_level = lowest_loaded_ancestor_ref.path.to_path_buf();
        let mut next_parent_id = lowest_loaded_ancestor_ref.id;
        let last_component = missing_part
            .components()
            .last()
            .context("Missing part should have at least one component")?;

        // We intentionally use a `Option` to track the input config
        // and guarantee that the input config is consumed only once.
        let mut config = Some(config);

        for component in missing_part.components() {
            current_level.push(component.as_os_str());

            let current_config = if component == last_component {
                // This should never happen. The input config should be consumed by the last component only.
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

            let entry = Entry {
                id,
                path: current_level.clone().into(),
                config: Some(config_handle),
            };

            self.snapshot.create_entry(entry, Some(next_parent_id))?;

            next_parent_id = id;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use moss_fs::RealFileSystem;

    use crate::worktree::snapshot::ItemConfigurationModel;

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

        worktree
            .load_entry(Path::new("foo"), u8::MAX)
            .await
            .unwrap();

        println!("{}", worktree.snapshot);
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
}

// impl Worktree {
//     pub fn new(
//         fs: Arc<dyn FileSystem>,
//         abs_path: Arc<Path>,
//         next_entry_id: Arc<AtomicUsize>, // TODO: replace with IdRegistry
//     ) -> Self {
//         debug_assert!(abs_path.is_absolute());

//         let next_virtual_entry_id = Arc::new(AtomicUsize::new(0)); // TODO: replace with IdRegistry
//         Self {
//             pwt: PhysicalWorktree::new(fs, abs_path, next_entry_id),
//             vwt: VirtualWorktree::new(next_virtual_entry_id),
//         }
//     }

//     pub fn is_empty(&self) -> bool {
//         self.vwt.is_empty()
//     }

//     pub async fn create_entry(
//         &mut self,
//         destination: PathBuf,
//         order: Option<usize>,
//         protocol: Option<RequestProtocol>,
//         specification: Option<Vec<u8>>,
//         classification: Classification,
//         is_dir: bool,
//     ) -> WorktreeResult<WorktreeDiff> {
//         // Check if an entry with the same virtual path already exists
//         if self.vwt.entry_by_path(&destination).is_some() {
//             return WorktreeResult::Err(WorktreeError::AlreadyExists(
//                 destination.to_string_lossy().to_string(),
//             ));
//         }

//         let (parent, name) = split_last_segment(&destination)
//             .ok_or_else(|| {
//                 WorktreeError::InvalidInput(format!(
//                     "Invalid destination path: {}",
//                     destination.display()
//                 ))
//             })
//             .map(|(parent, name)| (parent.unwrap_or_default(), name))?;

//         if is_dir {
//             self.create_dir(parent, name, order, classification, specification)
//                 .await
//         } else {
//             self.create_item(parent, name, classification, specification, order, protocol)
//                 .await
//         }
//     }

//     async fn create_dir(
//         &mut self,
//         parent: PathBuf,
//         name: String,
//         order: Option<usize>,
//         classification: Classification,
//         specification: Option<Vec<u8>>,
//     ) -> WorktreeResult<WorktreeDiff> {
//         let mut physical_changes = vec![];
//         let mut virtual_changes = vec![];

//         {
//             let encoded_path = {
//                 let encoded_name = sanitized::sanitize(&name);
//                 let encoded_path = moss_fs::utils::sanitize_path(&parent, None)?;

//                 encoded_path.join(encoded_name)
//             };
//             physical_changes.extend(
//                 self.pwt
//                     .create_entry(&encoded_path, true, None)
//                     .await?
//                     .into_iter()
//                     .cloned(),
//             );

//             let specfile_path = encoded_path.join("folder.sapic");
//             physical_changes.extend(
//                 self.pwt
//                     .create_entry(&specfile_path, false, specification)
//                     .await?
//                     .into_iter()
//                     .cloned(),
//             );
//         }

//         {
//             virtual_changes.extend(
//                 self.vwt
//                     .create_entry(parent.join(name), order, classification.clone(), None, true)?
//                     .into_iter()
//                     .cloned(),
//             );
//         }

//         Ok(WorktreeDiff {
//             physical_changes: ChangesDiffSet::from(physical_changes),
//             virtual_changes: ChangesDiffSet::from(virtual_changes),
//         })
//     }

//     async fn create_item(
//         &mut self,
//         parent: PathBuf,
//         name: String,
//         classification: Classification,
//         specification: Option<Vec<u8>>,
//         order: Option<usize>,
//         protocol: Option<RequestProtocol>,
//     ) -> WorktreeResult<WorktreeDiff> {
//         let mut physical_changes = vec![];
//         let mut virtual_changes = vec![];

//         let encoded_path = {
//             let encoded_name = sanitized::sanitize(&name);
//             let encoded_path = moss_fs::utils::sanitize_path(&parent, None)?;

//             encoded_path.join(dir_name_from_classification(&encoded_name, &classification))
//         };
//         physical_changes.extend(
//             self.pwt
//                 .create_entry(&encoded_path, true, None)
//                 .await?
//                 .into_iter()
//                 .cloned(),
//         );

//         // TODO: Handling protocol for non-request entities?
//         let protocol = protocol.unwrap_or_default();
//         let file_name = protocol.to_filename();
//         let file_path = encoded_path.join(file_name);
//         physical_changes.extend(
//             self.pwt
//                 .create_entry(&file_path, false, specification)
//                 .await?
//                 .into_iter()
//                 .cloned(),
//         );

//         virtual_changes.extend(
//             self.vwt
//                 .create_entry(
//                     &parent,
//                     None,
//                     classification.clone(),
//                     Some(protocol.clone()),
//                     true,
//                 )?
//                 .into_iter()
//                 .cloned(),
//         );
//         virtual_changes.extend(
//             self.vwt
//                 .create_entry(
//                     parent.join(name),
//                     order,
//                     classification,
//                     Some(protocol),
//                     false,
//                 )?
//                 .into_iter()
//                 .cloned(),
//         );

//         Ok(WorktreeDiff {
//             physical_changes: ChangesDiffSet::from(physical_changes),
//             virtual_changes: ChangesDiffSet::from(virtual_changes),
//         })
//     }

//     pub async fn delete_entry_by_virtual_id(
//         &mut self,
//         id: EntryId,
//     ) -> WorktreeResult<WorktreeDiff> {
//         // Find the physical and virtual path from the virtual ID
//         let virtual_entry = self
//             .vwt
//             .entry_by_id(id)
//             .ok_or(WorktreeError::NotFound(format!(
//                 "Virtual ID {} is not found",
//                 id.to_usize()
//             )))?
//             .clone();

//         let physical_path = virtual_entry.physical_path()?;
//         let virtual_path = virtual_entry.path();

//         let physical_changes = self.pwt.remove_entry(&physical_path).await?;
//         let virtual_changes = self.vwt.remove_entry(virtual_path)?;

//         Ok(WorktreeDiff {
//             physical_changes,
//             virtual_changes,
//         })
//     }

//     pub async fn update_entry_by_virtual_id(
//         &mut self,
//         id: EntryId,
//         name: Option<String>,
//         classification: Option<Classification>,
//         specification: Option<JsonValue>,
//         protocol: Option<RequestProtocol>,
//         order: Option<usize>,
//     ) -> WorktreeResult<WorktreeDiff> {
//         if let Some(new_name) = name {
//             self.rename_entry_by_virtual_id(id, &new_name).await
//         } else {
//             Ok(WorktreeDiff::default())
//         }
//         // TODO: Handle updating of other fields
//     }

//     async fn rename_entry_by_virtual_id(
//         &mut self,
//         id: EntryId,
//         new_name: &str,
//     ) -> WorktreeResult<WorktreeDiff> {
//         // Find the physical and virtual path from the virtual ID
//         let virtual_entry = Arc::unwrap_or_clone(
//             self.vwt
//                 .entry_by_id(id)
//                 .ok_or(WorktreeError::NotFound(format!(
//                     "Virtual ID {} is not found",
//                     id.to_usize()
//                 )))?
//                 .clone(),
//         );

//         let old_physical_path = virtual_entry.physical_path()?;
//         let old_virtual_path = virtual_entry.path();
//         let new_virtual_path = old_virtual_path
//             .parent()
//             .expect("Virtual path should have a parent")
//             .join(&new_name);
//         if old_virtual_path.to_path_buf() == new_virtual_path {
//             return Ok(WorktreeDiff::default());
//         }
//         if self.vwt.entry_by_path(&new_virtual_path).is_some() {
//             return Err(WorktreeError::AlreadyExists(
//                 new_virtual_path.to_string_lossy().to_string(),
//             ));
//         }
//         let virtual_changes = self
//             .vwt
//             .rename_entry(&old_virtual_path, &new_virtual_path)?;

//         let parent = old_physical_path
//             .parent()
//             .expect("Physical path should have a parent");

//         let new_filename = match virtual_entry {
//             VirtualEntry::Item { class, .. } => {
//                 dir_name_from_classification(&sanitized::sanitize(&new_name), &class)
//             }
//             VirtualEntry::Dir { .. } => sanitized::sanitize(&new_name),
//         };
//         let new_physical_path = parent.join(&new_filename);
//         let physical_changes = self
//             .pwt
//             .rename_entry(&old_physical_path, &new_physical_path)
//             .await?;

//         Ok(WorktreeDiff {
//             physical_changes,
//             virtual_changes,
//         })
//     }

//     pub fn iter_entries_by_prefix<'a>(
//         &'a self,
//         prefix: PathBuf,
//     ) -> impl Iterator<Item = (&'a EntryId, &'a Arc<VirtualEntry>)> + 'a {
//         self.vwt.iter_entries_by_prefix(prefix)
//     }
// }

// /// Splits the given path into its parent directory (if any) and the last segment.
// ///
// /// Returns:
// /// - `None` if the input path is empty or contains no segments.
// /// - `Some((parent, segment))` where:
// ///     - `parent` is:
// ///         - `Some(PathBuf)` if the path has a non-empty parent,
// ///         - `None` if the path consists of a single segment.
// ///     - `segment` is the last path component as a `String`.
// fn split_last_segment(path: &Path) -> Option<(Option<PathBuf>, String)> {
//     if path.as_os_str().is_empty() {
//         return None;
//     }

//     // Collect normalized components (ignores redundant separators)
//     let mut comps: Vec<Component> = path.components().collect();
//     if comps.is_empty() {
//         return None;
//     }

//     let last_comp = comps.pop().unwrap();

//     // Determine the string for the last segment
//     let last_os = match last_comp {
//         Component::Normal(os) => os,
//         Component::RootDir => std::ffi::OsStr::new("/"),
//         Component::Prefix(pref) => pref.as_os_str(),
//         _ => return None, // ignore CurDir, ParentDir, etc.
//     };
//     let last = last_os.to_string_lossy().into_owned();

//     // Build the parent PathBuf if there are remaining components
//     let parent = if comps.is_empty() {
//         None
//     } else {
//         let mut parent_pb = PathBuf::new();
//         for comp in comps {
//             parent_pb.push(comp.as_os_str());
//         }
//         Some(parent_pb)
//     };

//     Some((parent, last))
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::path::{Path, PathBuf};

//     #[test]
//     fn test_split_last_segment_with_parent() {
//         // Splitting a normal absolute path returns the parent directory and last segment
//         let path = Path::new("/test/foo/bar");
//         let result = split_last_segment(path);
//         assert_eq!(
//             result,
//             Some((Some(PathBuf::from("/test/foo")), "bar".to_string()))
//         );
//     }

//     #[test]
//     fn test_split_last_segment_with_trailing_slash() {
//         // A path ending with a slash should still return the correct parent and segment
//         let path = Path::new("/test/foo/bar/");
//         let result = split_last_segment(path);
//         assert_eq!(
//             result,
//             Some((Some(PathBuf::from("/test/foo")), "bar".to_string()))
//         );
//     }

//     #[test]
//     fn test_split_last_segment_single_segment() {
//         // A single-segment relative path returns None for parent and the segment itself
//         let path = Path::new("bar");
//         let result = split_last_segment(path);
//         assert_eq!(result, Some((None, "bar".to_string())));
//     }

//     #[test]
//     fn test_split_last_segment_empty_path() {
//         // An empty path should return None
//         let path = Path::new("");
//         let result = split_last_segment(path);
//         assert_eq!(result, None);
//     }
// }
