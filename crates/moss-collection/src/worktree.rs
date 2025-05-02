use anyhow::Result;
use file_id::FileId;
use futures::stream::BoxStream;
use futures::{FutureExt, StreamExt};
use moss_fs::FileSystem;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::{HashMap, HashSet};
use std::mem;
use std::path::Path;
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;
use std::{
    path::PathBuf,
    sync::{Arc, atomic::AtomicUsize},
    time::SystemTime,
};
use sweep_bptree::BPlusTreeMap;
use tokio::sync::{Barrier, mpsc};
use tokio::sync::{
    Mutex,
    mpsc::{UnboundedReceiver, UnboundedSender},
    watch::{self, Ref as WatchRef},
};
use tokio::task::JoinHandle;

struct ScanRequest {
    relative_paths: Vec<PathBuf>,
    done: Vec<PathBuf>,
}

#[derive(Debug)]
pub enum PathChange {
    Added,
    Removed,
    Updated,
    AddedOrUpdated,
    Loaded,
}

#[derive(Debug, Clone)]
pub enum UnitType {
    Endpoint,
    Request,
    Case,
    Schema,
    Component,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    file_id: FileId,
}

impl Entry {
    pub fn is_dir(&self) -> bool {
        matches!(self.kind, EntryKind::Dir | EntryKind::PendingDir)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[derive(Clone)]
pub struct Snapshot {
    scan_id: usize,
    abs_path: PathBuf,
    entries_by_id: Arc<Mutex<BPlusTreeMap<EntryId, Entry>>>,
    entries_by_path: Arc<Mutex<BPlusTreeMap<PathBuf, EntryId>>>,
}

impl Snapshot {
    pub async fn root_entry(&self) -> Option<Entry> {
        self.entry_by_path("").await
    }

    async fn ancestor_file_ids_for_path(&self, path: &PathBuf) -> HashSet<FileId> {
        let mut file_ids = HashSet::new();

        for ancestor in path.ancestors().skip(1) {
            if let Some(entry) = self.entry_by_path(ancestor).await {
                file_ids.insert(entry.file_id);
            }
        }

        file_ids
    }

    async fn entry_by_path(&self, path: impl AsRef<Path>) -> Option<Entry> {
        let path = path.as_ref();
        debug_assert!(path.is_relative());

        let entries_by_path = self.entries_by_path.lock().await;
        let entry_id = entries_by_path.get(path)?;
        let entries_by_id = self.entries_by_id.lock().await;
        entries_by_id.get(entry_id).cloned()
    }
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
    ancestor_file_ids: HashSet<FileId>,
}

#[derive(Clone)]
pub struct BackgroundScannerState {
    pub scan_id: usize,
    pub snapshot: Snapshot,
    pub prev_snapshot: Snapshot,
    pub changed_paths: Vec<PathBuf>,
    pub scanned_dirs: HashSet<EntryId>,
    pub removed_entries: HashMap<FileId, Entry>,
}

impl BackgroundScannerState {
    async fn enqueue_scan_dir(
        &self,
        abs_path: PathBuf,
        entry: &Entry,
        scan_job_tx: &UnboundedSender<ScanJob>,
    ) {
        let mut ancestor_file_ids = self.snapshot.ancestor_file_ids_for_path(&entry.path).await;
        if !ancestor_file_ids.contains(&entry.file_id) {
            ancestor_file_ids.insert(entry.file_id);
            scan_job_tx
                .send(ScanJob {
                    abs_path,
                    path: entry.path.clone(),
                    scan_queue: scan_job_tx.clone(),
                    ancestor_file_ids,
                })
                .unwrap();
        }
    }

    async fn reuse_entry_id(&mut self, entry: &mut Entry) {
        if let Some(mtime) = entry.mtime {
            if let Some(removed_entry) = self.removed_entries.remove(&entry.file_id) {
                if removed_entry.mtime == Some(mtime) || removed_entry.path == entry.path {
                    entry.id = removed_entry.id;
                }
            } else if let Some(existing_entry) = self.snapshot.entry_by_path(&entry.path).await {
                entry.id = existing_entry.id;
            }
        }
    }

    fn should_scan_directory(&self, _entry: &Entry) -> bool {
        // TODO: Implement this
        true
    }

    async fn populate_dir(
        &mut self,
        parent_path: &PathBuf,
        entries: impl IntoIterator<Item = Entry>,
    ) {
        dbg!("populate_dir");
        dbg!(parent_path);
        let mut parent_entry =
            if let Some(parent_entry) = self.snapshot.entry_by_path(parent_path).await {
                parent_entry.clone()
            } else {
                println!(
                    "populating a directory {:?} that has been removed",
                    parent_path
                );
                return;
            };

        match parent_entry.kind {
            EntryKind::PendingDir | EntryKind::UnloadedDir => parent_entry.kind = EntryKind::Dir,
            EntryKind::Dir => {}
            _ => return,
        }

        let parent_entry_id = parent_entry.id;
        self.scanned_dirs.insert(parent_entry_id);

        let mut entries_by_id = self.snapshot.entries_by_id.lock().await;
        let mut entries_by_path = self.snapshot.entries_by_path.lock().await;

        entries_by_path.insert(parent_entry.path.clone(), parent_entry.id);
        entries_by_id.insert(parent_entry.id, parent_entry);

        for entry in entries {
            entries_by_path.insert(entry.path.clone(), entry.id);
            entries_by_id.insert(entry.id, entry);
        }

        if let Err(ix) = self.changed_paths.binary_search(parent_path) {
            self.changed_paths.insert(ix, parent_path.clone());
        }
    }
}

pub struct BackgroundScanner {
    fs: Arc<dyn FileSystem>,
    watcher: RecommendedWatcher,
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

            if let Some(root_entry) = state.snapshot.root_entry().await {
                state
                    .enqueue_scan_dir(root_abs_path, &root_entry, &scan_job_tx)
                    .await;
            }
        }

        drop(scan_job_tx);
        self.scan_dirs(scan_job_rx).await;

        self.send_status_update(false, Vec::new()).await;

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
        // if state.changed_paths.is_empty() && scanning {
        //     return true;
        // }

        let new_snapshot = state.snapshot.clone();
        let old_snapshot = mem::replace(&mut state.prev_snapshot, new_snapshot.clone());

        let changes = build_diff(
            self.phase,
            &old_snapshot,
            &new_snapshot,
            &state.changed_paths,
        )
        .await;
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

        let mut progress = tokio::time::interval(Duration::from_millis(100));
        loop {
            tokio::select! {
                // maybe_scan_req = self.next_scan_request().fuse() => {}

                _ = progress.tick() => {
                    let _ = self.send_status_update(true, Vec::new()).await;
                }

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
        let mut new_entries = Vec::new();
        let mut new_jobs: Vec<Option<ScanJob>> = Vec::new();

        let mut read_dir = self.fs.read_dir(&job.abs_path).await?;
        let mut child_paths = Vec::new();
        while let Some(dir_entry) = read_dir.next_entry().await? {
            child_paths.push(dir_entry);
        }

        for child_entry in child_paths {
            let child_abs_path = child_entry.path();
            let child_name = child_abs_path.file_name().unwrap();
            let child_path = job.path.join(child_name);

            let child_metadata = match tokio::fs::metadata(&child_abs_path).await {
                Ok(metadata) => metadata,
                Err(err) => {
                    println!("error processing {child_abs_path:?}: {err:?}"); // TODO: log error
                    continue;
                }
            };

            let child_entry = Entry {
                id: EntryId::new(&self.next_entry_id),
                path: child_path.clone(),
                kind: if child_metadata.is_dir() {
                    EntryKind::PendingDir
                } else {
                    EntryKind::File
                },
                unit_type: None,
                mtime: Some(child_metadata.modified().unwrap()),
                file_id: file_id::get_file_id(&child_abs_path)?,
            };

            if child_entry.is_dir() {
                if job.ancestor_file_ids.contains(&child_entry.file_id) {
                    new_jobs.push(None);
                } else {
                    let mut ancestor_file_ids = job.ancestor_file_ids.clone();
                    ancestor_file_ids.insert(child_entry.file_id);

                    new_jobs.push(Some(ScanJob {
                        abs_path: child_abs_path.clone(),
                        path: child_path,
                        scan_queue: job.scan_queue.clone(),
                        ancestor_file_ids,
                    }));
                }
            }

            new_entries.push(child_entry);
        }

        let mut state = self.state.lock().await;

        let mut job_ix = 0;
        for entry in &mut new_entries {
            state.reuse_entry_id(entry).await;

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

        state.populate_dir(&job.path, new_entries).await;
        self.watcher
            .watch(&job.abs_path, RecursiveMode::Recursive)
            .unwrap();

        for new_job in new_jobs.into_iter().flatten() {
            job.scan_queue.send(new_job).unwrap();
        }

        Ok(())
    }
}

pub struct Worktree {
    snapshot: watch::Receiver<Snapshot>,
    scan_requests_tx: UnboundedSender<ScanRequest>,
    is_scanning_rx: watch::Receiver<bool>,
    _background_tasks: Vec<JoinHandle<()>>,
}

impl Worktree {
    pub async fn new(
        fs: Arc<dyn FileSystem>,
        root_abs_path: PathBuf,
        next_entry_id: Arc<AtomicUsize>,
    ) -> Result<Self> {
        let (scan_states_tx, mut scan_states_rx) = mpsc::unbounded_channel();
        let (scan_requests_tx, scan_requests_rx) = mpsc::unbounded_channel();

        let root_entry = {
            let metadata = tokio::fs::metadata(&root_abs_path).await?;

            Entry {
                id: EntryId::new(&next_entry_id),
                path: PathBuf::new(),
                kind: EntryKind::PendingDir,
                unit_type: None,
                mtime: Some(metadata.modified().unwrap()),
                file_id: file_id::get_file_id(&root_abs_path)?,
            }
        };

        let initial_snapshot = Snapshot {
            scan_id: 0,
            abs_path: root_abs_path.clone(),
            entries_by_path: Arc::new(Mutex::new(BPlusTreeMap::from_iter([(
                root_entry.path().clone(),
                root_entry.id,
            )]))),
            entries_by_id: Arc::new(Mutex::new(BPlusTreeMap::from_iter([(
                root_entry.id,
                root_entry,
            )]))),
        };

        let (snapshot_tx, snapshot_rx) = watch::channel(initial_snapshot.clone());
        let (is_scanning_tx, is_scanning_rx) = watch::channel(true);

        let (fs_events, watcher) = fs.watch(root_abs_path, Duration::from_millis(100))?;
        let fs_clone = fs.clone();

        let scanner_handle: JoinHandle<()> = tokio::spawn(async move {
            let initial_state = BackgroundScannerState {
                scan_id: 0,
                snapshot: initial_snapshot.clone(),
                prev_snapshot: initial_snapshot,
                changed_paths: Vec::new(),
                scanned_dirs: HashSet::new(),
                removed_entries: HashMap::new(),
            };

            let mut background_scanner = BackgroundScanner {
                fs: fs_clone,
                watcher,
                state: Mutex::new(initial_state),
                next_entry_id,
                phase: BackgroundScannerPhase::InitialScan,
                scan_req_rx: scan_requests_rx,
                status_updates_tx: scan_states_tx,
            };

            background_scanner.run(fs_events).await;
        });

        let updater_handle: JoinHandle<()> = tokio::spawn(async move {
            while let Some(scan_state) = scan_states_rx.recv().await {
                match scan_state {
                    ScanState::Started => {
                        let _ = is_scanning_tx.send(true);
                    }
                    ScanState::Updated {
                        snapshot,
                        changes,
                        scanning,
                        barrier,
                    } => {
                        let _ = snapshot_tx.send(snapshot);
                        let _ = is_scanning_tx.send(scanning);

                        dbg!(".....");
                    }
                    ScanState::RootUpdated { new_path } => {}
                }
            }
        });

        Ok(Self {
            snapshot: snapshot_rx,
            scan_requests_tx,
            is_scanning_rx,
            _background_tasks: vec![scanner_handle, updater_handle],
        })
    }

    pub async fn has_changed(&self) -> Result<bool> {
        let changed = self.snapshot.has_changed()?;
        Ok(changed)
    }

    pub async fn snapshot(&self) -> WatchRef<Snapshot> {
        self.snapshot.borrow()
    }
}

async fn build_diff(
    phase: BackgroundScannerPhase,
    old_snapshot: &Snapshot,
    new_snapshot: &Snapshot,
    changed_paths: &[PathBuf],
) -> UpdatedEntriesSet {
    // TODO: Implement this
    dbg!("build_diff");
    let diffs = Vec::with_capacity(changed_paths.len());

    Arc::from(diffs)
}

#[cfg(test)]
mod tests {
    use moss_fs::RealFileSystem;

    use super::*;

    #[tokio::test]
    async fn test() {
        let fs = Arc::new(RealFileSystem::new());

        let worktree = Worktree::new(
            fs,
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests"),
            Arc::new(AtomicUsize::new(0)),
        )
        .await
        .unwrap();

        {
            let mut is_scanning = worktree.is_scanning_rx.clone();
            while *is_scanning.borrow() {
                is_scanning.changed().await.unwrap();
            }
        }

        let snapshot = worktree.snapshot().await;

        let entries_by_path = snapshot.entries_by_id.lock().await;
        dbg!("----");

        for (_, entry) in entries_by_path.iter() {
            dbg!(entry);
        }
    }
}
