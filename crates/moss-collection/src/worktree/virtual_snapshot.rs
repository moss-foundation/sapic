use std::path::{Path, PathBuf};
use std::sync::Arc;
use anyhow::anyhow;
use moss_common::models::primitives::Identifier;
use moss_common::sanitized::sanitized_name::SanitizedName;
use sweep_bptree::BPlusTreeMap;
use moss_fs::utils::{encode_name, encode_path};
use crate::models::primitives::EntryId;
use crate::models::types::Classification;

use super::{split_last_segment, ROOT_PATH};

pub struct VirtualEntryId(Identifier);

pub struct Case {
    id: EntryId,
    name: SanitizedName,
    order: Option<usize>,
}

pub enum VirtualEntry {
    Item {
        // FIXME: Should we store info like request protocol here?
        id: EntryId,
        order: Option<usize>,
        class: Classification,
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

    pub fn physical_path(&self) -> anyhow::Result<PathBuf> {
        match self {
            VirtualEntry::Item { path, class, .. } => {
                if let Some((parent, name)) = split_last_segment(path) {
                    let encoded_parent = encode_path(&parent.unwrap_or_default(), None)?;
                    let encoded_name = format!("{}.{}", encode_name(&name), class.as_str());
                    Ok(encoded_parent.join(encoded_name))
                } else {
                    // TODO: replace with proper error
                    Err(anyhow!("Invalid virtual path"))
                }
            },
            VirtualEntry::Dir {path, .. } => {
                Ok(encode_path(path, None)?)
            },
        }
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
        prefix: &'a str,
    ) -> impl Iterator<Item = (&'a EntryId, &'a Arc<VirtualEntry>)> + 'a {
        self.entries_by_path
            .iter()
            .skip_while(move |(p, _)| !p.to_string_lossy().starts_with(prefix))
            .take_while(move |(p, _)| p.to_string_lossy().starts_with(prefix))
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
            let prefix = path.to_string_lossy();
            let entries_to_remove = self
                .iter_entries_by_prefix(&prefix)
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
