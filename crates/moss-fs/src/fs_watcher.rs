use std::{path::PathBuf, sync::Arc};

use tokio::sync::{mpsc, Mutex};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PathEventKind {
    Removed,
    Created,
    Changed,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PathEvent {
    pub path: PathBuf,
    pub kind: Option<PathEventKind>,
}

pub struct FsWatcher {
    tx: mpsc::UnboundedSender<()>,
    pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
}
