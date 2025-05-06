use file_id::FileId;
use std::{path::Path, sync::Arc, time::SystemTime};
use sweep_bptree::BPlusTreeMap;

use crate::models::{primitives::EntryId, types::UnitType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EntryKind {
    Unit, // Do we need this?
    PendingDir,
    UnloadedDir,
    Dir,
    File,
}

#[derive(Debug, Clone)]
pub(crate) struct Entry {
    pub id: EntryId,
    pub path: Arc<Path>,
    pub kind: EntryKind,
    pub unit_type: Option<UnitType>,
    pub mtime: Option<SystemTime>,
    pub file_id: FileId,
}

impl Entry {
    pub fn path(&self) -> &Arc<Path> {
        &self.path
    }
}

pub(super) type EntryRef = Arc<Entry>;

pub(crate) struct Snapshot {
    abs_path: Arc<Path>,
    entries_by_id: BPlusTreeMap<EntryId, EntryRef>,
    entries_by_path: BPlusTreeMap<Arc<Path>, EntryId>,
}

impl Clone for Snapshot {
    fn clone(&self) -> Self {
        let entries_by_id =
            BPlusTreeMap::from_iter(self.entries_by_id.iter().map(|(&k, v)| (k, v.clone())));

        let entries_by_path = BPlusTreeMap::from_iter(
            self.entries_by_path
                .iter()
                .map(|(&ref k, v)| (k.clone(), v.clone())),
        );

        Self {
            abs_path: self.abs_path.clone(),
            entries_by_id,
            entries_by_path,
        }
    }
}

impl Snapshot {
    pub fn new(abs_path: Arc<Path>) -> Self {
        Self {
            abs_path,
            entries_by_id: BPlusTreeMap::new(),
            entries_by_path: BPlusTreeMap::new(),
        }
    }

    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    pub fn insert(&mut self, entry: EntryRef) {
        self.entries_by_path.insert(entry.path.clone(), entry.id);
        self.entries_by_id.insert(entry.id, entry);
    }

    pub fn count_files(&self) -> usize {
        self.entries_by_path.len()
    }

    pub fn iter_entries_by_prefix<'a>(
        &'a self,
        prefix: &'a str,
    ) -> impl Iterator<Item = (&'a EntryId, &'a EntryRef)> + 'a {
        self.entries_by_path
            .iter()
            .skip_while(move |(p, _)| !p.starts_with(prefix))
            .take_while(move |(p, _)| p.starts_with(prefix))
            .filter_map(move |(_, id)| self.entries_by_id.get(id).map(|entry| (id, entry)))
    }
}
