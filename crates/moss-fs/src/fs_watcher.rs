use anyhow::Result;
use joinerror::bail;
use notify::Watcher;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex, OnceLock},
};
use tokio::sync::mpsc;

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

impl FsWatcher {
    pub fn new(
        tx: mpsc::UnboundedSender<()>,
        pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    ) -> Self {
        Self {
            tx,
            pending_path_events,
        }
    }

    pub fn add(&self, path: &PathBuf) -> joinerror::Result<()> {
        let tx = self.tx.clone();
        let pending_path_events = self.pending_path_events.clone();
        let path_owned = path.clone();

        global(|watcher| {
            watcher.add(move |event| {
                let kind = match event.kind {
                    notify::EventKind::Create(_) => Some(PathEventKind::Created),
                    notify::EventKind::Modify(_) => Some(PathEventKind::Changed),
                    notify::EventKind::Remove(_) => Some(PathEventKind::Removed),
                    _ => None,
                };

                let path_events = event
                    .paths
                    .iter()
                    .filter_map(|event_path| {
                        event_path.starts_with(&path_owned).then(|| PathEvent {
                            path: event_path.to_path_buf(),
                            kind,
                        })
                    })
                    .collect::<Vec<_>>();

                if !path_events.is_empty() {
                    // FIXME: We can't propagate the error here since it's in a closure
                    let pending_paths = pending_path_events.lock().expect("Mutex poisoned");
                    if pending_paths.is_empty() {
                        tx.send(()).unwrap();
                    }
                }
            });
        })?;
        if let Err(e) = global(|global_watcher| {
            let mut watcher = global_watcher.watcher.lock()?;
            watcher.watch(path, notify::RecursiveMode::NonRecursive)
        })? {
            bail!("failed to register global file watcher: {}", e);
        }

        Ok(())
    }
}

pub struct GlobalFsWatcher {
    watcher: Mutex<notify::RecommendedWatcher>,
    watchers: Mutex<Vec<Box<dyn Fn(&notify::Event) + Send + Sync>>>,
}

impl GlobalFsWatcher {
    pub fn add(&self, cb: impl Fn(&notify::Event) + Send + Sync + 'static) {
        if let Ok(mut watchers) = self.watchers.lock() {
            watchers.push(Box::new(cb));
        }
    }
}

static FS_WATCHER_INSTANCE: OnceLock<anyhow::Result<GlobalFsWatcher, notify::Error>> =
    OnceLock::new();

fn handle_event(event: Result<notify::Event, notify::Error>) {
    let event = if let Ok(event) = event {
        event
    } else {
        println!("error watching file: {:?}", event);
        return;
    };

    // Filter out access events, which could lead to a weird bug on Linux after upgrading notify
    // https://github.com/zed-industries/zed/actions/runs/14085230504/job/39449448832

    if matches!(event.kind, notify::EventKind::Access(_)) {
        return;
    }

    // Clone the event to avoid lifetime issues
    let event_to_use = event.clone();

    if let Err(e) = global::<()>(|watcher| {
        if let Ok(watchers) = watcher.watchers.lock() {
            for f in watchers.iter() {
                f(&event_to_use);
            }
        }
    }) {
        println!("error watching file: {:?}", e);
    }
}

pub fn global<T>(f: impl FnOnce(&GlobalFsWatcher) -> T) -> anyhow::Result<T> {
    let result = FS_WATCHER_INSTANCE.get_or_init(|| {
        notify::recommended_watcher(handle_event).map(|file_watcher| GlobalFsWatcher {
            watcher: Mutex::new(file_watcher),
            watchers: Default::default(),
        })
    });
    match result {
        Ok(g) => Ok(f(g)),
        Err(e) => Err(anyhow::anyhow!("{}", e)),
    }
}
