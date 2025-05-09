// use anyhow::{Context, Result, anyhow};
// use file_id::FileId;
// use futures::stream::BoxStream;
// use futures::{FutureExt, StreamExt};
// use moss_fs::{CreateOptions, FileSystem};
// use notify::{RecommendedWatcher, RecursiveMode, Watcher};
// use std::collections::{HashMap, HashSet};
// use std::mem;
// use std::path::Path;
// use std::time::Duration;
// use std::{
//     path::PathBuf,
//     sync::{Arc, atomic::AtomicUsize},
//     time::SystemTime,
// };
// use sweep_bptree::BPlusTreeMap;
// use tokio::sync::{
//     Mutex,
//     mpsc::{self, UnboundedReceiver, UnboundedSender},
//     oneshot::{self, Receiver as OneshotReceiver, Sender as OneshotSender},
//     watch::{self, Ref as WatchRef},
// };
// use tokio::task::JoinHandle;

// use crate::models::primitives::EntryId;
// use crate::models::types::UnitType;

// const ROOT_PATH: &str = "";
// const POLL_INTERVAL: Duration = Duration::from_millis(100);

// struct ScanRequest {
//     relative_paths: Vec<Arc<Path>>,
//     done: Vec<OneshotSender<()>>,
// }

// #[derive(Debug)]
// pub enum PathChange {
//     Added,
//     Removed,
//     Updated,
//     AddedOrUpdated,
//     Loaded,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum EntryKind {
//     Unit,
//     PendingDir,
//     UnloadedDir,
//     Dir,
//     File,
// }

// #[derive(Debug, Clone)]
// pub(crate) struct Entry {
//     pub id: EntryId,
//     pub path: Arc<Path>,
//     pub kind: EntryKind,
//     pub unit_type: Option<UnitType>,
//     pub mtime: Option<SystemTime>,
//     pub file_id: FileId,
// }

// type EntryRef = Arc<Entry>;

// #[derive(Debug)]
// pub(crate) enum CreatedEntry {
//     Included(EntryRef),
//     Excluded { abs_path: PathBuf },
// }

// impl Entry {
//     pub fn is_dir(&self) -> bool {
//         matches!(self.kind, EntryKind::Dir | EntryKind::PendingDir)
//     }

//     pub fn path(&self) -> &Arc<Path> {
//         &self.path
//     }
// }

// pub(crate) struct Snapshot {
//     scan_id: usize,
//     abs_path: Arc<Path>,
//     entries_by_id: BPlusTreeMap<EntryId, EntryRef>,
//     entries_by_path: BPlusTreeMap<Arc<Path>, EntryId>,
// }

// impl Clone for Snapshot {
//     fn clone(&self) -> Self {
//         let entries_by_id =
//             BPlusTreeMap::from_iter(self.entries_by_id.iter().map(|(&k, v)| (k, v.clone())));

//         let entries_by_path = BPlusTreeMap::from_iter(
//             self.entries_by_path
//                 .iter()
//                 .map(|(&ref k, v)| (k.clone(), v.clone())),
//         );

//         Self {
//             scan_id: self.scan_id.clone(),
//             abs_path: self.abs_path.clone(),
//             entries_by_id,
//             entries_by_path,
//         }
//     }
// }

// impl Snapshot {
//     pub fn count_files(&self) -> usize {
//         self.entries_by_path.len()
//     }

//     pub fn iter_entries_by_prefix<'a>(
//         &'a self,
//         prefix: &'a str,
//     ) -> impl Iterator<Item = (&'a EntryId, &'a EntryRef)> + 'a {
//         self.entries_by_path
//             .iter()
//             .skip_while(move |(p, _)| !p.starts_with(prefix))
//             .take_while(move |(p, _)| p.starts_with(prefix))
//             .filter_map(move |(_, id)| self.entries_by_id.get(id).map(|entry| (id, entry)))
//     }

//     pub fn root_entry(&self) -> Option<EntryRef> {
//         self.entry_by_path(ROOT_PATH)
//     }

