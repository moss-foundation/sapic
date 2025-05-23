use super::{
    WorktreeResult,
    virtual_snapshot::{VirtualEntry, VirtualSnapshot},
};
use crate::models::types::RequestProtocol;
use crate::models::{
    primitives::{ChangesDiffSet, EntryId},
    types::{Classification, PathChangeKind},
};
use std::path::PathBuf;
use std::{
    path::Path,
    sync::{Arc, atomic::AtomicUsize},
};

pub struct VirtualWorktree {
    next_entry_id: Arc<AtomicUsize>,
    snapshot: VirtualSnapshot,
}

impl VirtualWorktree {
    pub fn new(next_entry_id: Arc<AtomicUsize>) -> Self {
        Self {
            next_entry_id,
            snapshot: VirtualSnapshot::new(),
        }
    }

    pub fn entry_by_id(&self, id: EntryId) -> Option<&Arc<VirtualEntry>> {
        self.snapshot.entry_by_id(id)
    }

    pub fn entry_by_path(&self, path: &Path) -> Option<Arc<VirtualEntry>> {
        self.snapshot.entry_by_path(path)
    }

    pub fn is_empty(&self) -> bool {
        self.snapshot.is_empty()
    }

    pub fn create_entry(
        &mut self,
        destination: impl AsRef<Path>,
        order: Option<usize>,
        class: Classification,
        protocol: Option<RequestProtocol>,
        is_dir: bool,
    ) -> WorktreeResult<ChangesDiffSet> {
        if is_dir {
            if self.snapshot.exists(&destination) {
                return Ok(ChangesDiffSet::from(vec![]));
            }

            let lowest_ancestor_path = self.snapshot.lowest_ancestor_path(&destination);

            // If the ancestor is the same as the destination, we don't need to create any directories
            if lowest_ancestor_path.as_ref() == destination.as_ref() {
                return Ok(ChangesDiffSet::from(vec![]));
            }

            // Build intermediate paths from the lowest existing ancestor to the destination
            let relative_path = destination
                .as_ref()
                .strip_prefix(&lowest_ancestor_path)
                .unwrap_or(destination.as_ref());
            let mut current_path = lowest_ancestor_path.clone();
            let mut created_entries = Vec::new();

            // For each component in the relative path, create a directory entry
            for (index, component) in relative_path.components().enumerate() {
                let component_path = Path::new(component.as_os_str());
                current_path = Arc::from(current_path.join(component_path));

                if self.snapshot.exists(&current_path) {
                    continue;
                }

                let component_name = component_path
                    .file_name()
                    .map(|os_str| os_str.to_string_lossy().to_string())
                    .unwrap_or_default();
                debug_assert_ne!(component_name, "");

                // Determine if this is the last segment
                let is_last_segment = index == relative_path.components().count() - 1;

                // Only apply order to the last segment
                let segment_order = if is_last_segment { order } else { None };

                let dir_id = EntryId::new(&self.next_entry_id);
                let dir_entry = VirtualEntry::Dir {
                    id: dir_id,
                    order: segment_order,
                    class: class.clone(),
                    path: current_path.clone(),
                };

                self.snapshot.create_entry(Arc::new(dir_entry));
                created_entries.push((current_path.clone(), dir_id, PathChangeKind::Created));
            }

            Ok(ChangesDiffSet::from(created_entries))
        } else {
            let id = EntryId::new(&self.next_entry_id);
            let path: Arc<Path> = destination.as_ref().into();
            let entry = VirtualEntry::Item {
                id,
                order,
                class,
                path: path.clone(),
                cases: vec![],
                protocol,
            };

            self.snapshot.create_entry(Arc::new(entry));

            Ok(ChangesDiffSet::from(vec![(
                path,
                id,
                PathChangeKind::Created,
            )]))
        }
    }

    pub fn remove_entry(&mut self, path: impl AsRef<Path>) -> WorktreeResult<ChangesDiffSet> {
        let path = path.as_ref();

        let removed_entries = self.snapshot.remove_entry(path);

        let changes = removed_entries
            .into_iter()
            .map(|entry| (entry.path().clone(), entry.id(), PathChangeKind::Removed))
            .collect::<Vec<_>>();

        Ok(ChangesDiffSet::from(changes))
    }

    pub fn rename_entry(
        &mut self,
        old_path: impl AsRef<Path>,
        new_path: impl AsRef<Path>,
    ) -> WorktreeResult<ChangesDiffSet> {
        let old_path = old_path.as_ref();
        let new_path = new_path.as_ref();

        self.snapshot.rename_entry(old_path, new_path)
    }

    pub fn iter_entries_by_prefix<'a>(
        &'a self,
        prefix: PathBuf,
    ) -> impl Iterator<Item = (&'a EntryId, &'a Arc<VirtualEntry>)> + 'a {
        self.snapshot.iter_entries_by_prefix(prefix)
    }
}
