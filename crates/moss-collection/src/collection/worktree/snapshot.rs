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

    pub fn create_entry(&mut self, entry: EntryRef) {
        self.entries_by_path.insert(entry.path.clone(), entry.id);
        self.entries_by_id.insert(entry.id, entry);
    }

    pub fn count_files(&self) -> usize {
        debug_assert_eq!(self.entries_by_path.len(), self.entries_by_id.len());

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

    pub fn entry_by_path(&self, path: impl AsRef<Path>) -> Option<EntryRef> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        let entry_id = self.entries_by_path.get(path)?;
        self.entries_by_id.get(entry_id).cloned()
    }

    pub fn remove_entry(&mut self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        let is_dir = if let Some(entry) = self.entry_by_path(path) {
            matches!(entry.kind, EntryKind::Dir)
        } else {
            return;
        };

        if is_dir {
            let prefix = path.to_string_lossy();
            let entries_to_remove = self
                .iter_entries_by_prefix(&prefix)
                .map(|(&id, entry)| (Arc::clone(&entry.path), id))
                .collect::<Vec<(Arc<Path>, EntryId)>>();

            for (entry_path, entry_id) in entries_to_remove {
                self.entries_by_path.remove(&entry_path);
                self.entries_by_id.remove(&entry_id);
            }
        } else {
            if let Some(entry_id) = self.entries_by_path.remove(path) {
                self.entries_by_id.remove(&entry_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::PathBuf, sync::atomic::AtomicUsize};

    fn create_test_entry(id: usize, path: &str, kind: EntryKind) -> Entry {
        Entry {
            id: EntryId::new(&Arc::new(AtomicUsize::new(id))),
            path: Arc::from(PathBuf::from(path)),
            kind,
            unit_type: None,
            mtime: None,
            file_id: FileId::new_inode(0, 0),
        }
    }

    #[test]
    fn test_remove_file() {
        let mut snapshot = Snapshot::new(Arc::from(PathBuf::from("/root")));
        let entry = create_test_entry(1, "test.txt", EntryKind::File);
        let entry_ref = Arc::new(entry.clone());

        snapshot.create_entry(entry_ref.clone());
        assert_eq!(snapshot.count_files(), 1);

        snapshot.remove_entry("test.txt");
        assert_eq!(snapshot.count_files(), 0);
        assert!(snapshot.entry_by_path("test.txt").is_none());
    }

    #[test]
    fn test_remove_nonexistent_file() {
        let mut snapshot = Snapshot::new(Arc::from(PathBuf::from("/root")));
        snapshot.remove_entry("nonexistent.txt");
        assert_eq!(snapshot.count_files(), 0);
    }

    #[test]
    fn test_remove_directory_with_files() {
        let mut snapshot = Snapshot::new(Arc::from(PathBuf::from("/root")));

        // Create a directory and some files inside it
        let dir_entry = create_test_entry(1, "test_dir", EntryKind::Dir);
        let file1_entry = create_test_entry(2, "test_dir/file1.txt", EntryKind::File);
        let file2_entry = create_test_entry(3, "test_dir/file2.txt", EntryKind::File);

        snapshot.create_entry(Arc::new(dir_entry));
        snapshot.create_entry(Arc::new(file1_entry));
        snapshot.create_entry(Arc::new(file2_entry));

        assert_eq!(snapshot.count_files(), 3);

        // Remove the directory
        snapshot.remove_entry("test_dir");

        // Verify all entries are removed
        assert_eq!(snapshot.count_files(), 0);
        assert!(snapshot.entry_by_path("test_dir").is_none());
        assert!(snapshot.entry_by_path("test_dir/file1.txt").is_none());
        assert!(snapshot.entry_by_path("test_dir/file2.txt").is_none());
    }

    #[test]
    fn test_remove_nested_directory() {
        let mut snapshot = Snapshot::new(Arc::from(PathBuf::from("/root")));

        // Create a nested directory structure
        let dir1_entry = create_test_entry(1, "dir1", EntryKind::Dir);
        let dir2_entry = create_test_entry(2, "dir1/dir2", EntryKind::Dir);
        let file_entry = create_test_entry(3, "dir1/dir2/file.txt", EntryKind::File);

        snapshot.create_entry(Arc::new(dir1_entry));
        snapshot.create_entry(Arc::new(dir2_entry));
        snapshot.create_entry(Arc::new(file_entry));

        assert_eq!(snapshot.count_files(), 3);

        // Remove the parent directory
        snapshot.remove_entry("dir1");

        // Verify all entries are removed
        assert_eq!(snapshot.count_files(), 0);
        assert!(snapshot.entry_by_path("dir1").is_none());
        assert!(snapshot.entry_by_path("dir1/dir2").is_none());
        assert!(snapshot.entry_by_path("dir1/dir2/file.txt").is_none());
    }

    #[test]
    fn test_remove_partial_path() {
        let mut snapshot = Snapshot::new(Arc::from(PathBuf::from("/root")));

        // Create entries with similar prefixes
        let dir1_entry = create_test_entry(1, "test", EntryKind::Dir);
        let dir2_entry = create_test_entry(2, "test_dir", EntryKind::Dir);
        let file_entry = create_test_entry(3, "test_file.txt", EntryKind::File);

        snapshot.create_entry(Arc::new(dir1_entry));
        snapshot.create_entry(Arc::new(dir2_entry));
        snapshot.create_entry(Arc::new(file_entry));

        assert_eq!(snapshot.count_files(), 3);

        // Remove only the "test" directory
        snapshot.remove_entry("test");

        // Verify only the "test" directory is removed
        assert_eq!(snapshot.count_files(), 2);
        assert!(snapshot.entry_by_path("test").is_none());
        assert!(snapshot.entry_by_path("test_dir").is_some());
        assert!(snapshot.entry_by_path("test_file.txt").is_some());
    }
}