//     fn ancestor_file_ids_for_path(&self, path: &Arc<Path>) -> HashSet<FileId> {
//         let mut file_ids = HashSet::new();

//         for ancestor in path.ancestors().skip(1) {
//             if let Some(entry) = self.entry_by_path(ancestor) {
//                 file_ids.insert(entry.file_id);
//             }
//         }

//         file_ids
//     }

//     fn entry_by_path(&self, path: impl AsRef<Path>) -> Option<EntryRef> {
//         let path = path.as_ref();
//         debug_assert!(path.is_relative());

//         let entry_id = self.entries_by_path.get(path)?;
//         self.entries_by_id.get(entry_id).cloned()
//     }
// }

// pub type UpdatedEntriesSet = Arc<[(PathBuf, EntryId, PathChange)]>;

// pub enum ScanState {
//     Started,
//     Updated {
//         snapshot: Snapshot,
//         changes: UpdatedEntriesSet,
//         scanning: bool,
//         barrier: Vec<OneshotSender<()>>,
//     },
//     RootUpdated {
//         new_path: Option<PathBuf>,
//     },
// }

// #[derive(Copy, Clone, PartialEq)]
// pub enum BackgroundScannerPhase {
//     InitialScan,
//     Events,
// }

// #[derive(Debug)]
// struct ScanJob {
//     abs_path: Arc<Path>,
//     path: Arc<Path>,
//     scan_queue: UnboundedSender<ScanJob>,
//     ancestor_file_ids: HashSet<FileId>,
// }

// #[derive(Clone)]
// pub struct BackgroundScannerState {
//     pub scan_id: usize,
//     pub snapshot: Snapshot,
//     pub prev_snapshot: Snapshot,
//     pub changed_paths: Vec<Arc<Path>>,
//     pub scanned_dirs: HashSet<EntryId>,
//     pub removed_entries: HashMap<FileId, EntryRef>,
//     pub paths_to_scan: HashSet<Arc<Path>>,
// }

// impl BackgroundScannerState {
//     fn remove_path(&mut self, path: &Path) {
//         let mut new_entries = BPlusTreeMap::new();
//         let mut removed_entries = Vec::new();

//         // Remove entries from entries_by_path
//         for (entry_path, entry_id) in self.snapshot.entries_by_path.iter() {
//             if entry_path.starts_with(path) {
//                 let entry_arc = self
//                     .snapshot
//                     .entries_by_id
//                     .get(entry_id)
//                     .expect(&format!("entry_id {} not found in entries_by_id", entry_id));

//                 removed_entries.push(entry_arc.clone());
//             } else {
//                 new_entries.insert(entry_path.clone(), entry_id.clone());
//             }
//         }

//         self.snapshot.entries_by_path = new_entries;

//         // Update removed_entries for possible reuse of EntryId
//         // for entries with the same file_id
//         for entry in &removed_entries {
//             self.removed_entries.entry(entry.file_id).and_modify(|old| {
//                 if entry.id > old.id {
//                     *old = entry.clone();
//                 }
//             });
//         }

//         // Remove entries from entries_by_id
//         for entry in removed_entries {
//             self.snapshot.entries_by_id.remove(&entry.id);
//         }
//     }

//     fn enqueue_scan_dir(
//         &self,
//         abs_path: Arc<Path>,
//         entry: &EntryRef,
//         scan_job_tx: &UnboundedSender<ScanJob>,
//     ) {
//         let mut ancestor_file_ids = self.snapshot.ancestor_file_ids_for_path(&entry.path);
//         if !ancestor_file_ids.contains(&entry.file_id) {
//             ancestor_file_ids.insert(entry.file_id);
//             scan_job_tx
//                 .send(ScanJob {
//                     abs_path,
//                     path: Arc::clone(&entry.path),
//                     scan_queue: scan_job_tx.clone(),
//                     ancestor_file_ids,
//                 })
//                 .ok();
//         }
//     }

