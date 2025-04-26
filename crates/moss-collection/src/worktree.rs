use anyhow::Result;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use moss_fs::FileSystem;
use notify::{FsEventWatcher, RecursiveMode, Watcher};
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
    done: Vec<PathBuf>,
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

    fn reuse_entry_id(&mut self, entry: &mut Entry) {
        if let Some(mtime) = entry.mtime {
            if let Some(removed_entry) = self.removed_entries.remove(&entry.inode) {
                if removed_entry.mtime == Some(mtime) || removed_entry.path == entry.path {
                    entry.id = removed_entry.id;
                }
            } else if let Some(existing_entry) = self.snapshot.entry_by_path(&entry.path) {
                entry.id = existing_entry.id;
            }
        }
    }

    fn should_scan_directory(&self, entry: &Entry) -> bool {
        // TODO: Implement this
        true
    }

    async fn populate_dir(&mut self, path: &PathBuf, entries: Vec<Entry>) {
        todo!()
    }
}

pub struct BackgroundScanner {
    fs: Arc<dyn FileSystem>,
    watcher: FsEventWatcher,
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

    async fn next_scan_request(&mut self) -> Option<ScanRequest> {
        let mut request = self.scan_req_rx.recv().await?;

        while let Ok(next_request) = self.scan_req_rx.try_recv() {
            request.relative_paths.extend(next_request.relative_paths);
            request.done.extend(next_request.done);
        }

        Some(request)
    }

    async fn scan_dirs(&mut self, mut scan_job_rx: UnboundedReceiver<ScanJob>) {
        self.status_updates_tx.send(ScanState::Started).unwrap(); // FIXME: Handle errors

        loop {
            tokio::select! {
                maybe_scan_req = self.next_scan_request().fuse() => {}

                maybe_scan_job = scan_job_rx.recv().fuse() => {
                    let Some(scan_job) = maybe_scan_job else { break; };
                    if let Err(e) = self.scan_dir(scan_job).await {
                        println!("Error scanning directory: {}", e);
                    }
                }
            }
        }
    }

    async fn scan_dir(&mut self, job: ScanJob) -> Result<()> {
        let root_abs_path = self.state.lock().await.snapshot.abs_path.clone();
        let mut new_entries = Vec::new();
        let mut new_jobs: Vec<Option<ScanJob>> = Vec::new();

        let mut read_dir = self.fs.read_dir(&job.abs_path).await?;
        let mut child_paths = read_dir
            .next_entry()
            .await
            .into_iter()
            .filter_map(|dir_entry| {
                if let Some(dir_entry) = dir_entry {
                    Some(dir_entry)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for child_entry in child_paths {
            let child_abs_path = child_entry.path();
            let child_name = child_abs_path.file_name().unwrap();
            let child_path = job.path.join(child_name);

            let child_metadata = match tokio::fs::metadata(&child_abs_path).await {
                Ok(metadata) => metadata,
                Err(err) => {
                    println!("error processing {child_abs_path:?}: {err:?}"); // FIXME: log error
                    continue;
                }
            };

            let mut child_entry = Entry {
                id: EntryId::new(&self.next_entry_id),
                path: child_path.clone(),
                kind: if child_metadata.is_dir() {
                    EntryKind::PendingDir
                } else {
                    EntryKind::File
                },
                unit_type: None,
                mtime: Some(child_metadata.modified().unwrap()),
                inode: child_entry.ino(),
            };

            if child_entry.is_dir() {
                if job.ancestor_inodes.contains(&child_entry.inode) {
                    new_jobs.push(None);
                } else {
                    let mut ancestor_inodes = job.ancestor_inodes.clone();
                    ancestor_inodes.insert(child_entry.inode);

                    new_jobs.push(Some(ScanJob {
                        abs_path: child_abs_path.clone(),
                        path: child_path,
                        scan_queue: job.scan_queue.clone(),
                        ancestor_inodes,
                    }));
                }
            }

            new_entries.push(child_entry);
        }

        let mut state = self.state.lock().await;

        let mut job_ix = 0;
        for entry in &mut new_entries {
            state.reuse_entry_id(entry);
            if entry.is_dir() {
                if state.should_scan_directory(entry) {
                    job_ix += 1;
                } else {
                    println!("defer scanning directory {:?}", entry.path);
                    entry.kind = EntryKind::UnloadedDir;
                    new_jobs.remove(job_ix);
                }
            }
        }

        state.populate_dir(&job.abs_path, new_entries).await;
        self.watcher
            .watch(&job.abs_path, RecursiveMode::Recursive)
            .unwrap();

        todo!()
    }

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
    PendingDir,
    UnloadedDir,
    Dir,
    File,
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

impl Entry {
    pub fn is_dir(&self) -> bool {
        matches!(self.kind, EntryKind::Dir | EntryKind::PendingDir)
    }
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

        let (fs_events, watcher) = fs.watch(root_abs_path, Duration::from_millis(100))?;
        let empty_snapshot = snapshot.clone();
        let fs_clone = fs.clone();
        let scanner_handle: JoinHandle<()> = tokio::spawn(async move {
            let mut background_scanner = BackgroundScanner {
                fs: fs_clone,
                watcher,
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
