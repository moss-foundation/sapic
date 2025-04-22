use std::{
    path::PathBuf,
    sync::{atomic::AtomicUsize, Arc},
    time::SystemTime,
};

use sweep_bptree::BPlusTreeMap;
use tokio::sync::watch;

pub struct Scanner {}

pub enum EntryKind {}

pub struct Entry {
    id: usize,
    kind: EntryKind,
    mtime: Option<SystemTime>,
    size: u64,
}

pub struct Snapshot {
    id: Arc<AtomicUsize>,
    entries_by_path: BPlusTreeMap<PathBuf, Entry>,
    // entries_by_id: todo!(),
}

pub struct Worktree {
    next_snapshot_id: Arc<AtomicUsize>,
    is_scanning: (watch::Sender<bool>, watch::Receiver<bool>),
}

impl Worktree {
    pub fn new() -> Self {
        todo!()
    }
}
