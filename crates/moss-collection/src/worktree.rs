use std::sync::{atomic::AtomicUsize, Arc};

use tokio::sync::watch;

pub struct Snapshot {
    id: Arc<AtomicUsize>,
    // entries_by_path: todo!(),
    // entries_by_id: todo!(),
}

pub struct Worktree {
    next_snapshot_id: Arc<AtomicUsize>,
    is_scanning: (watch::Sender<bool>, watch::Receiver<bool>),
}
