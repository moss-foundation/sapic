use anyhow::{Context, Result};
use moss_file::toml;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    graphmap::NodeTrait,
    prelude::DiGraphMap,
    visit::EdgeRef,
};
use serde::{Deserialize, Serialize};
use std::{
    cell::OnceCell,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fmt::{self, Display, Formatter},
    path::{Path, PathBuf},
    sync::Arc,
};
use uuid::Uuid;

use super::ROOT_PATH;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecificationMetadata {
    pub id: Uuid,
}

// Items specs

pub enum RequestItemSpecificationModel {}

pub enum ItemSpecificationModelInner {
    Request(RequestItemSpecificationModel),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemConfigurationModel {
    pub metadata: SpecificationMetadata,
    // pub inner: ItemSpecificationModelInner,
}

// Dirs specs

pub struct HttpDirSpecificationModel {}

pub enum RequestDirSpecificationModel {
    Http(HttpDirSpecificationModel),
}

pub enum DirSpecificationModelInner {
    Request(RequestDirSpecificationModel),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirConfigurationModel {
    pub metadata: SpecificationMetadata,
    // pub inner: DirSpecificationModelInner,
}

// Specification model

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigurationModel {
    Item(ItemConfigurationModel),
    Dir(DirConfigurationModel),
}

impl ConfigurationModel {
    pub fn id(&self) -> Uuid {
        match self {
            ConfigurationModel::Item(item) => item.metadata.id,
            ConfigurationModel::Dir(dir) => dir.metadata.id,
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnloadedEntry {
    Item {
        id: usize,
        abs_path: Arc<Path>,
        path: Arc<Path>,
    },
    Dir {
        id: usize,
        abs_path: Arc<Path>,
        path: Arc<Path>,
    },
}

impl UnloadedEntry {
    pub fn id(&self) -> usize {
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
        self.id() == 0
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
        self.id == Uuid::nil()
    }
}

#[derive(Debug, Clone)]
pub struct EntryRef {
    pub id: Uuid,
    pub idx: NodeIndex,
    pub path: Arc<Path>,
}

pub struct UnloadedEntryRef {
    pub id: usize,
    pub path: Arc<Path>,
}

pub struct Snapshot {
    entries: DiGraph<Entry, ()>,
    entries_by_id: HashMap<Uuid, NodeIndex>,
    entries_by_path: HashMap<Arc<Path>, NodeIndex>,
    unloaded_entries: DiGraphMap<usize, ()>,
    unloaded_entries_by_id: HashMap<usize, UnloadedEntry>,
    unloaded_entries_by_path: HashMap<Arc<Path>, usize>,
}

impl Snapshot {
    pub fn new(mut unloaded_entries: Vec<(UnloadedEntry, Option<usize>)>) -> Self {
        debug_assert!(
            unloaded_entries.len() > 0,
            "At least one the root entry must be present"
        );

        unloaded_entries.sort_by_key(|(e, _)| e.path().components().count());

        let entries = DiGraph::new();
        let entries_by_id = HashMap::new();
        let entries_by_path = HashMap::new();

        let mut unloaded_entries_by_id: HashMap<usize, UnloadedEntry> = HashMap::new();
        let mut unloaded_entries_by_path: HashMap<Arc<Path>, usize> = HashMap::new();
        let mut unloaded_entries_graph = DiGraphMap::new();

        for (unloaded_entry, parent_opt) in unloaded_entries.into_iter() {
            let id = unloaded_entry.id();

            unloaded_entries_graph.add_node(id);
            unloaded_entries_by_id.insert(id, unloaded_entry.clone());
            unloaded_entries_by_path.insert(Arc::clone(unloaded_entry.path()), id);

            if let Some(pid) = parent_opt {
                unloaded_entries_graph.add_edge(pid, id, ());
            }
        }
        Self {
            entries,
            entries_by_id,
            entries_by_path,
            unloaded_entries: unloaded_entries_graph,
            unloaded_entries_by_id,
            unloaded_entries_by_path,
        }
    }

    pub fn root_idx(&self) -> NodeIndex {
        self.entries_by_id
            .get(&Uuid::nil())
            .expect("Root path must be present")
            .clone()
    }

    pub fn is_loaded(&self, path: &Path) -> bool {
        self.entries_by_path.contains_key(path)
    }

    pub fn loaded_entries_count(&self) -> usize {
        self.entries_by_id.len()
    }

    pub fn unloaded_entries_count(&self) -> usize {
        self.unloaded_entries_by_id.len()
    }

    pub fn unloaded_entry_by_path(&self, path: &Path) -> Option<&UnloadedEntry> {
        self.unloaded_entries_by_path
            .get(path)
            .and_then(|id| self.unloaded_entries_by_id.get(id))
    }

    pub fn unloaded_entry_children(&self, unloaded_id: usize, depth: u8) -> Vec<UnloadedEntry> {
        if !self.unloaded_entries.contains_node(unloaded_id) {
            return vec![];
        }

        let mut current: Vec<usize> = vec![unloaded_id];
        for _ in 0..depth {
            let next: Vec<usize> = current
                .into_iter()
                .flat_map(|id| self.unloaded_entries.neighbors(id))
                .collect();
            current = next;
        }

        current
            .into_iter()
            .filter_map(|id| self.unloaded_entries_by_id.get(&id).cloned())
            .collect()
    }

    pub fn unloaded_children_by_parent_path(&self, parent_path: &Path) -> Vec<UnloadedEntry> {
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

    pub fn load_entry(&mut self, unloaded_id: usize, entry: Entry) -> Result<()> {
        let parent_path = entry.path().parent().unwrap_or(Path::new(ROOT_PATH));

        self.unloaded_entries.remove_node(unloaded_id);
        self.unloaded_entries_by_path.remove(entry.path());

        dbg!(parent_path);
        if let Some(parent_idx) = self.entries_by_path.get(parent_path) {
            let parent_id = self.entries[*parent_idx].id;
            self.create_entry(entry, Some(parent_id))?;
        } else if parent_path == Path::new(ROOT_PATH) {
            self.create_entry(entry, None)?;
        } else {
            return Err(anyhow::anyhow!(
                "Child entry cannot be unloaded before its parent"
            ));
        }

        Ok(())
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

    pub fn descendants_by_id(&self, id: Uuid) -> Option<HashSet<NodeIndex>> {
        let idx = self.entries_by_id.get(&id)?;
        self.descendants_by_idx(idx)
    }

    pub fn descendants_by_path(&self, path: &Path) -> Option<HashSet<NodeIndex>> {
        let idx = self.entries_by_path.get(path)?;
        self.descendants_by_idx(idx)
    }

    pub fn descendants_by_idx(&self, idx: &NodeIndex) -> Option<HashSet<NodeIndex>> {
        let mut to_visit = VecDeque::new();
        let mut visited = HashSet::new();

        to_visit.push_back(*idx);

        while let Some(current) = to_visit.pop_front() {
            if visited.insert(current) {
                for edge in self.entries.edges_directed(current, petgraph::Outgoing) {
                    to_visit.push_back(edge.target());
                }
            }
        }

        Some(visited)
    }

    pub fn lowest_loaded_ancestor_path(&self, path: &Path) -> Option<EntryRef> {
        for ancestor in path.ancestors() {
            if let Some(entry) = self.entry_by_path(ancestor) {
                return Some(EntryRef {
                    id: entry.id,
                    idx: self.entries_by_path[&entry.path],
                    path: Arc::clone(&entry.path),
                });
            }
        }

        None
    }

    pub fn lowest_ancestor_path(&self, path: &Path) -> UnloadedEntryRef {
        for ancestor in path.ancestors() {
            if let Some(unloaded_entry) = self.unloaded_entry_by_path(ancestor) {
                return UnloadedEntryRef {
                    id: unloaded_entry.id(),
                    path: Arc::clone(unloaded_entry.path()),
                };
            }
        }

        UnloadedEntryRef {
            id: 0,
            path: Path::new(ROOT_PATH).into(),
        }
    }
}

impl Display for Snapshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // Print root entry without prefix
        writeln!(f, "(X)")?;

        // Collect children of root, sort them by path
        let mut children: Vec<NodeIndex> = self
            .entries
            .edges_directed(self.root_idx(), petgraph::Outgoing)
            .map(|edge| edge.target())
            .collect();
        children.sort_by_key(|&child_idx| self.entries[child_idx].path.clone());

        // For each child, call recursive printer
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
