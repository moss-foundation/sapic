pub mod physical_snapshot;
pub mod physical_worktree;
pub mod util;
pub mod virtual_snapshot;
pub mod virtual_worktree;

use moss_common::api::OperationError;
use moss_fs::FileSystem;
use moss_fs::utils::encode_name;
use physical_worktree::PhysicalWorktree;
use serde_json::Value as JsonValue;
use std::{
    path::{Component, Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use thiserror::Error;
use util::names::dir_name_from_classification;
use virtual_worktree::VirtualWorktree;

use crate::models::primitives::EntryId;
use crate::models::types::RequestProtocol;
use crate::models::{primitives::ChangesDiffSet, types::Classification};
use crate::worktree::virtual_snapshot::VirtualEntry;

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

pub struct WorktreeDiff {
    pub physical_changes: ChangesDiffSet,
    pub virtual_changes: ChangesDiffSet,
}

impl Default for WorktreeDiff {
    fn default() -> Self {
        Self {
            physical_changes: Arc::new([]),
            virtual_changes: Arc::new([]),
        }
    }
}

pub struct Worktree {
    pwt: PhysicalWorktree,
    vwt: VirtualWorktree,
}

impl Worktree {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        abs_path: Arc<Path>,
        next_entry_id: Arc<AtomicUsize>, // TODO: replace with IdRegistry
    ) -> Self {
        debug_assert!(abs_path.is_absolute());

        let next_virtual_entry_id = Arc::new(AtomicUsize::new(0)); // TODO: replace with IdRegistry
        Self {
            pwt: PhysicalWorktree::new(fs, abs_path, next_entry_id),
            vwt: VirtualWorktree::new(next_virtual_entry_id),
        }
    }

    pub async fn create_entry(
        &mut self,
        destination: PathBuf,
        order: Option<usize>,
        protocol: Option<RequestProtocol>,
        specification: Option<Vec<u8>>,
        classification: Classification,
        is_dir: bool,
    ) -> WorktreeResult<WorktreeDiff> {
        // Check if an entry with the same virtual path already exists
        if self.vwt.entry_by_path(&destination).is_some() {
            return WorktreeResult::Err(WorktreeError::AlreadyExists(
                destination.to_string_lossy().to_string(),
            ));
        }

        let (parent, name) = split_last_segment(&destination)
            .ok_or_else(|| {
                WorktreeError::InvalidInput(format!(
                    "Invalid destination path: {}",
                    destination.display()
                ))
            })
            .map(|(parent, name)| (parent.unwrap_or_default(), name))?;

        if is_dir {
            self.create_dir(parent, name, order, classification, specification)
                .await
        } else {
            self.create_item(parent, name, classification, specification, order, protocol)
                .await
        }
    }

    async fn create_dir(
        &mut self,
        parent: PathBuf,
        name: String,
        order: Option<usize>,
        classification: Classification,
        specification: Option<Vec<u8>>,
    ) -> WorktreeResult<WorktreeDiff> {
        let mut physical_changes = vec![];
        let mut virtual_changes = vec![];

        {
            let encoded_path = {
                let encoded_name = moss_fs::utils::encode_name(&name);
                let encoded_path = moss_fs::utils::encode_path(&parent, None)?;

                encoded_path.join(encoded_name)
            };
            physical_changes.extend(
                self.pwt
                    .create_entry(&encoded_path, true, None)
                    .await?
                    .into_iter()
                    .cloned(),
            );

            let specfile_path = encoded_path.join("folder.sapic");
            physical_changes.extend(
                self.pwt
                    .create_entry(&specfile_path, false, specification)
                    .await?
                    .into_iter()
                    .cloned(),
            );
        }

        {
            virtual_changes.extend(
                self.vwt
                    .create_entry(parent.join(name), order, classification.clone(), None, true)?
                    .into_iter()
                    .cloned(),
            );
        }

        Ok(WorktreeDiff {
            physical_changes: ChangesDiffSet::from(physical_changes),
            virtual_changes: ChangesDiffSet::from(virtual_changes),
        })
    }

    async fn create_item(
        &mut self,
        parent: PathBuf,
        name: String,
        classification: Classification,
        specification: Option<Vec<u8>>,
        order: Option<usize>,
        protocol: Option<RequestProtocol>,
    ) -> WorktreeResult<WorktreeDiff> {
        let mut physical_changes = vec![];
        let mut virtual_changes = vec![];

        let encoded_path = {
            let encoded_name = moss_fs::utils::encode_name(&name);
            let encoded_path = moss_fs::utils::encode_path(&parent, None)?;

            encoded_path.join(dir_name_from_classification(&encoded_name, &classification))
        };
        physical_changes.extend(
            self.pwt
                .create_entry(&encoded_path, true, None)
                .await?
                .into_iter()
                .cloned(),
        );

        // TODO: Handling protocol for non-request entities?
        let protocol = protocol.unwrap_or_default();
        let file_name = protocol.to_filename();
        let file_path = encoded_path.join(file_name);
        physical_changes.extend(
            self.pwt
                .create_entry(&file_path, false, specification)
                .await?
                .into_iter()
                .cloned(),
        );

        virtual_changes.extend(
            self.vwt
                .create_entry(
                    &parent,
                    None,
                    classification.clone(),
                    Some(protocol.clone()),
                    true,
                )?
                .into_iter()
                .cloned(),
        );
        virtual_changes.extend(
            self.vwt
                .create_entry(
                    parent.join(name),
                    order,
                    classification,
                    Some(protocol),
                    false,
                )?
                .into_iter()
                .cloned(),
        );

        Ok(WorktreeDiff {
            physical_changes: ChangesDiffSet::from(physical_changes),
            virtual_changes: ChangesDiffSet::from(virtual_changes),
        })
    }

    pub async fn delete_entry_by_virtual_id(
        &mut self,
        id: EntryId,
    ) -> WorktreeResult<WorktreeDiff> {
        // Find the physical and virtual path from the virtual ID
        let virtual_entry = self
            .vwt
            .entry_by_id(id)
            .ok_or(WorktreeError::NotFound(format!(
                "Virtual ID {} is not found",
                id.to_usize()
            )))?
            .clone();

        let physical_path = virtual_entry.physical_path()?;
        let virtual_path = virtual_entry.path();

        let physical_changes = self.pwt.remove_entry(&physical_path).await?;
        let virtual_changes = self.vwt.remove_entry(virtual_path)?;

        Ok(WorktreeDiff {
            physical_changes,
            virtual_changes,
        })
    }

    pub async fn update_entry_by_virtual_id(
        &mut self,
        id: EntryId,
        name: Option<String>,
        classification: Option<Classification>,
        specification: Option<JsonValue>,
        protocol: Option<RequestProtocol>,
        order: Option<usize>,
    ) -> WorktreeResult<WorktreeDiff> {
        if let Some(new_name) = name {
            self.rename_entry_by_virtual_id(id, &new_name).await
        } else {
            Ok(WorktreeDiff::default())
        }
        // TODO: Handle updating of other fields
    }

    async fn rename_entry_by_virtual_id(
        &mut self,
        id: EntryId,
        new_name: &str,
    ) -> WorktreeResult<WorktreeDiff> {
        // Find the physical and virtual path from the virtual ID
        let virtual_entry = Arc::unwrap_or_clone(
            self.vwt
                .entry_by_id(id)
                .ok_or(WorktreeError::NotFound(format!(
                    "Virtual ID {} is not found",
                    id.to_usize()
                )))?
                .clone(),
        );

        let old_physical_path = virtual_entry.physical_path()?;
        let old_virtual_path = virtual_entry.path();
        let new_virtual_path = old_virtual_path
            .parent()
            .expect("Virtual path should have a parent")
            .join(&new_name);
        if old_virtual_path.to_path_buf() == new_virtual_path {
            return Ok(WorktreeDiff::default());
        }
        if self.vwt.entry_by_path(&new_virtual_path).is_some() {
            return Err(WorktreeError::AlreadyExists(
                new_virtual_path.to_string_lossy().to_string(),
            ));
        }
        let virtual_changes = self
            .vwt
            .rename_entry(&old_virtual_path, &new_virtual_path)?;

        let parent = old_physical_path
            .parent()
            .expect("Physical path should have a parent");

        let new_filename = match virtual_entry {
            VirtualEntry::Item { class, .. } => {
                dir_name_from_classification(&encode_name(&new_name), &class)
            }
            VirtualEntry::Dir { .. } => encode_name(&new_name),
        };
        let new_physical_path = parent.join(&new_filename);
        let physical_changes = self
            .pwt
            .rename_entry(&old_physical_path, &new_physical_path)
            .await?;

        Ok(WorktreeDiff {
            physical_changes,
            virtual_changes,
        })
    }

    pub fn iter_entries_by_prefix<'a>(
        &'a self,
        prefix: PathBuf,
    ) -> impl Iterator<Item = (&'a EntryId, &'a Arc<VirtualEntry>)> + 'a {
        self.vwt.iter_entries_by_prefix(prefix)
    }
}