//     async fn reuse_entry_id(&mut self, entry: &mut Entry) {
//         if let Some(mtime) = entry.mtime {
//             if let Some(removed_entry) = self.removed_entries.remove(&entry.file_id) {
//                 if removed_entry.mtime == Some(mtime) || removed_entry.path == entry.path {
//                     entry.id = removed_entry.id;
//                 }
//             } else if let Some(existing_entry) = self.snapshot.entry_by_path(&entry.path) {
//                 entry.id = existing_entry.id;
//             }
//         }
//     }

//     fn should_scan_directory(&self, _entry: &Entry) -> bool {
//         // TODO: Implement this
//         true
//     }

//     async fn populate_dir(
//         &mut self,
//         parent_path: &Arc<Path>,
//         entries: impl IntoIterator<Item = Entry>,
//     ) {
//         dbg!("populate_dir");
//         dbg!(parent_path);

//         let mut parent_entry = if let Some(parent_entry) = self.snapshot.entry_by_path(parent_path)
//         {
//             (*parent_entry).clone()
//         } else {
//             println!(
//                 "populating a directory {:?} that has been removed",
//                 parent_path
//             );
//             return;
//         };

//         match parent_entry.kind {
//             EntryKind::PendingDir | EntryKind::UnloadedDir => parent_entry.kind = EntryKind::Dir,
//             EntryKind::Dir => {}
//             _ => return,
//         }

//         let parent_entry_id = parent_entry.id;
//         self.scanned_dirs.insert(parent_entry_id);

//         self.snapshot
//             .entries_by_path
//             .insert(parent_entry.path.clone(), parent_entry.id);
//         self.snapshot
//             .entries_by_id
//             .insert(parent_entry.id, Arc::new(parent_entry));

//         for entry in entries {
//             self.snapshot
//                 .entries_by_path
//                 .insert(entry.path.clone(), entry.id);
//             self.snapshot
//                 .entries_by_id
//                 .insert(entry.id, Arc::new(entry));
//         }

//         if let Err(ix) = self.changed_paths.binary_search(parent_path) {
//             self.changed_paths.insert(ix, parent_path.clone());
//         }
//     }
// }

// pub struct BackgroundScanner {
//     fs: Arc<dyn FileSystem>,
//     watcher: RecommendedWatcher,
//     state: Mutex<BackgroundScannerState>,
//     next_entry_id: Arc<AtomicUsize>,
//     phase: BackgroundScannerPhase,
//     scan_req_rx: UnboundedReceiver<ScanRequest>,
//     status_updates_tx: UnboundedSender<ScanState>,
// }

// impl BackgroundScanner {
//     pub async fn run(&mut self, mut fs_events: BoxStream<'static, Vec<notify::Event>>) {
//         let (scan_job_tx, scan_job_rx) = mpsc::unbounded_channel();
//         {
//             let mut state = self.state.lock().await;
//             let root_abs_path = state.snapshot.abs_path.clone();

//             state.scan_id += 1;

//             if let Some(root_entry) = state.snapshot.root_entry() {
//                 state.enqueue_scan_dir(root_abs_path, &root_entry, &scan_job_tx);
//             }
//         }

//         drop(scan_job_tx);
//         self.scan_dirs(scan_job_rx).await;

//         self.send_status_update(false, Vec::new()).await;

//         self.phase = BackgroundScannerPhase::Events;
//         loop {
//             tokio::select! {
//                 maybe_scan_req = self.scan_req_rx.recv() => {
//                     let Some(scan_req) = maybe_scan_req else { break; };
//                     self.handle_scan_request(scan_req, false).await;
//                 }

//                 maybe_fs_events = fs_events.next() => {
//                     let Some(events) = maybe_fs_events else { break; };
//                     self.handle_fs_events(events).await;
//                 }
//             }
//         }
//     }

//     async fn send_status_update(&self, scanning: bool, barrier: Vec<OneshotSender<()>>) -> bool {
//         let mut state = self.state.lock().await;

//         // if state.changed_paths.is_empty() && scanning {
//         //     return true;
//         // }

//         let new_snapshot = state.snapshot.clone();
//         let old_snapshot = mem::replace(&mut state.prev_snapshot, new_snapshot.clone());

