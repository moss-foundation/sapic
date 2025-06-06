use file_id::FileId;
use moss_common::models::primitives::Identifier;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};
use sweep_bptree::BPlusTreeMap;

use crate::models::{primitives::EntryId, types::EntryKind};

use super::ROOT_PATH;

pub struct PhysicalEntryId(Identifier);

#[derive(Debug, Clone)]
pub struct PhysicalEntry {
    pub id: EntryId,
    pub path: Arc<Path>,
    pub kind: EntryKind,
    pub mtime: Option<SystemTime>,
    pub file_id: FileId,
}

impl PhysicalEntry {
    pub fn is_dir(&self) -> bool {
        matches!(self.kind, EntryKind::Dir)
    }
}

pub struct PhysicalSnapshot {
    abs_path: Arc<Path>,
    entries_by_id: BPlusTreeMap<EntryId, Arc<PhysicalEntry>>,
    entries_by_path: BPlusTreeMap<Arc<Path>, EntryId>,
}

impl Clone for PhysicalSnapshot {
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

impl PhysicalSnapshot {
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

    pub fn create_entry(&mut self, entry: Arc<PhysicalEntry>) {
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
    ) -> impl Iterator<Item = (&'a EntryId, &'a Arc<PhysicalEntry>)> + 'a {
        self.entries_by_path
            .iter()
            .skip_while(move |(p, _)| !p.starts_with(prefix))
            .take_while(move |(p, _)| p.starts_with(prefix))
            .filter_map(move |(_, id)| self.entries_by_id.get(id).map(|entry| (id, entry)))
    }

