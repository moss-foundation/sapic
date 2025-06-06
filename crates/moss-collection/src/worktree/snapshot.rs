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
    pub path: Arc<Path>,
    pub config: Option<toml::EditableInPlaceFileHandle<ConfigurationModel>>,
}

impl Entry {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn path(&self) -> &Arc<Path> {
        &self.path
    }

    pub fn is_root(&self) -> bool {
        self.id == ROOT_ID
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
        let parent_path = entry.path().parent().unwrap_or(Path::new(ROOT_PATH));

        self.unloaded_entries.remove_node(unloaded_id);
        self.unloaded_entries_by_path.remove(entry.path());

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

    pub fn entry_by_id_mut(&mut self, id: Uuid) -> Option<&mut Entry> {
        let idx = self.entries_by_id.get(&id)?;
        Some(&mut self.entries[*idx])
    }

    pub fn entry_by_path(&self, path: &Path) -> Option<&Entry> {
        let idx = self.entries_by_path.get(path)?;
        Some(&self.entries[*idx])
    }

    pub fn entry_by_path_mut(&mut self, path: &Path) -> Option<&mut Entry> {
        let idx = self.entries_by_path.get(path)?;
        Some(&mut self.entries[*idx])
    }

    pub fn remove_entry(&mut self, id: Uuid) -> Option<NodeIndex> {
        if let Some(idx) = self.entries_by_id.remove(&id) {
            let node = self.entries.remove_node(idx).unwrap();
            self.entries_by_path.remove(&node.path);

            Some(idx)
        } else {
            None
        }
    }

    pub fn lowest_ancestor_path(&self, path: &Path) -> Arc<Path> {
        for ancestor in path.ancestors() {
            if let Some(unloaded_entry) = self.known_paths.get(ancestor) {
                return unloaded_entry.clone();
            }
        }
        Path::new(ROOT_PATH).into()
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
        // Select branch symbols
        let branch = if is_last { "└── " } else { "├── " };
        // Print current node with prefix and branch
        let entry = &self.entries[idx];
        let name = entry
            .path
            .file_name()
            .unwrap_or_else(|| entry.path.as_os_str())
            .to_string_lossy();

        writeln!(f, "{}{}{}", prefix, branch, name)?;

        // Calculate new prefix for children:
        // if current is not last, draw «│   », otherwise draw «    »
        let child_prefix = if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        // Collect and sort children by name (path)
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