//         let changes = build_diff(
//             self.phase,
//             &old_snapshot,
//             &new_snapshot,
//             &state.changed_paths,
//         )
//         .await;
//         state.changed_paths.clear();

//         self.status_updates_tx
//             .send(ScanState::Updated {
//                 snapshot: new_snapshot,
//                 changes,
//                 scanning,
//                 barrier,
//             })
//             .is_ok()
//     }

//     async fn forcibly_load_paths(&mut self, paths: &[Arc<Path>]) -> bool {
//         let (scan_job_tx, mut scan_job_rx) = mpsc::unbounded_channel();
//         {
//             let mut state = self.state.lock().await;
//             let root_path = state.snapshot.abs_path.clone();
//             for path in paths {
//                 for ancestor in path.ancestors() {
//                     if let Some(entry) = state.snapshot.entry_by_path(ancestor) {
//                         if entry.kind == EntryKind::UnloadedDir {
//                             let abs_path = root_path.join(ancestor);
//                             state.enqueue_scan_dir(abs_path.into(), &entry, &scan_job_tx);
//                             state.paths_to_scan.insert(path.clone());
//                             break;
//                         }
//                     }
//                 }
//             }
//         }
//         drop(scan_job_tx);

//         while let Some(job) = scan_job_rx.recv().await {
//             if let Err(e) = self.scan_dir(job).await {
//                 println!("Error scanning directory: {}", e);
//             }
//         }

//         !mem::take(&mut self.state.lock().await.paths_to_scan).is_empty()
//     }

//     async fn reload_entries_for_paths(&self, root_abs_path: Arc<Path>, abs_paths: Vec<Arc<Path>>) {
//         let metadata = futures::future::join_all(
//             abs_paths
//                 .iter()
//                 .map(|abs_path| async move {
//                     let metadata = tokio::fs::metadata(&abs_path).await;
//                     if let Ok(metadata) = metadata {
//                         anyhow::Ok(Some((metadata, abs_path)))
//                     } else {
//                         anyhow::Ok(None)
//                     }
//                 })
//                 .collect::<Vec<_>>(),
//         )
//         .await;
//     }

//     async fn handle_scan_request(&mut self, mut scan_req: ScanRequest, scanning: bool) -> bool {
//         scan_req.relative_paths.sort_unstable();
//         self.forcibly_load_paths(&scan_req.relative_paths).await;

//         let root_path = self.state.lock().await.snapshot.abs_path.clone();

//         todo!()
//     }

//     async fn handle_fs_events(&self, events: Vec<notify::Event>) {
//         todo!()
//     }

//     async fn next_scan_request(&mut self) -> Option<ScanRequest> {
//         let mut request = self.scan_req_rx.recv().await?;

//         while let Ok(next_request) = self.scan_req_rx.try_recv() {
//             request.relative_paths.extend(next_request.relative_paths);
//             request.done.extend(next_request.done);
//         }

//         Some(request)
//     }

//     async fn scan_dirs(&mut self, mut scan_job_rx: UnboundedReceiver<ScanJob>) {
//         self.status_updates_tx.send(ScanState::Started).unwrap(); // FIXME: Handle errors

//         let mut progress = tokio::time::interval(POLL_INTERVAL);
//         loop {
//             tokio::select! {
//                 maybe_scan_req = self.next_scan_request().fuse() => {
//                     let Some(scan_req) = maybe_scan_req else { break; };
//                     self.handle_scan_request(scan_req, true).await;
//                 }

//                 _ = progress.tick() => {
//                     let _ = self.send_status_update(true, Vec::new()).await;
//                 }

//                 maybe_scan_job = scan_job_rx.recv().fuse() => {
//                     let Some(scan_job) = maybe_scan_job else { break; };
//                     if let Err(e) = self.scan_dir(scan_job).await {
//                         println!("Error scanning directory: {}", e);
//                     }
//                 }
//             }
//         }
//     }

//     async fn scan_dir(&mut self, job: ScanJob) -> Result<()> {
//         let mut new_entries = Vec::new();
//         let mut new_jobs: Vec<Option<ScanJob>> = Vec::new();