/// Splits the given path into its parent directory (if any) and the last segment.
///
/// Returns:
/// - `None` if the input path is empty or contains no segments.
/// - `Some((parent, segment))` where:
///     - `parent` is:
///         - `Some(PathBuf)` if the path has a non-empty parent,
///         - `None` if the path consists of a single segment.
///     - `segment` is the last path component as a `String`.
fn split_last_segment(path: &Path) -> Option<(Option<PathBuf>, String)> {
    if path.as_os_str().is_empty() {
        return None;
    }

    // Collect normalized components (ignores redundant separators)
    let mut comps: Vec<Component> = path.components().collect();
    if comps.is_empty() {
        return None;
    }

    let last_comp = comps.pop().unwrap();

    // Determine the string for the last segment
    let last_os = match last_comp {
        Component::Normal(os) => os,
        Component::RootDir => std::ffi::OsStr::new("/"),
        Component::Prefix(pref) => pref.as_os_str(),
        _ => return None, // ignore CurDir, ParentDir, etc.
    };
    let last = last_os.to_string_lossy().into_owned();

    // Build the parent PathBuf if there are remaining components
    let parent = if comps.is_empty() {
        None
    } else {
        let mut parent_pb = PathBuf::new();
        for comp in comps {
            parent_pb.push(comp.as_os_str());
        }
        Some(parent_pb)
    };

    Some((parent, last))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_split_last_segment_with_parent() {
        // Splitting a normal absolute path returns the parent directory and last segment
        let path = Path::new("/test/foo/bar");
        let result = split_last_segment(path);
        assert_eq!(
            result,
            Some((Some(PathBuf::from("/test/foo")), "bar".to_string()))
        );
    }

    #[test]
    fn test_split_last_segment_with_trailing_slash() {
        // A path ending with a slash should still return the correct parent and segment
        let path = Path::new("/test/foo/bar/");
        let result = split_last_segment(path);
        assert_eq!(
            result,
            Some((Some(PathBuf::from("/test/foo")), "bar".to_string()))
        );
    }

    #[test]
    fn test_split_last_segment_single_segment() {
        // A single-segment relative path returns None for parent and the segment itself
        let path = Path::new("bar");
        let result = split_last_segment(path);
        assert_eq!(result, Some((None, "bar".to_string())));
    }

    #[test]
    fn test_split_last_segment_empty_path() {
        // An empty path should return None
        let path = Path::new("");
        let result = split_last_segment(path);
        assert_eq!(result, None);
    }
}
