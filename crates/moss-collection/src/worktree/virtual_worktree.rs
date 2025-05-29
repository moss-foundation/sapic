use super::{
    WorktreeError, WorktreeResult,
    virtual_snapshot::{VirtualEntry, VirtualSnapshot},
};
use crate::models::{
    primitives::{ChangesDiffSet, EntryId},
    types::{Classification, PathChangeKind},
};
use moss_kdl::spec_models::entry_spec::WorktreeEntrySpecificationModel;
use std::path::PathBuf;
use std::{
    path::Path,
    sync::{Arc, atomic::AtomicUsize},
};
use uuid::Uuid;

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

    pub fn snapshot(&mut self) -> &VirtualSnapshot {
        &mut self.snapshot
    }

    pub fn entry_by_id(&self, id: Uuid) -> Option<&Arc<VirtualEntry>> {
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
        model: Arc<WorktreeEntrySpecificationModel>,
    ) -> WorktreeResult<ChangesDiffSet> {
        let path = Arc::from(destination.as_ref());
        if self.snapshot.exists(&path) {
            return Err(WorktreeError::AlreadyExists(
                path.to_string_lossy().to_string(),
            ));
        }
        let mut created_entries = Vec::new();
        let id = model.id();
        match model {
            WorktreeEntrySpecificationModel::Item(item) => {
                let id = item.id();
                let entry = VirtualEntry::Item {
                    id,
                    class,
                    path,
                    specification: ,
                };
            }
            WorktreeEntrySpecificationModel::Dir(dir) => {}
        }

        // if is_dir {
        //     let dir_id = EntryId::new(&self.next_entry_id);
        //     let dir_entry = VirtualEntry::Dir {
        //         id: dir_id,
        //         order,
        //         class,
        //         path: path.clone(),
        //     };
        //
        //     self.snapshot.create_entry(Arc::new(dir_entry));
        //     created_entries.push((path.clone(), dir_id, PathChangeKind::Created));
        //
        //     Ok(ChangesDiffSet::from(created_entries))
        // } else {
        //     let id = EntryId::new(&self.next_entry_id);
        //     let entry = VirtualEntry::Item {
        //         id,
        //         order,
        //         class,
        //         path: path.clone(),
        //         protocol,
        //     };

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