//         let mut read_dir = self.fs.read_dir(&job.abs_path).await?;
//         let mut child_paths = Vec::new();
//         while let Some(dir_entry) = read_dir.next_entry().await? {
//             child_paths.push(dir_entry);
//         }

//         for child_entry in child_paths {
//             let child_abs_path: Arc<Path> = child_entry.path().into();
//             let child_name = child_abs_path.file_name().unwrap();
//             let child_path: Arc<Path> = job.path.join(child_name).into();

//             let child_metadata = match tokio::fs::metadata(&child_abs_path).await {
//                 Ok(metadata) => metadata,
//                 Err(err) => {
//                     println!("error processing {child_abs_path:?}: {err:?}"); // TODO: log error
//                     continue;
//                 }
//             };

//             let child_entry = Entry {
//                 id: EntryId::new(&self.next_entry_id),
//                 path: child_path,
//                 kind: if child_metadata.is_dir() {
//                     EntryKind::PendingDir
//                 } else {
//                     EntryKind::File
//                 },
//                 unit_type: None,
//                 mtime: Some(child_metadata.modified().unwrap()),
//                 file_id: file_id::get_file_id(&child_abs_path)?,
//             };

//             if child_entry.is_dir() {
//                 if job.ancestor_file_ids.contains(&child_entry.file_id) {
//                     new_jobs.push(None);
//                 } else {
//                     let mut ancestor_file_ids = job.ancestor_file_ids.clone();
//                     ancestor_file_ids.insert(child_entry.file_id);

//                     new_jobs.push(Some(ScanJob {
//                         abs_path: child_abs_path.clone(),
//                         path: Arc::clone(&child_entry.path),
//                         scan_queue: job.scan_queue.clone(),
//                         ancestor_file_ids,
//                     }));
//                 }
//             }

//             new_entries.push(child_entry);
//         }

//         let mut state = self.state.lock().await;

//         let mut job_ix = 0;
//         for entry in &mut new_entries {
//             state.reuse_entry_id(entry).await;

//             if entry.is_dir() {
//                 if state.should_scan_directory(entry) {
//                     job_ix += 1;
//                 } else {
//                     println!("defer scanning directory {:?}", entry.path);
//                     entry.kind = EntryKind::UnloadedDir;
//                     new_jobs.remove(job_ix);
//                 }
//             }
//         }

//         state.populate_dir(&job.path, new_entries).await;
//         self.watcher
//             .watch(&job.abs_path, RecursiveMode::Recursive)
//             .unwrap();

//         for new_job in new_jobs.into_iter().flatten() {
//             job.scan_queue.send(new_job).unwrap();
//         }

//         Ok(())
//     }
// }

// pub(crate) struct Worktree {
//     fs: Arc<dyn FileSystem>,
//     snapshot: watch::Receiver<Snapshot>,
//     scan_requests_tx: UnboundedSender<ScanRequest>,
//     is_scanning_rx: watch::Receiver<bool>,
//     _background_tasks: Vec<JoinHandle<()>>,
// }

// impl Worktree {
//     pub async fn new(
//         fs: Arc<dyn FileSystem>,
//         root_abs_path: Arc<Path>,
//         next_entry_id: Arc<AtomicUsize>,
//     ) -> Result<Self> {
//         let (scan_states_tx, mut scan_states_rx) = mpsc::unbounded_channel();
//         let (scan_requests_tx, scan_requests_rx) = mpsc::unbounded_channel();

//         let root_entry = {
//             let metadata = tokio::fs::metadata(&root_abs_path).await?;

//             Entry {
//                 id: EntryId::new(&next_entry_id),
//                 path: Arc::from(PathBuf::from(ROOT_PATH)),
//                 kind: EntryKind::PendingDir,
//                 unit_type: None,
//                 mtime: Some(metadata.modified().unwrap()),
//                 file_id: file_id::get_file_id(&root_abs_path)?,
//             }
//         };

