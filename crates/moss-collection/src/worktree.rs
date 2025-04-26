use anyhow::Result;
use futures::stream::BoxStream;
use futures::StreamExt;
use moss_fs::FileSystem;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::path::Path;
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

#[derive(Copy, Clone, PartialEq)]
pub enum BackgroundScannerPhase {
    InitialScan,
    Events,
}

#[derive(Debug)]
struct ScanJob {
    abs_path: PathBuf,
    path: PathBuf,
    scan_queue: UnboundedSender<ScanJob>,
    ancestor_inodes: HashSet<u64>,
}

#[derive(Clone)]
pub struct BackgroundScannerState {
    pub scan_id: usize,
    pub snapshot: Snapshot,
    pub prev_snapshot: Snapshot,
    pub changed_paths: Vec<PathBuf>,
    pub removed_entries: HashMap<u64, Entry>,
}

impl BackgroundScannerState {
    fn enqueue_scan_dir(
        &self,
        abs_path: PathBuf,
        entry: &Entry,
        scan_job_tx: &UnboundedSender<ScanJob>,
    ) {
        let mut ancestor_inodes = self.snapshot.ancestor_inodes_for_path(&entry.path);
        if !ancestor_inodes.contains(&entry.inode) {
            ancestor_inodes.insert(entry.inode);
            scan_job_tx
                .send(ScanJob {
                    abs_path,
                    path: entry.path.clone(),
                    scan_queue: scan_job_tx.clone(),
                    ancestor_inodes,
                })
                .unwrap();
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
        let (scan_job_tx, scan_job_rx) = mpsc::unbounded_channel();
        {
            let mut state = self.state.lock().await;
            let root_abs_path = state.snapshot.abs_path.clone();

            state.scan_id += 1;

            if let Some(root_entry) = state.snapshot.root_entry() {
                state.enqueue_scan_dir(root_abs_path, root_entry, &scan_job_tx);
            }
        }

        drop(scan_job_tx);
        self.scan_dirs(scan_job_rx).await;

        self.send_status_update(false, Vec::new()).await;

        // Start listening for events
        self.phase = BackgroundScannerPhase::Events;

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

    async fn send_status_update(&self, scanning: bool, barrier: Vec<Barrier>) -> bool {
        let mut state = self.state.lock().await;
        if state.changed_paths.is_empty() && scanning {
            return true;
        }

        let new_snapshot = state.snapshot.clone();
        let old_snapshot = mem::replace(&mut state.prev_snapshot, new_snapshot.clone());

        let changes = build_diff(
            self.phase,
            &old_snapshot,
            &new_snapshot,
            &state.changed_paths,
        );
        state.changed_paths.clear();

        self.status_updates_tx
            .send(ScanState::Updated {
                snapshot: new_snapshot,
                changes,
                scanning,
                barrier,
            })
            .is_ok()
    }

    async fn do_initial_scan(&self) {
        todo!()
    }

    async fn handle_scan_request(&self, scan_req: ScanRequest) {
        todo!()
    }

    async fn handle_fs_events(&self, events: Vec<notify::Event>) {
        todo!()
    }

    async fn scan_dirs(&self, scan_job_rx: UnboundedReceiver<ScanJob>) {
        todo!()
    }

    async fn scan_dir(&self, dir: PathBuf) {}

    async fn send_snapshot_update(&self, scanning: bool, barrier: Vec<Barrier>) {
        todo!()
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
    path: PathBuf,
    kind: EntryKind,
    unit_type: Option<UnitType>,
    mtime: Option<SystemTime>,
    inode: u64,
    // size: u64,
}

#[derive(Clone)]
pub struct Snapshot {
    scan_id: usize,
    abs_path: PathBuf,
    entries_by_id: Arc<BPlusTreeMap<EntryId, Entry>>,
    entries_by_path: Arc<BPlusTreeMap<PathBuf, EntryId>>,
}

impl Snapshot {
    pub fn root_entry(&self) -> Option<&Entry> {
        self.entry_by_path("")
    }

    fn ancestor_inodes_for_path(&self, path: &PathBuf) -> HashSet<u64> {
        let mut inodes = HashSet::new();

        for ancestor in path.ancestors().skip(1) {
            if let Some(entry) = self.entry_by_path(ancestor) {
                inodes.insert(entry.inode);
            }
        }

        inodes
    }

    fn entry_by_path(&self, path: impl AsRef<Path>) -> Option<&Entry> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        self.entries_by_path
            .get(path)
            .and_then(|id| self.entries_by_id.get(id))
    }
}

pub struct Worktree {
    snapshot: Snapshot,
    is_scanning: (watch::Sender<bool>, watch::Receiver<bool>),
    background_tasks: Vec<JoinHandle<()>>,
}

impl Worktree {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        root_abs_path: PathBuf,
        next_entry_id: Arc<AtomicUsize>,
    ) -> Result<Self> {
        // Rename: status_updates_tx
        let (scan_states_tx, mut scan_states_rx) = mpsc::unbounded_channel();
        let (scan_req_tx, mut scan_req_rx) = mpsc::unbounded_channel();
        let snapshot = Snapshot {
            scan_id: 0,
            abs_path: root_abs_path.clone(),
            entries_by_id: Arc::new(BPlusTreeMap::new()),
            entries_by_path: Arc::new(BPlusTreeMap::new()),
        };

        let (fs_events, _watcher) = fs.watch(root_abs_path, Duration::from_millis(100))?;
        let empty_snapshot = snapshot.clone();
        let scanner_handle: JoinHandle<()> = tokio::spawn(async move {
            let mut background_scanner = BackgroundScanner {
                state: Mutex::new(BackgroundScannerState {
                    scan_id: 0,
                    snapshot: empty_snapshot.clone(),
                    prev_snapshot: empty_snapshot,
                    changed_paths: Vec::new(),
                    removed_entries: HashMap::new(),
                }),
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

fn build_diff(
    phase: BackgroundScannerPhase,
    old_snapshot: &Snapshot,
    new_snapshot: &Snapshot,
    changed_paths: &[PathBuf],
) -> UpdatedEntriesSet {
    let changes: Vec<(PathBuf, EntryId, PathChange)> = Vec::new();
    Arc::from(changes)
}
