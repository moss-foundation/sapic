use anyhow::anyhow;
use moss_common::models::primitives::Identifier;
use moss_common::sanitized::{sanitize, sanitized_name::SanitizedName};
use moss_fs::utils::sanitize_path;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use sweep_bptree::BPlusTreeMap;

use crate::models::primitives::{ChangesDiffSet, EntryId};
use crate::models::types::{Classification, PathChangeKind, RequestProtocol};

use super::{ROOT_PATH, WorktreeError, WorktreeResult, split_last_segment};

pub struct VirtualEntryId(Identifier);

#[derive(Clone)]
pub struct Case {
    id: EntryId,
    name: SanitizedName,
    order: Option<usize>,
}

#[derive(Clone)]
pub enum VirtualEntry {
    Item {
        // FIXME: Should we store info like request protocol here?
        id: EntryId,
        order: Option<usize>,
        class: Classification,
        protocol: Option<RequestProtocol>,
        path: Arc<Path>,
        cases: Vec<Case>,
    },
    Dir {
        id: EntryId,
        order: Option<usize>,
        class: Classification,
        path: Arc<Path>,
    },
}

impl VirtualEntry {
    pub fn id(&self) -> EntryId {
        match self {
            VirtualEntry::Item { id, .. } => *id,
            VirtualEntry::Dir { id, .. } => *id,
        }
    }

    pub fn path(&self) -> &Arc<Path> {
        match self {
            VirtualEntry::Item { path, .. } => path,
            VirtualEntry::Dir { path, .. } => path,
        }
    }

    pub fn classification(&self) -> Classification {
        match self {
            VirtualEntry::Item { class, .. } => class.clone(),
            VirtualEntry::Dir { class, .. } => class.clone(),
        }
    }

    pub fn order(&self) -> Option<usize> {
        match self {
            VirtualEntry::Item { order, .. } => *order,
            VirtualEntry::Dir { order, .. } => *order,
        }
    }

    pub fn protocol(&self) -> Option<RequestProtocol> {
        match self {
            VirtualEntry::Item { protocol, .. } => protocol.clone(),
            VirtualEntry::Dir { .. } => None,
        }
    }

    pub fn physical_path(&self) -> anyhow::Result<PathBuf> {
        match self {
            VirtualEntry::Item { path, class, .. } => {
                if let Some((parent, name)) = split_last_segment(path) {
                    let encoded_parent = sanitize_path(&parent.unwrap_or_default(), None)?;
                    let encoded_name = format!("{}.{}", sanitize(&name), class.as_str());
                    Ok(encoded_parent.join(encoded_name))
                } else {
                    // TODO: replace with proper error
                    Err(anyhow!("Invalid virtual path"))
                }
            }
            VirtualEntry::Dir { path, .. } => Ok(sanitize_path(path, None)?),
        }
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, VirtualEntry::Dir { .. })
    }
}

pub struct VirtualSnapshot {
    entries_by_id: BPlusTreeMap<EntryId, Arc<VirtualEntry>>,
    entries_by_path: BPlusTreeMap<Arc<Path>, EntryId>,
}