//         let initial_snapshot = Snapshot {
//             scan_id: 0,
//             abs_path: Arc::from(root_abs_path.clone()),
//             entries_by_path: BPlusTreeMap::from_iter([(root_entry.path().clone(), root_entry.id)]),
//             entries_by_id: BPlusTreeMap::from_iter([(root_entry.id, Arc::new(root_entry))]),
//         };

//         let (snapshot_tx, snapshot_rx) = watch::channel(initial_snapshot.clone());
//         let (is_scanning_tx, is_scanning_rx) = watch::channel(true);

//         let (fs_events, watcher) = fs.watch(&root_abs_path, Duration::from_millis(100))?;
//         let fs_clone = fs.clone();

//         let scanner_handle: JoinHandle<()> = tokio::spawn(async move {
//             let initial_state = BackgroundScannerState {
//                 scan_id: 0,
//                 snapshot: initial_snapshot.clone(),
//                 prev_snapshot: initial_snapshot,
//                 changed_paths: Vec::new(),
//                 scanned_dirs: HashSet::new(),
//                 removed_entries: HashMap::new(),
//                 paths_to_scan: HashSet::new(),
//             };

//             let mut background_scanner = BackgroundScanner {
//                 fs: fs_clone,
//                 watcher,
//                 state: Mutex::new(initial_state),
//                 next_entry_id,
//                 phase: BackgroundScannerPhase::InitialScan,
//                 scan_req_rx: scan_requests_rx,
//                 status_updates_tx: scan_states_tx,
//             };

//             background_scanner.run(fs_events).await;
//         });

//         let updater_handle: JoinHandle<()> = tokio::spawn(async move {
//             while let Some(scan_state) = scan_states_rx.recv().await {
//                 match scan_state {
//                     ScanState::Started => {
//                         let _ = is_scanning_tx.send(true);
//                     }
//                     ScanState::Updated {
//                         snapshot,
//                         changes,
//                         scanning,
//                         barrier,
//                     } => {
//                         let _ = snapshot_tx.send(snapshot);
//                         let _ = is_scanning_tx.send(scanning);

//                         dbg!(".....");
//                     }
//                     ScanState::RootUpdated { new_path } => {}
//                 }
//             }
//         });

//         Ok(Self {
//             fs,
//             snapshot: snapshot_rx,
//             scan_requests_tx,
//             is_scanning_rx,
//             _background_tasks: vec![scanner_handle, updater_handle],
//         })
//     }

//     pub async fn absolutize(&self, path: &Path) -> Result<PathBuf> {
//         if path
//             .components()
//             .any(|component| !matches!(component, std::path::Component::Normal(_)))
//         {
//             return Err(anyhow!("invalid path"));
//         }

//         let snapshot = self.snapshot().await;
//         if path.file_name().is_some() {
//             Ok(snapshot.abs_path.join(path))
//         } else {
//             Ok(snapshot.abs_path.to_path_buf())
//         }
//     }

//     pub async fn lowest_ancestor(&self, path: &Path) -> PathBuf {
//         let snapshot = self.snapshot().await;
//         let mut lowest_ancestor = None;
//         for path in path.ancestors() {
//             if snapshot.entry_by_path(path).is_some() {
//                 lowest_ancestor = Some(path.to_path_buf());
//                 break;
//             }
//         }

//         lowest_ancestor.unwrap_or_else(|| PathBuf::from(""))
//     }

//     pub async fn has_changed(&self) -> Result<bool> {
//         let changed = self.snapshot.has_changed()?;
//         Ok(changed)
//     }

//     pub async fn snapshot(&self) -> WatchRef<Snapshot> {
//         {
//             let mut is_scanning = self.is_scanning_rx.clone();
//             while *is_scanning.borrow() {
//                 is_scanning.changed().await.unwrap();
//             }
//         }

//         self.snapshot.borrow()
//     }

//     pub async fn create_entry(
//         &self,
//         path: impl Into<Arc<Path>>,
//         is_dir: bool,
//         content: Option<Vec<u8>>,
//     ) -> Result<CreatedEntry> {
//         let path = path.into();
//         let abs_path = self
//             .absolutize(&path)
//             .await
//             .context(format!("absolutizing path {path:?}"))?;

