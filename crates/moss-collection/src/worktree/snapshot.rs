use anyhow::Result;
use moss_file::toml;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    prelude::DiGraphMap,
    visit::EdgeRef,
};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
    path::Path,
    sync::Arc,
};
use uuid::Uuid;

use crate::{
    configuration::ConfigurationModel,
    worktree::constants::{ROOT_ID, ROOT_PATH, ROOT_UNLOADED_ID},
};

pub type UnloadedId = usize;
pub type UnloadedParentId = UnloadedId;

#[derive(Debug, Clone)]
pub enum UnloadedEntry {
    Item {
        id: UnloadedId,
        abs_path: Arc<Path>,
        path: Arc<Path>,
    },
    Dir {
        id: UnloadedId,
        abs_path: Arc<Path>,
        path: Arc<Path>,
    },
}

impl UnloadedEntry {
    pub fn id(&self) -> UnloadedId {
        match self {
            UnloadedEntry::Item { id, .. } => *id,
            UnloadedEntry::Dir { id, .. } => *id,
        }
    }

    pub fn path(&self) -> &Arc<Path> {
        match self {
            UnloadedEntry::Item { path, .. } => path,
            UnloadedEntry::Dir { path, .. } => path,
        }
    }

    pub fn is_root(&self) -> bool {
        self.id() == ROOT_UNLOADED_ID
    }
}

pub struct Entry {
    pub id: Uuid,
    pub name: String,
    pub path: Arc<Path>,
    pub is_dir: bool,

    /// The configuration file for the entry.
    /// Should always be present for non-root entries.
    pub config: Option<toml::EditableInPlaceFileHandle<ConfigurationModel>>,
}

impl Entry {
    pub fn is_root(&self) -> bool {
        self.id == ROOT_ID
    }

    pub fn config(&self) -> &toml::EditableInPlaceFileHandle<ConfigurationModel> {
        debug_assert!(!self.is_root(), "Root entry does not have a config");

        self.config.as_ref().unwrap()
    }

    pub fn config_mut(&mut self) -> &mut toml::EditableInPlaceFileHandle<ConfigurationModel> {
        debug_assert!(!self.is_root(), "Root entry does not have a config");

        self.config.as_mut().unwrap()
    }
}

pub struct Snapshot {
    entries: DiGraph<Entry, ()>,
    entries_by_id: HashMap<Uuid, NodeIndex>,
    entries_by_path: HashMap<Arc<Path>, NodeIndex>,
    unloaded_entries: DiGraphMap<UnloadedId, ()>,
    unloaded_entries_by_id: HashMap<UnloadedId, UnloadedEntry>,
    unloaded_entries_by_path: HashMap<Arc<Path>, UnloadedId>,
    known_paths: HashSet<Arc<Path>>,
}

impl From<Vec<(UnloadedEntry, Option<UnloadedParentId>)>> for Snapshot {
    fn from(mut list: Vec<(UnloadedEntry, Option<UnloadedParentId>)>) -> Self {
        debug_assert!(
            list.len() > 0,
            "At least one the root entry must be present"
        );

        list.sort_by_key(|(e, _)| e.path().components().count());

        let mut known_paths = HashSet::new();
        let mut unloaded_entries_by_id = HashMap::new();
        let mut unloaded_entries_by_path = HashMap::new();
        let mut unloaded_entries = DiGraphMap::new();

        for (unloaded_entry, parent) in list {
            let id = unloaded_entry.id();
            let path = Arc::clone(unloaded_entry.path());

            unloaded_entries.add_node(id);
            unloaded_entries_by_id.insert(id, unloaded_entry.clone());
            unloaded_entries_by_path.insert(path.clone(), id);
            known_paths.insert(path);

            parent.map(|idx| unloaded_entries.add_edge(idx, id, ()));
        }

        Self {
            entries: DiGraph::new(),
            entries_by_id: HashMap::new(),
            entries_by_path: HashMap::new(),
            unloaded_entries,
            unloaded_entries_by_id,
            unloaded_entries_by_path,
            known_paths,
        }
    }
}

