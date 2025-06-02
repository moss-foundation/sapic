use anyhow::Result;
use moss_file::kdl::KdlFileHandle;
use petgraph::{
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    path::Path,
    sync::Arc,
};
use uuid::Uuid;

use kdl::KdlDocument;

#[derive(Debug, Clone)]
pub struct SpecificationMetadata {
    pub id: Uuid,
}

impl From<KdlDocument> for SpecificationMetadata {
    fn from(document: KdlDocument) -> Self {
        todo!()
    }
}

// Items specs

pub enum RequestItemSpecificationModel {}

pub enum ItemSpecificationModelInner {
    Request(RequestItemSpecificationModel),
}

#[derive(Debug, Clone)]
pub struct ItemSpecificationModel {
    pub metadata: SpecificationMetadata,
    // pub inner: ItemSpecificationModelInner,
}

impl From<KdlDocument> for ItemSpecificationModel {
    fn from(document: KdlDocument) -> Self {
        todo!()
    }
}

// Dirs specs

pub struct HttpDirSpecificationModel {}

pub enum RequestDirSpecificationModel {
    Http(HttpDirSpecificationModel),
}

pub enum DirSpecificationModelInner {
    Request(RequestDirSpecificationModel),
}

#[derive(Debug, Clone)]
pub struct DirSpecificationModel {
    pub metadata: SpecificationMetadata,
    // pub inner: DirSpecificationModelInner,
}

impl From<KdlDocument> for DirSpecificationModel {
    fn from(document: KdlDocument) -> Self {
        todo!()
    }
}

// Specification model

#[derive(Debug, Clone)]
pub enum SpecificationModel {
    Item(ItemSpecificationModel),
    Dir(DirSpecificationModel),
}

impl SpecificationModel {
    pub fn id(&self) -> Uuid {
        match self {
            SpecificationModel::Item(item) => item.metadata.id,
            SpecificationModel::Dir(dir) => dir.metadata.id,
        }
    }
}

impl Into<KdlDocument> for SpecificationModel {
    fn into(self) -> KdlDocument {
        todo!()
    }
}

impl From<KdlDocument> for SpecificationModel {
    fn from(document: KdlDocument) -> Self {
        todo!()
    }
}

pub struct Entry {
    pub id: Uuid,
    pub path: Arc<Path>,
    // pub config: KdlFileHandle<SpecificationModel>,
}

pub struct Snapshot {
    entries: DiGraph<Entry, ()>,
    entries_by_id: HashMap<Uuid, NodeIndex>,
    entries_by_path: HashMap<Arc<Path>, NodeIndex>,
}

impl Snapshot {
    pub fn new() -> Self {
        Self {
            entries: DiGraph::new(),
            entries_by_id: HashMap::new(),
            entries_by_path: HashMap::new(),
        }
    }

    pub fn create_entry(&mut self, entry: Entry, parent_id: Option<Uuid>) -> Result<NodeIndex> {
        let id = entry.id;
        let path = Arc::clone(&entry.path);

        let idx = self.entries.try_add_node(entry)?;
        self.entries_by_id.insert(id, idx);
        self.entries_by_path.insert(path, idx);

        if let Some(parent_id) = parent_id {
            if let Some(&parent_idx) = self.entries_by_id.get(&parent_id) {
                self.entries.try_add_edge(parent_idx, idx, ())?;
            }
        }

        Ok(idx)
    }

    pub fn entry_by_id(&self, id: Uuid) -> Option<&Entry> {
        let idx = self.entries_by_id.get(&id)?;
        Some(&self.entries[*idx])
    }

    pub fn entry_by_path(&self, path: &Path) -> Option<&Entry> {
        let idx = self.entries_by_path.get(path)?;
        Some(&self.entries[*idx])
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
}