//         if is_dir {
//             self.fs.create_dir(&abs_path).await?;
//         } else {
//             self.fs
//                 .create_file_with(
//                     &abs_path,
//                     String::from_utf8(content.as_deref().unwrap_or(&[]).to_vec())?,
//                     CreateOptions {
//                         overwrite: true,
//                         ignore_if_exists: false,
//                     },
//                 )
//                 .await?;
//         }

//         let lowest_ancestor = self.lowest_ancestor(&path).await;
//         let mut refreshes = Vec::new();
//         let refresh_paths = path.strip_prefix(&lowest_ancestor).unwrap();

//         for refresh_path in refresh_paths.ancestors() {
//             if refresh_path == Path::new("") {
//                 continue;
//             }

//             let refresh_full_path = lowest_ancestor.join(refresh_path);

//             refreshes.push(self.refresh_entry(refresh_full_path.into(), None));
//         }
//         let result_refresh_entry_handle = self.refresh_entry(path.into(), None);

//         if let Err(e) = futures::future::try_join_all(refreshes).await {
//             println!("error refreshing entry: {}", e); // TODO: log error
//         }

//         Ok(result_refresh_entry_handle
//             .await??
//             .map(CreatedEntry::Included)
//             .unwrap_or_else(|| CreatedEntry::Excluded { abs_path }))
//     }

//     fn refresh_entry(
//         &self,
//         path: Arc<Path>,
//         old_path: Option<Arc<Path>>,
//     ) -> JoinHandle<Result<Option<EntryRef>>> {
//         let paths = if let Some(old_path) = old_path.as_ref() {
//             vec![old_path.clone(), path.clone()]
//         } else {
//             vec![path.clone()]
//         };

//         let refresh_rx = self.refresh_entries_for_paths(paths);
//         let snapshot_rx = self.snapshot.clone();

//         tokio::spawn(async move {
//             refresh_rx.await.ok();

//             let snapshot = snapshot_rx.borrow();
//             let entry = snapshot
//                 .entry_by_path(&path)
//                 .ok_or_else(|| anyhow!("failed to read path after update"))?;

//             Ok(Some(entry))
//         })
//     }

//     fn refresh_entries_for_paths(&self, paths: Vec<Arc<Path>>) -> OneshotReceiver<()> {
//         let (tx, rx) = oneshot::channel();
//         self.scan_requests_tx
//             .send(ScanRequest {
//                 relative_paths: paths,
//                 done: vec![tx],
//             })
//             .ok();

//         rx
//     }
// }

// async fn build_diff(
//     phase: BackgroundScannerPhase,
//     old_snapshot: &Snapshot,
//     new_snapshot: &Snapshot,
//     changed_paths: &[Arc<Path>],
// ) -> UpdatedEntriesSet {
//     // TODO: Implement this
//     dbg!("build_diff");
//     let diffs = Vec::with_capacity(changed_paths.len());

//     Arc::from(diffs)
// }

// #[cfg(test)]
// mod tests {
//     use moss_fs::RealFileSystem;

//     use super::*;

//     #[tokio::test]
//     async fn test() {
//         let fs = Arc::new(RealFileSystem::new());

//         let worktree = Worktree::new(
//             fs,
//             Arc::from(
//                 PathBuf::from(env!("CARGO_MANIFEST_DIR"))
//                     .join("tests")
//                     .join("TestCollection"),
//             ),
//             Arc::new(AtomicUsize::new(0)),
//         )
//         .await
//         .unwrap();

//         // worktree.snapshot()

//         {
//             let mut is_scanning = worktree.is_scanning_rx.clone();
//             while *is_scanning.borrow() {
//                 is_scanning.changed().await.unwrap();
//             }
//         }

//         let snapshot = worktree.snapshot().await;

//         dbg!("----");

//         for (_, entry) in snapshot.entries_by_id.iter() {
//             dbg!(entry);
//         }

//         dbg!("----");

//         for (path, _) in snapshot.entries_by_path.iter() {
//             dbg!(path);
//         }
//     }
// }
