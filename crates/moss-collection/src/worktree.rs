use anyhow::Result;
use futures::stream::BoxStream;
use futures::StreamExt;
use moss_fs::FileSystem;
use std::collections::HashMap;
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;
use std::{
    path::PathBuf,
    sync::{atomic::AtomicUsize, Arc},
    time::SystemTime,
};
use sweep_bptree::BPlusTreeMap;
use tokio::sync::{mpsc, Barrier};
use tokio::sync::{
    mpsc::{UnboundedReceiver, UnboundedSender},
    watch, Mutex,
};
use tokio::task::JoinHandle;

struct ScanRequest {
    relative_paths: Vec<PathBuf>,
    done: Barrier,
}

pub enum PathChange {
    Added,
    Removed,
    Updated,
    AddedOrUpdated,
    Loaded,
}

pub type UpdatedEntriesSet = Arc<[(PathBuf, EntryId, PathChange)]>;

pub enum ScanState {
    Started,
    Updated {
        snapshot: Snapshot,
        changes: UpdatedEntriesSet,
        scanning: bool,
        barrier: Vec<Barrier>,
    },
    RootUpdated {
        new_path: Option<PathBuf>,
    },
}

pub enum BackgroundScannerPhase {
    InitialScan,
}

#[derive(Clone)]
pub struct BackgroundScannerState {
    pub scan_id: usize,
    pub snapshot: Snapshot,
    pub prev_snapshot: Snapshot,
    pub changed_paths: Vec<PathBuf>,
    pub removed_entries: HashMap<u64, Entry>,
}

impl Default for BackgroundScannerState {
    fn default() -> Self {
        let empty = Snapshot {
            scan_id: 0,
            entries_by_id: Arc::new(BPlusTreeMap::new()),
            entries_by_path: Arc::new(BPlusTreeMap::new()),
        };
        Self {
            scan_id: 0,
            snapshot: empty.clone(),
            prev_snapshot: empty,
            changed_paths: Vec::new(),
            removed_entries: HashMap::new(),
        }
    }
}

pub struct BackgroundScanner {
    state: Mutex<BackgroundScannerState>,
    next_entry_id: Arc<AtomicUsize>,
    phase: BackgroundScannerPhase,
    scan_req_rx: UnboundedReceiver<ScanRequest>,
    status_updates_tx: UnboundedSender<ScanState>,
}

impl BackgroundScanner {
    pub async fn run(&mut self, mut fs_events: BoxStream<'static, Vec<notify::Event>>) {
        self.do_initial_scan().await;

        loop {
            tokio::select! {
                maybe_scan_req = self.scan_req_rx.recv() => {
                    let Some(scan_req) = maybe_scan_req else { break; };
                    self.handle_scan_request(scan_req).await;
                }

                maybe_fs_events = fs_events.next() => {
                    let Some(events) = maybe_fs_events else { break; };
                    self.handle_fs_events(events).await;
                }
            }
        }
    }

    async fn send_is_scanning(&self, is_scanning: bool) {
        if is_scanning {
            let _ = self.status_updates_tx.send(ScanState::Started);
        } else {
            let state = self.state.lock().await;
            let _ = self.status_updates_tx.send(ScanState::Updated {
                snapshot: state.snapshot.clone(),
                changes: Arc::new([]),
                scanning: false,
                barrier: Vec::new(),
            });
        }
    }

    async fn do_initial_scan(&self) {
        self.send_is_scanning(true).await;
        self.scan_dirs().await;
        self.send_is_scanning(false).await;
    }

    async fn handle_scan_request(&self, scan_req: ScanRequest) {
        {
            let mut state_lock = self.state.lock().await;
            state_lock.scan_id += 1;
            state_lock
                .changed_paths
                .extend(scan_req.relative_paths.clone());
        }

        self.scan_dirs().await;
        scan_req.done.wait().await;
    }

    async fn handle_fs_events(&self, events: Vec<notify::Event>) {
        todo!()
    }

    async fn scan_dirs(&self) {
        let mut to_scan = {
            let mut st = self.state.lock().await;
            std::mem::take(&mut st.changed_paths)
        };

        for path in to_scan {
            self.scan_dir(path).await;
        }

        self.send_snapshot_update(false, Vec::new()).await;
    }

    async fn scan_dir(&self, dir: PathBuf) {}

    async fn send_snapshot_update(&self, scanning: bool, barrier: Vec<Barrier>) {
        let (snapshot, prev_snapshot, changes, scan_id) = {
            let mut st = self.state.lock().await;
            // TODO: invoke build_diff(&st.prev_snapshot, &st.snapshot)
            let changes: Vec<(PathBuf, EntryId, PathChange)> = Vec::new();
            st.prev_snapshot = st.snapshot.clone();

            (
                st.snapshot.clone(),
                st.prev_snapshot.clone(),
                Arc::<[(PathBuf, EntryId, PathChange)]>::from(changes),
                st.scan_id,
            )
        };

        let _ = self.status_updates_tx.send(ScanState::Updated {
            snapshot,
            changes,
            scanning,
            barrier,
        });
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum EntryKind {
    Unit,
    Folder,
}

#[derive(Debug, Clone)]
pub struct Entry {
    id: EntryId,
    kind: EntryKind,
    unit_type: Option<UnitType>,
    mtime: Option<SystemTime>,
    size: u64,
}

#[derive(Clone)]
pub struct Snapshot {
    scan_id: usize,
    entries_by_id: Arc<BPlusTreeMap<EntryId, Entry>>,
    entries_by_path: Arc<BPlusTreeMap<PathBuf, EntryId>>,
}

pub struct Worktree {
    snapshot: Snapshot,
    is_scanning: (watch::Sender<bool>, watch::Receiver<bool>),
    background_tasks: Vec<JoinHandle<()>>,
}

impl Worktree {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        root_abs: PathBuf,
        next_entry_id: Arc<AtomicUsize>,
    ) -> Result<Self> {
        // Rename: status_updates_tx
        let (scan_states_tx, mut scan_states_rx) = mpsc::unbounded_channel();
        let (scan_req_tx, mut scan_req_rx) = mpsc::unbounded_channel();
        let snapshot = Snapshot {
            scan_id: 0,
            entries_by_id: Arc::new(BPlusTreeMap::new()),
            entries_by_path: Arc::new(BPlusTreeMap::new()),
        };

        let (fs_events, _watcher) = fs.watch(root_abs, Duration::from_millis(100))?;
        let scanner_handle: JoinHandle<()> = tokio::spawn(async move {
            let mut background_scanner = BackgroundScanner {
                state: Mutex::new(BackgroundScannerState::default()),
                next_entry_id,
                phase: BackgroundScannerPhase::InitialScan,
                scan_req_rx,
                status_updates_tx: scan_states_tx,
            };

            background_scanner.run(fs_events).await;
        });

        let updater_handle: JoinHandle<()> = tokio::spawn(async move {
            while let Some(scan_state) = scan_states_rx.recv().await {
                match scan_state {
                    ScanState::Started => {}
                    ScanState::Updated {
                        snapshot,
                        changes,
                        scanning,
                        barrier,
                    } => {}
                    ScanState::RootUpdated { new_path } => {}
                }
            }
        });

        Ok(Self {
            snapshot,
            is_scanning: watch::channel(true),
            background_tasks: vec![scanner_handle, updater_handle],
        })
    }
}
