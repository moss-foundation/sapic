use futures::stream::BoxStream;
use std::sync::atomic::Ordering::SeqCst;
use std::{
    path::PathBuf,
    sync::{atomic::AtomicUsize, Arc},
    time::SystemTime,
};
use sweep_bptree::BPlusTreeMap;
use tokio::sync::{mpsc, Barrier};
use tokio::sync::{mpsc::UnboundedReceiver, watch, Mutex};
use tokio::task::JoinHandle;

struct ScanRequest {
    relative_paths: Vec<PathBuf>,
    done: Barrier,
}

pub enum BackgroundScannerPhase {
    InitialScan,
}

pub struct BackgroundScannerState {}

pub struct BackgroundScanner {
    state: Mutex<BackgroundScannerState>,
    next_entry_id: Arc<AtomicUsize>,
    phase: BackgroundScannerPhase,
    scan_req_rx: UnboundedReceiver<ScanRequest>,
}

impl BackgroundScanner {
    pub async fn run(&self, fs_events_stream: BoxStream<'static, Vec<notify::Event>>) {}
}

pub enum UnitType {
    Endpoint,
    Request,
    Case,
    Schema,
    Component,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryId(usize);

impl EntryId {
    pub fn new(counter: &AtomicUsize) -> Self {
        Self(counter.fetch_add(1, SeqCst))
    }

    pub fn to_usize(&self) -> usize {
        self.0
    }
}

pub enum EntryKind {
    Unit,
    Folder,
}

pub struct Entry {
    id: EntryId,
    kind: EntryKind,
    unit_type: Option<UnitType>,
    mtime: Option<SystemTime>,
    size: u64,
}

pub struct Snapshot {
    scan_id: usize,
    entries_by_id: BPlusTreeMap<EntryId, Entry>,
    entries_by_path: BPlusTreeMap<PathBuf, EntryId>,
}

pub struct Worktree {
    snapshot: Snapshot,
    is_scanning: (watch::Sender<bool>, watch::Receiver<bool>),
    background_tasks: Vec<JoinHandle<()>>,
}

impl Worktree {
    pub fn new(next_entry_id: Arc<AtomicUsize>) -> Self {
        let (scan_req_tx, mut scan_req_rx) = mpsc::unbounded_channel();
        let snapshot = Snapshot {
            scan_id: 0,
            entries_by_id: BPlusTreeMap::new(),
            entries_by_path: BPlusTreeMap::new(),
        };

        let scanner_handle: JoinHandle<()> = tokio::spawn(async move {
            let background_scanner = BackgroundScanner {
                state: Mutex::new(BackgroundScannerState {}),
                next_entry_id,
                phase: BackgroundScannerPhase::InitialScan,
                scan_req_rx,
            };

            // loop {
            //     tokio::select! {

            //     }
            // }
        });

        let updater_handle: JoinHandle<()> = tokio::spawn(async move {
            //
        });

        Self {
            snapshot,
            is_scanning: watch::channel(true),
            background_tasks: vec![scanner_handle, updater_handle],
        }
    }
}