impl VirtualSnapshot {
    pub fn new() -> Self {
        Self {
            entries_by_id: BPlusTreeMap::new(),
            entries_by_path: BPlusTreeMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.entries_by_id.is_empty()
    }
    pub fn exists(&self, path: impl AsRef<Path>) -> bool {
        self.entry_by_path(path).is_some()
    }

    pub fn create_entry(&mut self, entry: Arc<VirtualEntry>) {
        let path = entry.path().clone();
        let id = entry.id();

        self.entries_by_path.insert(path, id);
        self.entries_by_id.insert(id, entry);
    }

    pub fn entry_by_id(&self, id: EntryId) -> Option<&Arc<VirtualEntry>> {
        self.entries_by_id.get(&id)
    }

    pub fn entry_by_path(&self, path: impl AsRef<Path>) -> Option<Arc<VirtualEntry>> {
        let path = path.as_ref();

        let entry_id = self.entries_by_path.get(path)?;
        self.entries_by_id.get(entry_id).cloned()
    }

    // TODO: Try to generalize the delete methods in both the Virtual and Physical snapshots.
    pub fn iter_entries_by_prefix<'a>(
        &'a self,
        prefix: PathBuf,
    ) -> impl Iterator<Item = (&'a EntryId, &'a Arc<VirtualEntry>)> + 'a {
        let prefix = prefix.to_path_buf();
        self.entries_by_path
            .iter()
            .filter(move |(path, _id)| path.starts_with(&prefix))
            .filter_map(move |(_, id)| self.entries_by_id.get(id).map(|entry| (id, entry)))
    }

    // TODO: Try to generalize the delete methods in both the Virtual and Physical snapshots.
    pub fn remove_entry(&mut self, path: impl AsRef<Path>) -> Vec<Arc<VirtualEntry>> {
        let path = path.as_ref();
        let entry_opt = self.entry_by_path(path);
        let is_dir = if let Some(entry) = &entry_opt {
            matches!(entry.as_ref(), VirtualEntry::Dir { .. })
        } else {
            return Vec::new();
        };

        let mut removed_entries = Vec::new();

        if is_dir {
            let entries_to_remove = self
                .iter_entries_by_prefix(path.to_path_buf())
                .map(|(id, entry)| (*id, entry.clone()))
                .collect::<Vec<(EntryId, Arc<VirtualEntry>)>>();

            for (entry_id, entry) in entries_to_remove {
                let entry_path = match entry.as_ref() {
                    VirtualEntry::Item { path, .. } => path.clone(),
                    VirtualEntry::Dir { path, .. } => path.clone(),
                };

                self.entries_by_path.remove(&entry_path);
                self.entries_by_id.remove(&entry_id);
                removed_entries.push(entry);
            }
        } else if let Some(entry) = entry_opt {
            let entry_path = match entry.as_ref() {
                VirtualEntry::Item { path, .. } => path.clone(),
                VirtualEntry::Dir { path, .. } => path.clone(),
            };

            self.entries_by_path.remove(&entry_path);
            self.entries_by_id.remove(&entry.id());
            removed_entries.push(entry);
        }

        removed_entries
    }

    /// Rename the entry and update the paths of all entries that descends from it
    pub fn rename_entry(
        &mut self,
        old_path: impl AsRef<Path>,
        new_path: impl AsRef<Path>,
    ) -> WorktreeResult<ChangesDiffSet> {
        let mut changes = vec![];
        let old_path = old_path.as_ref();
        let new_path = new_path.as_ref();

        if self.entries_by_path.get(new_path).is_some() {
            return Err(WorktreeError::AlreadyExists(
                new_path.to_string_lossy().to_string(),
            ));
        }
        if self.entries_by_path.get(old_path).is_none() {
            return Err(WorktreeError::NotFound(
                old_path.to_string_lossy().to_string(),
            ));
        }

        // Find the entry id of all entries that need updating
        let ids = self
            .iter_entries_by_prefix(old_path.to_path_buf())
            .map(|(id, _)| id.clone())
            .collect::<Vec<_>>();

        for id in ids {
            let entry = Arc::unwrap_or_clone(self.entries_by_id.remove(&id).unwrap());
            let old_entry_path = entry.path().clone();
            self.entries_by_path.remove(&old_entry_path).unwrap();

            // Strip the old prefix and attach the new prefix
            let new_entry_path = if old_entry_path.to_path_buf() == old_path {
                // Prevent appending trailing slashes
                new_path.to_path_buf()
            } else {
                new_path.join(
                    old_entry_path
                        .strip_prefix(old_path)
                        .expect("Old entry path has the given prefix"),
                )
            };

            let new_entry = match entry {
                VirtualEntry::Item {
                    id,
                    order,
                    class,
                    cases,
                    protocol,
                    ..
                } => VirtualEntry::Item {
                    path: Arc::from(new_entry_path.as_path()),
                    id,
                    order,
                    class,
                    cases,
                    protocol,
                },
                VirtualEntry::Dir {
                    id, order, class, ..
                } => VirtualEntry::Dir {
                    path: Arc::from(new_entry_path.as_path()),
                    id,
                    order,
                    class,
                },
            };
            self.entries_by_path
                .insert(Arc::from(new_entry_path.clone()), id);
            self.entries_by_id.insert(id, Arc::new(new_entry));
            changes.push((Arc::from(new_entry_path), id, PathChangeKind::Updated));
        }
        Ok(ChangesDiffSet::from(changes))
    }

    pub fn lowest_ancestor_path(&self, path: impl AsRef<Path>) -> Arc<Path> {
        let input_path = path.as_ref();

        for ancestor in input_path.ancestors() {
            if let Some(entry_ref) = self.entry_by_path(ancestor) {
                return entry_ref.path().clone();
            }
        }

        Arc::from(PathBuf::from(ROOT_PATH))
    }
}