    pub fn entry_by_path(&self, path: impl AsRef<Path>) -> Option<Arc<PhysicalEntry>> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        let entry_id = self.entries_by_path.get(path)?;
        self.entries_by_id.get(entry_id).cloned()
    }

    pub fn entry_by_id(&self, id: EntryId) -> Option<&Arc<PhysicalEntry>> {
        self.entries_by_id.get(&id)
    }

    /// Removes an entry and all its children (if it's a directory) from the snapshot.
    ///
    /// This method removes the specified entry from the snapshot's internal collections.
    /// If the entry is a directory, it also recursively removes all entries whose paths
    /// have the directory's path as a prefix.
    ///
    /// # Returns
    ///
    /// A vector of removed entries (EntryRef objects).
    ///
    /// # Behavior
    ///
    /// - If the path doesn't exist in the snapshot, the method returns an empty list.
    /// - If the path points to a file, only that file entry is removed.
    /// - If the path points to a directory, the directory and all its contents (files and subdirectories)
    ///   are removed recursively.
    /// - Only exact path matches are considered; similar prefixes are not affected.
    pub fn remove_entry(&mut self, path: impl AsRef<Path>) -> Vec<Arc<PhysicalEntry>> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        let entry_opt = self.entry_by_path(path);
        let is_dir = if let Some(entry) = &entry_opt {
            matches!(entry.kind, EntryKind::Dir)
        } else {
            return Vec::new();
        };

        let mut removed_entries = Vec::new();

        if is_dir {
            let prefix = path.to_string_lossy();
            let entries_to_remove = self
                .iter_entries_by_prefix(&prefix)
                .map(|(id, entry)| (*id, entry.clone()))
                .collect::<Vec<(EntryId, Arc<PhysicalEntry>)>>();

            for (entry_id, entry) in entries_to_remove {
                self.entries_by_path.remove(&entry.path);
                self.entries_by_id.remove(&entry_id);
                removed_entries.push(entry);
            }
        } else if let Some(entry) = entry_opt {
            self.entries_by_path.remove(&entry.path);
            self.entries_by_id.remove(&entry.id);
            removed_entries.push(entry);
        }

        removed_entries
    }

    /// Finds the closest ancestor path of the given `path` that is known in the snapshot.
    ///
    /// This function iteratively checks the `path` and its parents until a known path is found
    /// in the snapshot's entries. For example, if the snapshot contains "a/b" and "a/b/c",
    /// and the input `path` is "a/b/c/d/e", this function will return "a/b/c".
    pub fn lowest_ancestor_path(&self, path: impl AsRef<Path>) -> Arc<Path> {
        let input_path = path.as_ref();

        for ancestor in input_path.ancestors() {
            if let Some(entry_ref) = self.entry_by_path(ancestor) {
                return entry_ref.path.clone();
            }
        }

        // No ancestor (including the path itself) was found in the snapshot.
        // Return an empty path representing the root of the snapshot context.
        Arc::from(PathBuf::from(ROOT_PATH))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{path::PathBuf, sync::atomic::AtomicUsize};

    fn create_test_entry(id: usize, path: &str, kind: EntryKind) -> PhysicalEntry {
        PhysicalEntry {
            id: EntryId::new(&Arc::new(AtomicUsize::new(id))),
            path: Arc::from(PathBuf::from(path)),
            kind,
            mtime: None,
            file_id: FileId::new_inode(0, 0),
        }
    }

    #[test]
    fn test_remove_file() {
        let mut snapshot = PhysicalSnapshot::new(Arc::from(PathBuf::from("/root")));
        let entry = create_test_entry(1, "test.txt", EntryKind::File);
        let entry_ref = Arc::new(entry.clone());

        snapshot.create_entry(entry_ref.clone());
        assert_eq!(snapshot.count_files(), 1);

        let removed = snapshot.remove_entry("test.txt");
        assert_eq!(snapshot.count_files(), 0);
        assert!(snapshot.entry_by_path("test.txt").is_none());

        // Verify the returned result
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0].path.to_string_lossy(), "test.txt");
        assert_eq!(removed[0].id, entry.id);
    }

    #[test]
    fn test_remove_nonexistent_file() {
        let mut snapshot = PhysicalSnapshot::new(Arc::from(PathBuf::from("/root")));
        let removed = snapshot.remove_entry("nonexistent.txt");
        assert_eq!(snapshot.count_files(), 0);

        // Verify the result is empty
        assert_eq!(removed.len(), 0);
    }

    #[test]
    fn test_remove_directory_with_files() {
        let mut snapshot = PhysicalSnapshot::new(Arc::from(PathBuf::from("/root")));

        // Create a directory and some files inside it
        let dir_entry = create_test_entry(1, "test_dir", EntryKind::Dir);
        let file1_entry = create_test_entry(2, "test_dir/file1.txt", EntryKind::File);
        let file2_entry = create_test_entry(3, "test_dir/file2.txt", EntryKind::File);

        snapshot.create_entry(Arc::new(dir_entry.clone()));
        snapshot.create_entry(Arc::new(file1_entry.clone()));
        snapshot.create_entry(Arc::new(file2_entry.clone()));

        assert_eq!(snapshot.count_files(), 3);

        // Remove the directory
        let removed = snapshot.remove_entry("test_dir");

        // Verify all entries are removed
        assert_eq!(snapshot.count_files(), 0);
        assert!(snapshot.entry_by_path("test_dir").is_none());
        assert!(snapshot.entry_by_path("test_dir/file1.txt").is_none());
        assert!(snapshot.entry_by_path("test_dir/file2.txt").is_none());

        // Verify the returned result
        assert_eq!(removed.len(), 3);

        // Check that all removed paths and IDs are present in the result
        let paths: Vec<String> = removed
            .iter()
            .map(|entry| entry.path.to_string_lossy().to_string())
            .collect();
        let ids: Vec<EntryId> = removed.iter().map(|entry| entry.id).collect();

        assert!(paths.contains(&"test_dir".to_string()));
        assert!(paths.contains(&"test_dir/file1.txt".to_string()));
        assert!(paths.contains(&"test_dir/file2.txt".to_string()));

        assert!(ids.contains(&dir_entry.id));
        assert!(ids.contains(&file1_entry.id));
        assert!(ids.contains(&file2_entry.id));
    }

    #[test]
    fn test_remove_nested_directory() {
        let mut snapshot = PhysicalSnapshot::new(Arc::from(PathBuf::from("/root")));

        // Create a nested directory structure
        let dir1_entry = create_test_entry(1, "dir1", EntryKind::Dir);
        let dir2_entry = create_test_entry(2, "dir1/dir2", EntryKind::Dir);
        let file_entry = create_test_entry(3, "dir1/dir2/file.txt", EntryKind::File);

        snapshot.create_entry(Arc::new(dir1_entry.clone()));
        snapshot.create_entry(Arc::new(dir2_entry.clone()));
        snapshot.create_entry(Arc::new(file_entry.clone()));

        assert_eq!(snapshot.count_files(), 3);

        // Remove the parent directory
        let removed = snapshot.remove_entry("dir1");

        // Verify all entries are removed
        assert_eq!(snapshot.count_files(), 0);
        assert!(snapshot.entry_by_path("dir1").is_none());
        assert!(snapshot.entry_by_path("dir1/dir2").is_none());
        assert!(snapshot.entry_by_path("dir1/dir2/file.txt").is_none());

        // Verify the returned result
        assert_eq!(removed.len(), 3);

        // Check that all removed paths and IDs are present in the result
        let paths: Vec<String> = removed
            .iter()
            .map(|entry| entry.path.to_string_lossy().to_string())
            .collect();
        let ids: Vec<EntryId> = removed.iter().map(|entry| entry.id).collect();

        assert!(paths.contains(&"dir1".to_string()));
        assert!(paths.contains(&"dir1/dir2".to_string()));
        assert!(paths.contains(&"dir1/dir2/file.txt".to_string()));

        assert!(ids.contains(&dir1_entry.id));
        assert!(ids.contains(&dir2_entry.id));
        assert!(ids.contains(&file_entry.id));
    }

    #[test]
    fn test_remove_partial_path() {
        let mut snapshot = PhysicalSnapshot::new(Arc::from(PathBuf::from("/root")));

        // Create entries with similar prefixes
        let dir1_entry = create_test_entry(1, "test", EntryKind::Dir);
        let dir2_entry = create_test_entry(2, "test_dir", EntryKind::Dir);
        let file_entry = create_test_entry(3, "test_file.txt", EntryKind::File);

        snapshot.create_entry(Arc::new(dir1_entry.clone()));
        snapshot.create_entry(Arc::new(dir2_entry));
        snapshot.create_entry(Arc::new(file_entry));

        assert_eq!(snapshot.count_files(), 3);

        // Remove only the "test" directory
        let removed = snapshot.remove_entry("test");

        // Verify only the "test" directory is removed
        assert_eq!(snapshot.count_files(), 2);
        assert!(snapshot.entry_by_path("test").is_none());
        assert!(snapshot.entry_by_path("test_dir").is_some());
        assert!(snapshot.entry_by_path("test_file.txt").is_some());

        // Verify the returned result
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0].path.to_string_lossy(), "test");
        assert_eq!(removed[0].id, dir1_entry.id);
    }

    #[test]
    fn test_lowest_ancestor_path_exact_match() {
        let mut snapshot = PhysicalSnapshot::new(Arc::from(PathBuf::from("/root")));

        // Create some entries
        let dir_entry = create_test_entry(1, "dir1", EntryKind::Dir);
        let subdir_entry = create_test_entry(2, "dir1/dir2", EntryKind::Dir);
        let file_entry = create_test_entry(3, "dir1/dir2/file.txt", EntryKind::File);

        snapshot.create_entry(Arc::new(dir_entry));
        snapshot.create_entry(Arc::new(subdir_entry));
        snapshot.create_entry(Arc::new(file_entry));

        // Test exact path match
        let result = snapshot.lowest_ancestor_path("dir1/dir2/file.txt");
        assert_eq!(result.to_string_lossy(), "dir1/dir2/file.txt");
    }

    #[test]
    fn test_lowest_ancestor_path_direct_ancestor() {
        let mut snapshot = PhysicalSnapshot::new(Arc::from(PathBuf::from("/root")));

        // Create some entries, but not the leaf
        let dir_entry = create_test_entry(1, "dir1", EntryKind::Dir);
        let subdir_entry = create_test_entry(2, "dir1/dir2", EntryKind::Dir);

        snapshot.create_entry(Arc::new(dir_entry));
        snapshot.create_entry(Arc::new(subdir_entry));

        // Test path that doesn't exist but has ancestors
        let result = snapshot.lowest_ancestor_path("dir1/dir2/file.txt");
        assert_eq!(result.to_string_lossy(), "dir1/dir2");
    }

    #[test]
    fn test_lowest_ancestor_path_multiple_ancestors() {
        let mut snapshot = PhysicalSnapshot::new(Arc::from(PathBuf::from("/root")));

        // Create some entries
        let dir_entry = create_test_entry(1, "dir1", EntryKind::Dir);

        snapshot.create_entry(Arc::new(dir_entry));

        // Test deeply nested path - should find the lowest/closest ancestor
        let result = snapshot.lowest_ancestor_path("dir1/dir2/dir3/dir4/file.txt");
        assert_eq!(result.to_string_lossy(), "dir1");
    }

    #[test]
    fn test_lowest_ancestor_path_no_ancestors() {
        let snapshot = PhysicalSnapshot::new(Arc::from(PathBuf::from("/root")));

        // Test path with no ancestors in the snapshot
        let result = snapshot.lowest_ancestor_path("non/existent/path");
        assert_eq!(result.to_string_lossy(), ROOT_PATH);
    }

    #[test]
    fn test_lowest_ancestor_path_empty_snapshot() {
        let snapshot = PhysicalSnapshot::new(Arc::from(PathBuf::from("/root")));

        // Test any path with empty snapshot
        let result = snapshot.lowest_ancestor_path("dir1/file.txt");
        assert_eq!(result.to_string_lossy(), ROOT_PATH);
    }

    #[test]
    fn test_lowest_ancestor_path_root_level_file() {
        let mut snapshot = PhysicalSnapshot::new(Arc::from(PathBuf::from("/root")));

        // Create a root-level file
        let file_entry = create_test_entry(1, "file.txt", EntryKind::File);
        snapshot.create_entry(Arc::new(file_entry));

        // Test a different root-level file
        let result = snapshot.lowest_ancestor_path("different.txt");
        assert_eq!(result.to_string_lossy(), ROOT_PATH);

        // Test the actual file
        let result = snapshot.lowest_ancestor_path("file.txt");
        assert_eq!(result.to_string_lossy(), "file.txt");
    }
}