impl Snapshot {
    pub fn root_idx(&self) -> NodeIndex {
        self.entries_by_id
            .get(&Uuid::nil())
            .expect("The root entry must be present")
            .clone()
    }

    pub fn is_loaded(&self, path: &Path) -> bool {
        self.entries_by_path.contains_key(path)
    }

    pub fn unloaded_entry_by_path(&self, path: &Path) -> Option<&UnloadedEntry> {
        self.unloaded_entries_by_path
            .get(path)
            .and_then(|id| self.unloaded_entries_by_id.get(id))
    }

    pub fn unloaded_entry_children_by_path(&self, parent_path: &Path) -> Vec<UnloadedEntry> {
        self.unloaded_entries_by_path
            .iter()
            .filter_map(|(child_path, &child_id)| {
                if let Some(parent) = child_path.parent() {
                    if parent == parent_path {
                        self.unloaded_entries_by_id.get(&child_id).cloned()
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn load_entry(&mut self, unloaded_id: usize, entry: Entry) -> Result<Uuid> {
        let parent_path = entry.path.parent().unwrap_or(Path::new(ROOT_PATH));

        self.unloaded_entries.remove_node(unloaded_id);
        self.unloaded_entries_by_path.remove(&entry.path);

        let id = entry.id;
        if entry.is_root() {
            self.create_entry(entry, None)?;
        } else if let Some(parent_idx) = self.entries_by_path.get(parent_path) {
            let parent_id = self.entries[*parent_idx].id;
            self.create_entry(entry, Some(parent_id))?;
        } else {
            anyhow::bail!("Child entry cannot be loaded before its parent");
        }

        Ok(id)
    }

    pub fn create_entry(&mut self, entry: Entry, parent_id: Option<Uuid>) -> Result<NodeIndex> {
        let id = entry.id;
        let path = Arc::clone(&entry.path);
        let is_root = entry.is_root();

        let idx = self.entries.try_add_node(entry)?;
        self.entries_by_id.insert(id, idx);
        self.entries_by_path.insert(path, idx);

        if let Some(parent_id) = parent_id {
            if let Some(&parent_idx) = self.entries_by_id.get(&parent_id) {
                self.entries.try_add_edge(parent_idx, idx, ())?;
            }
        } else if !is_root {
            self.entries.try_add_edge(self.root_idx(), idx, ())?;
        }

        Ok(idx)
    }

    pub fn entry_by_id(&self, id: Uuid) -> Option<&Entry> {
        let idx = self.entries_by_id.get(&id)?;
        Some(&self.entries[*idx])
    }

    pub fn entry_by_id_unchecked(&self, id: Uuid) -> &Entry {
        let idx = self.entries_by_id[&id];
        &self.entries[idx]
    }

    pub fn entry_by_id_mut(&mut self, id: Uuid) -> Option<&mut Entry> {
        let idx = self.entries_by_id.get(&id)?;
        Some(&mut self.entries[*idx])
    }

    pub fn entry_by_id_mut_unchecked(&mut self, id: Uuid) -> &mut Entry {
        let idx = self.entries_by_id[&id];
        &mut self.entries[idx]
    }

    pub fn entry_by_path(&self, path: &Path) -> Option<&Entry> {
        let idx = self.entries_by_path.get(path)?;
        Some(&self.entries[*idx])
    }

    pub fn entry_by_path_mut(&mut self, path: &Path) -> Option<&mut Entry> {
        let idx = self.entries_by_path.get(path)?;
        Some(&mut self.entries[*idx])
    }

    pub fn lowest_ancestor_path(&self, path: &Path) -> Arc<Path> {
        for ancestor in path.ancestors() {
            if let Some(unloaded_entry) = self.known_paths.get(ancestor) {
                return unloaded_entry.clone();
            }
        }
        Path::new(ROOT_PATH).into()
    }

    pub fn move_entry(&mut self, entry_id: Uuid, new_parent_id: Uuid) -> Result<()> {
        let entry_idx = *self
            .entries_by_id
            .get(&entry_id)
            .ok_or_else(|| anyhow::anyhow!("Entry not found"))?;

        let parent_idx = *self
            .entries_by_id
            .get(&new_parent_id)
            .ok_or_else(|| anyhow::anyhow!("New parent not found"))?;

        let old_path = self.entries[entry_idx].path.clone();
        let parent_path = self.entries[parent_idx].path.clone();
        let entry_name = self.entries[entry_idx].name.clone();

        let new_path: Arc<Path> = parent_path.join(&entry_name).into();

        self.entries_by_path.remove(&old_path);

        let entry = &mut self.entries[entry_idx];
        entry.path = new_path.clone();

        self.entries_by_path.insert(new_path.clone(), entry_idx);
        self.known_paths.remove(&old_path);
        self.known_paths.insert(new_path);

        let old_edges: Vec<_> = self
            .entries
            .edges_directed(entry_idx, petgraph::Incoming)
            .map(|edge| edge.id())
            .collect();

        for edge_id in old_edges {
            self.entries.remove_edge(edge_id);
        }

        self.entries.add_edge(parent_idx, entry_idx, ());

        Ok(())
    }

    pub fn remove_entry(&mut self, id: Uuid) -> Option<Entry> {
        let idx = self.entries_by_id.get(&id);
        if idx.is_none() {
            return None;
        }
        let idx = *idx.unwrap();

        let entry = self.entries.remove_node(idx);

        if let Some(entry) = &entry {
            self.entries_by_id.remove(&entry.id);
            self.entries_by_path.remove(&entry.path);
            self.known_paths.remove(&entry.path);
        }

        entry
    }

    pub fn remove_unloaded_by_prefix(&mut self, prefix: &Path) {
        let paths_to_remove: Vec<Arc<Path>> = self
            .unloaded_entries_by_path
            .keys()
            .filter(|path| path.starts_with(prefix))
            .cloned()
            .collect();

        for path in paths_to_remove {
            let id = match self.unloaded_entries_by_path.get(&path) {
                Some(&id) => id,
                None => continue,
            };

            self.unloaded_entries.remove_node(id);
            self.unloaded_entries_by_id.remove(&id);
            self.known_paths.remove(&path);
            self.unloaded_entries_by_path.remove(&path);
        }
    }

    pub fn entry_parent_id(&self, entry_id: Uuid) -> Option<Uuid> {
        let entry_idx = self.entries_by_id.get(&entry_id)?;

        let parent_edges: Vec<_> = self
            .entries
            .edges_directed(*entry_idx, petgraph::Incoming)
            .collect();

        if let Some(parent_edge) = parent_edges.first() {
            Some(self.entries[parent_edge.source()].id)
        } else {
            None // Root has no parent
        }
    }

    pub fn entry_node_index(&self, entry_id: Uuid) -> Option<NodeIndex> {
        self.entries_by_id.get(&entry_id).copied()
    }

    pub fn collect_loaded_descendants(&self, entry_id: Uuid) -> Vec<Uuid> {
        let mut descendants = Vec::new();
        let mut to_visit = vec![entry_id];

        while let Some(current_id) = to_visit.pop() {
            // Find all loaded children of current node
            if let Some(current_idx) = self.entries_by_id.get(&current_id) {
                let children: Vec<Uuid> = self
                    .entries
                    .edges_directed(*current_idx, petgraph::Outgoing)
                    .map(|edge| self.entries[edge.target()].id)
                    .collect();

                for child_id in children {
                    descendants.push(child_id);
                    to_visit.push(child_id);
                }
            }
        }

        // Reverse to get bottom-up order (deepest nodes first)
        descendants.reverse();
        descendants
    }
}

impl Display for Snapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "*")?;

        // Collect children of root, sort them by path
        let mut children: Vec<NodeIndex> = self
            .entries
            .edges_directed(self.root_idx(), petgraph::Outgoing)
            .map(|edge| edge.target())
            .collect();
        children.sort_by_key(|&child_idx| self.entries[child_idx].path.clone());

        let last = children.len().saturating_sub(1);
        for (i, &child_idx) in children.iter().enumerate() {
            let is_last = i == last;
            self.fmt_subtree(child_idx, "", is_last, f)?;
        }

        Ok(())
    }
}

impl Snapshot {
    fn fmt_subtree(
        &self,
        idx: NodeIndex,
        prefix: &str,
        is_last: bool,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        let branch = if is_last { "└── " } else { "├── " };
        let entry = &self.entries[idx];
        let name = entry
            .path
            .file_name()
            .unwrap_or_else(|| entry.path.as_os_str())
            .to_string_lossy();

        let display_name = if entry.is_dir {
            format!("/{}", name)
        } else {
            name.to_string()
        };

        writeln!(f, "{}{}{}", prefix, branch, display_name)?;

        let child_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        let mut children: Vec<NodeIndex> = self
            .entries
            .edges_directed(idx, petgraph::Outgoing)
            .map(|edge| edge.target())
            .collect();
        children.sort_by_key(|&child_idx| self.entries[child_idx].path.clone());

        // Recursively print all children
        let last = children.len().saturating_sub(1);
        for (i, &child_idx) in children.iter().enumerate() {
            let is_last_child = i == last;
            self.fmt_subtree(child_idx, &child_prefix, is_last_child, f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_test_entry(id: Uuid, name: &str, path: &str, is_dir: bool) -> Entry {
        Entry {
            id,
            name: name.to_string(),
            path: Path::new(path).into(),
            is_dir,
            config: if id == ROOT_ID {
                None
            } else {
                // For tests, we'll use None for non-root entries too since we can't easily create EditableInPlaceFileHandle
                None
            },
        }
    }

    fn create_test_unloaded_entries() -> Vec<(UnloadedEntry, Option<UnloadedParentId>)> {
        vec![
            (
                UnloadedEntry::Dir {
                    id: ROOT_UNLOADED_ID,
                    abs_path: Path::new("/test").into(),
                    path: Path::new("").into(),
                },
                None,
            ),
            (
                UnloadedEntry::Dir {
                    id: 1,
                    abs_path: Path::new("/test/foo").into(),
                    path: Path::new("foo").into(),
                },
                Some(ROOT_UNLOADED_ID),
            ),
            (
                UnloadedEntry::Item {
                    id: 2,
                    abs_path: Path::new("/test/foo/bar").into(),
                    path: Path::new("foo/bar").into(),
                },
                Some(1),
            ),
            (
                UnloadedEntry::Dir {
                    id: 3,
                    abs_path: Path::new("/test/baz").into(),
                    path: Path::new("baz").into(),
                },
                Some(ROOT_UNLOADED_ID),
            ),
        ]
    }

    #[test]
    fn test_snapshot_creation_from_unloaded_entries() {
        let unloaded_entries = create_test_unloaded_entries();
        let snapshot = Snapshot::from(unloaded_entries);

        // Check that unloaded entries are properly stored
        assert!(snapshot.unloaded_entry_by_path(Path::new("")).is_some());
        assert!(snapshot.unloaded_entry_by_path(Path::new("foo")).is_some());
        assert!(
            snapshot
                .unloaded_entry_by_path(Path::new("foo/bar"))
                .is_some()
        );
        assert!(snapshot.unloaded_entry_by_path(Path::new("baz")).is_some());
        assert!(
            snapshot
                .unloaded_entry_by_path(Path::new("nonexistent"))
                .is_none()
        );

        // Check known paths
        assert!(snapshot.known_paths.contains(Path::new("")));
        assert!(snapshot.known_paths.contains(Path::new("foo")));
        assert!(snapshot.known_paths.contains(Path::new("foo/bar")));
        assert!(snapshot.known_paths.contains(Path::new("baz")));
    }

    #[test]
    fn test_unloaded_entry_children_by_path() {
        let unloaded_entries = create_test_unloaded_entries();
        let snapshot = Snapshot::from(unloaded_entries);

        // Test root children
        let root_children = snapshot.unloaded_entry_children_by_path(Path::new(""));
        assert_eq!(root_children.len(), 2);
        let child_paths: Vec<_> = root_children.iter().map(|e| e.path().as_ref()).collect();
        assert!(child_paths.contains(&Path::new("foo")));
        assert!(child_paths.contains(&Path::new("baz")));

        // Test foo children
        let foo_children = snapshot.unloaded_entry_children_by_path(Path::new("foo"));
        assert_eq!(foo_children.len(), 1);
        assert_eq!(foo_children[0].path().as_ref(), Path::new("foo/bar"));

        // Test path with no children
        let baz_children = snapshot.unloaded_entry_children_by_path(Path::new("baz"));
        assert_eq!(baz_children.len(), 0);
    }

    #[test]
    fn test_load_entry() {
        let unloaded_entries = create_test_unloaded_entries();
        let mut snapshot = Snapshot::from(unloaded_entries);

        // Load root entry
        let root_entry = create_test_entry(ROOT_ID, "", "", true);
        let root_id = snapshot.load_entry(ROOT_UNLOADED_ID, root_entry).unwrap();
        assert_eq!(root_id, ROOT_ID);
        assert!(snapshot.is_loaded(Path::new("")));

        // Load foo entry (child of root)
        let foo_id = Uuid::new_v4();
        let foo_entry = create_test_entry(foo_id, "foo", "foo", true);
        let loaded_foo_id = snapshot.load_entry(1, foo_entry).unwrap();
        assert_eq!(loaded_foo_id, foo_id);
        assert!(snapshot.is_loaded(Path::new("foo")));

        // Verify the entry can be retrieved
        let retrieved_foo = snapshot.entry_by_id(foo_id).unwrap();
        assert_eq!(retrieved_foo.name, "foo");
        assert_eq!(retrieved_foo.path.as_ref(), Path::new("foo"));
        assert!(retrieved_foo.is_dir);
    }

    #[test]
    fn test_create_entry() {
        let mut snapshot = Snapshot::from(vec![(
            UnloadedEntry::Dir {
                id: ROOT_UNLOADED_ID,
                abs_path: Path::new("/test").into(),
                path: Path::new("").into(),
            },
            None,
        )]);

        // Create root entry first
        let root_entry = create_test_entry(ROOT_ID, "", "", true);
        snapshot.create_entry(root_entry, None).unwrap();

        // Create a child entry
        let child_id = Uuid::new_v4();
        let child_entry = create_test_entry(child_id, "child", "child", false);
        let _child_idx = snapshot.create_entry(child_entry, Some(ROOT_ID)).unwrap();

        // Verify the entry was created
        assert!(snapshot.is_loaded(Path::new("child")));
        let retrieved_child = snapshot.entry_by_id(child_id).unwrap();
        assert_eq!(retrieved_child.name, "child");
        assert!(!retrieved_child.is_dir);

        // Verify parent-child relationship
        let parent_id = snapshot.entry_parent_id(child_id).unwrap();
        assert_eq!(parent_id, ROOT_ID);
    }

    #[test]
    fn test_entry_retrieval_methods() {
        let mut snapshot = Snapshot::from(vec![(
            UnloadedEntry::Dir {
                id: ROOT_UNLOADED_ID,
                abs_path: Path::new("/test").into(),
                path: Path::new("").into(),
            },
            None,
        )]);

        let root_entry = create_test_entry(ROOT_ID, "", "", true);
        snapshot.create_entry(root_entry, None).unwrap();

        let test_id = Uuid::new_v4();
        let test_entry = create_test_entry(test_id, "test", "test", false);
        snapshot.create_entry(test_entry, Some(ROOT_ID)).unwrap();

        // Test entry_by_id
        assert!(snapshot.entry_by_id(test_id).is_some());
        assert!(snapshot.entry_by_id(Uuid::new_v4()).is_none());

        // Test entry_by_id_unchecked
        let entry = snapshot.entry_by_id_unchecked(test_id);
        assert_eq!(entry.id, test_id);

        // Test entry_by_path
        assert!(snapshot.entry_by_path(Path::new("test")).is_some());
        assert!(snapshot.entry_by_path(Path::new("nonexistent")).is_none());

        // Test mutable versions
        assert!(snapshot.entry_by_id_mut(test_id).is_some());
        assert!(snapshot.entry_by_id_mut(Uuid::new_v4()).is_none());
        assert!(snapshot.entry_by_path_mut(Path::new("test")).is_some());
        assert!(
            snapshot
                .entry_by_path_mut(Path::new("nonexistent"))
                .is_none()
        );
    }

    #[test]
    fn test_move_entry() {
        let mut snapshot = Snapshot::from(vec![(
            UnloadedEntry::Dir {
                id: ROOT_UNLOADED_ID,
                abs_path: Path::new("/test").into(),
                path: Path::new("").into(),
            },
            None,
        )]);

        // Create root
        let root_entry = create_test_entry(ROOT_ID, "", "", true);
        snapshot.create_entry(root_entry, None).unwrap();

        // Create source directory
        let source_id = Uuid::new_v4();
        let source_entry = create_test_entry(source_id, "source", "source", true);
        snapshot.create_entry(source_entry, Some(ROOT_ID)).unwrap();

        // Create target directory
        let target_id = Uuid::new_v4();
        let target_entry = create_test_entry(target_id, "target", "target", true);
        snapshot.create_entry(target_entry, Some(ROOT_ID)).unwrap();

        // Create entry to move
        let move_id = Uuid::new_v4();
        let move_entry = create_test_entry(move_id, "moveme", "source/moveme", false);
        snapshot.create_entry(move_entry, Some(source_id)).unwrap();

        // Verify initial state
        assert!(snapshot.is_loaded(Path::new("source/moveme")));
        assert!(!snapshot.is_loaded(Path::new("target/moveme")));

        // Move the entry
        snapshot.move_entry(move_id, target_id).unwrap();

        // Verify the move
        assert!(!snapshot.is_loaded(Path::new("source/moveme")));
        assert!(snapshot.is_loaded(Path::new("target/moveme")));

        let moved_entry = snapshot.entry_by_id(move_id).unwrap();
        assert_eq!(moved_entry.path.as_ref(), Path::new("target/moveme"));

        // Verify parent relationship changed
        let new_parent_id = snapshot.entry_parent_id(move_id).unwrap();
        assert_eq!(new_parent_id, target_id);
    }

    #[test]
    fn test_remove_entry() {
        let mut snapshot = Snapshot::from(vec![(
            UnloadedEntry::Dir {
                id: ROOT_UNLOADED_ID,
                abs_path: Path::new("/test").into(),
                path: Path::new("").into(),
            },
            None,
        )]);

        // Create root
        let root_entry = create_test_entry(ROOT_ID, "", "", true);
        snapshot.create_entry(root_entry, None).unwrap();

        // Create entry to remove
        let remove_id = Uuid::new_v4();
        let remove_entry = create_test_entry(remove_id, "remove", "remove", false);
        snapshot.create_entry(remove_entry, Some(ROOT_ID)).unwrap();

        // Verify entry exists
        assert!(snapshot.is_loaded(Path::new("remove")));
        assert!(snapshot.entry_by_id(remove_id).is_some());

        // Remove the entry
        let removed_entry = snapshot.remove_entry(remove_id).unwrap();
        assert_eq!(removed_entry.id, remove_id);
        assert_eq!(removed_entry.name, "remove");

        // Verify entry is gone
        assert!(!snapshot.is_loaded(Path::new("remove")));
        assert!(snapshot.entry_by_id(remove_id).is_none());
        assert!(!snapshot.known_paths.contains(Path::new("remove")));

        // Test removing non-existent entry
        assert!(snapshot.remove_entry(Uuid::new_v4()).is_none());
    }

    #[test]
    fn test_remove_unloaded_by_prefix() {
        let unloaded_entries = create_test_unloaded_entries();
        let mut snapshot = Snapshot::from(unloaded_entries);

        // Verify initial state
        assert!(snapshot.unloaded_entry_by_path(Path::new("foo")).is_some());
        assert!(
            snapshot
                .unloaded_entry_by_path(Path::new("foo/bar"))
                .is_some()
        );
        assert!(snapshot.unloaded_entry_by_path(Path::new("baz")).is_some());

        // Remove all entries with "foo" prefix
        snapshot.remove_unloaded_by_prefix(Path::new("foo"));

        // Verify foo entries are removed
        assert!(snapshot.unloaded_entry_by_path(Path::new("foo")).is_none());
        assert!(
            snapshot
                .unloaded_entry_by_path(Path::new("foo/bar"))
                .is_none()
        );

        // Verify other entries remain
        assert!(snapshot.unloaded_entry_by_path(Path::new("")).is_some());
        assert!(snapshot.unloaded_entry_by_path(Path::new("baz")).is_some());

        // Verify known_paths are updated
        assert!(!snapshot.known_paths.contains(Path::new("foo")));
        assert!(!snapshot.known_paths.contains(Path::new("foo/bar")));
        assert!(snapshot.known_paths.contains(Path::new("baz")));
    }

    #[test]
    fn test_entry_parent_id() {
        let mut snapshot = Snapshot::from(vec![(
            UnloadedEntry::Dir {
                id: ROOT_UNLOADED_ID,
                abs_path: Path::new("/test").into(),
                path: Path::new("").into(),
            },
            None,
        )]);

        // Create root
        let root_entry = create_test_entry(ROOT_ID, "", "", true);
        snapshot.create_entry(root_entry, None).unwrap();

        // Create child
        let child_id = Uuid::new_v4();
        let child_entry = create_test_entry(child_id, "child", "child", false);
        snapshot.create_entry(child_entry, Some(ROOT_ID)).unwrap();

        // Create grandchild
        let grandchild_id = Uuid::new_v4();
        let grandchild_entry =
            create_test_entry(grandchild_id, "grandchild", "child/grandchild", false);
        snapshot
            .create_entry(grandchild_entry, Some(child_id))
            .unwrap();

        // Test parent relationships
        assert_eq!(snapshot.entry_parent_id(ROOT_ID), None); // Root has no parent
        assert_eq!(snapshot.entry_parent_id(child_id), Some(ROOT_ID));
        assert_eq!(snapshot.entry_parent_id(grandchild_id), Some(child_id));
        assert_eq!(snapshot.entry_parent_id(Uuid::new_v4()), None); // Non-existent entry
    }

    #[test]
    fn test_collect_loaded_descendants() {
        let mut snapshot = Snapshot::from(vec![(
            UnloadedEntry::Dir {
                id: ROOT_UNLOADED_ID,
                abs_path: Path::new("/test").into(),
                path: Path::new("").into(),
            },
            None,
        )]);

        // Create root
        let root_entry = create_test_entry(ROOT_ID, "", "", true);
        snapshot.create_entry(root_entry, None).unwrap();

        // Create a tree structure
        let parent_id = Uuid::new_v4();
        let parent_entry = create_test_entry(parent_id, "parent", "parent", true);
        snapshot.create_entry(parent_entry, Some(ROOT_ID)).unwrap();

        let child1_id = Uuid::new_v4();
        let child1_entry = create_test_entry(child1_id, "child1", "parent/child1", false);
        snapshot
            .create_entry(child1_entry, Some(parent_id))
            .unwrap();

        let child2_id = Uuid::new_v4();
        let child2_entry = create_test_entry(child2_id, "child2", "parent/child2", true);
        snapshot
            .create_entry(child2_entry, Some(parent_id))
            .unwrap();

        let grandchild_id = Uuid::new_v4();
        let grandchild_entry = create_test_entry(
            grandchild_id,
            "grandchild",
            "parent/child2/grandchild",
            false,
        );
        snapshot
            .create_entry(grandchild_entry, Some(child2_id))
            .unwrap();

        // Test collecting descendants
        let descendants = snapshot.collect_loaded_descendants(parent_id);

        // Should return all descendants in bottom-up order (deepest first)
        assert_eq!(descendants.len(), 3);
        assert!(descendants.contains(&grandchild_id));
        assert!(descendants.contains(&child1_id));
        assert!(descendants.contains(&child2_id));

        // Grandchild should come before its parent (child2) due to reverse order
        let grandchild_pos = descendants
            .iter()
            .position(|&id| id == grandchild_id)
            .unwrap();
        let child2_pos = descendants.iter().position(|&id| id == child2_id).unwrap();
        assert!(grandchild_pos < child2_pos);

        // Test with leaf node (no descendants)
        let leaf_descendants = snapshot.collect_loaded_descendants(child1_id);
        assert_eq!(leaf_descendants.len(), 0);

        // Test with non-existent entry
        let empty_descendants = snapshot.collect_loaded_descendants(Uuid::new_v4());
        assert_eq!(empty_descendants.len(), 0);
    }

    #[test]
    fn test_lowest_ancestor_path() {
        let unloaded_entries = create_test_unloaded_entries();
        let snapshot = Snapshot::from(unloaded_entries);

        // Test with exact match
        let ancestor = snapshot.lowest_ancestor_path(Path::new("foo"));
        assert_eq!(ancestor.as_ref(), Path::new("foo"));

        // Test with child path
        let ancestor = snapshot.lowest_ancestor_path(Path::new("foo/bar/baz"));
        assert_eq!(ancestor.as_ref(), Path::new("foo/bar"));

        // Test with deep path where only root exists
        let ancestor = snapshot.lowest_ancestor_path(Path::new("nonexistent/deep/path"));
        assert_eq!(ancestor.as_ref(), Path::new(""));

        // Test with root path
        let ancestor = snapshot.lowest_ancestor_path(Path::new(""));
        assert_eq!(ancestor.as_ref(), Path::new(""));
    }

    #[test]
    fn test_unloaded_entry_methods() {
        let entry_item = UnloadedEntry::Item {
            id: 1,
            abs_path: Path::new("/test/item").into(),
            path: Path::new("item").into(),
        };

        let entry_dir = UnloadedEntry::Dir {
            id: 2,
            abs_path: Path::new("/test/dir").into(),
            path: Path::new("dir").into(),
        };

        let entry_root = UnloadedEntry::Dir {
            id: ROOT_UNLOADED_ID,
            abs_path: Path::new("/test").into(),
            path: Path::new("").into(),
        };

        // Test id() method
        assert_eq!(entry_item.id(), 1);
        assert_eq!(entry_dir.id(), 2);
        assert_eq!(entry_root.id(), ROOT_UNLOADED_ID);

        // Test path() method
        assert_eq!(entry_item.path().as_ref(), Path::new("item"));
        assert_eq!(entry_dir.path().as_ref(), Path::new("dir"));
        assert_eq!(entry_root.path().as_ref(), Path::new(""));

        // Test is_root() method
        assert!(!entry_item.is_root());
        assert!(!entry_dir.is_root());
        assert!(entry_root.is_root());
    }

    #[test]
    fn test_entry_methods() {
        let root_entry = Entry {
            id: ROOT_ID,
            name: "".to_string(),
            path: Path::new("").into(),
            is_dir: true,
            config: None,
        };

        let regular_entry = Entry {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            path: Path::new("test").into(),
            is_dir: false,
            config: None,
        };

        // Test is_root() method
        assert!(root_entry.is_root());
        assert!(!regular_entry.is_root());
    }
}
