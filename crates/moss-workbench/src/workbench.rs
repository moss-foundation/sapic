use anyhow::Result;
use arc_swap::ArcSwapOption;
use moss_activity_indicator::ActivityIndicator;
use moss_common::identifier::Identifier;
use moss_fs::FileSystem;
use moss_storage::GlobalStorage;
use moss_workspace::workspace::Workspace;
use std::{
    collections::HashMap,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::{OnceCell, RwLock};

#[derive(Debug, Clone)]
pub struct WorkspaceInfoEntry {
    pub id: Identifier,
    pub name: String,
    pub display_name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

pub(crate) type WorkspaceInfoEntryRef = Arc<WorkspaceInfoEntry>;
type WorkspaceMap = HashMap<Identifier, WorkspaceInfoEntryRef>;

pub struct ActiveWorkspace<R: TauriRuntime> {
    pub id: Identifier,
    pub inner: Workspace<R>,
}

impl<R: TauriRuntime> Deref for ActiveWorkspace<R> {
    type Target = Workspace<R>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug)]
pub struct Options {
    pub workspaces_abs_path: Arc<Path>,
    pub next_workspace_id: Arc<AtomicUsize>,
}

pub struct Workbench<R: TauriRuntime> {
    pub(super) app_handle: AppHandle<R>,
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) activity_indicator: ActivityIndicator<R>,
    pub(super) active_workspace: ArcSwapOption<ActiveWorkspace<R>>,
    pub(super) known_workspaces: OnceCell<RwLock<WorkspaceMap>>,
    pub(super) global_storage: Arc<dyn GlobalStorage>,
    pub(crate) options: Options,
}

impl<R: TauriRuntime> Workbench<R> {
    pub fn new(
        app_handle: AppHandle<R>,
        fs: Arc<dyn FileSystem>,
        global_storage: Arc<dyn GlobalStorage>,
        options: Options,
    ) -> Self {
        Self {
            app_handle: app_handle.clone(),
            fs,
            activity_indicator: ActivityIndicator::new(app_handle),
            active_workspace: ArcSwapOption::new(None),
            known_workspaces: OnceCell::new(),
            global_storage,
            options,
        }
    }

    pub fn active_workspace(&self) -> Result<Arc<ActiveWorkspace<R>>> {
        let workspace = self
            .active_workspace
            .load_full()
            .ok_or(anyhow::anyhow!("Workspace is not opened"))?;

        Ok(workspace)
    }

    pub(super) fn set_active_workspace(&self, id: Identifier, workspace: Workspace<R>) {
        self.active_workspace.store(Some(Arc::new(ActiveWorkspace {
            id,
            inner: workspace,
        })));
    }

    pub(super) async fn workspace_by_name(
        &self,
        name: &str,
    ) -> Result<Option<WorkspaceInfoEntryRef>> {
        let workspaces = self.known_workspaces().await?;
        let workspaces_lock = workspaces.read().await;

        Ok(workspaces_lock.iter().find_map(|(_, entry)| {
            if &entry.name == name {
                Some(Arc::clone(entry))
            } else {
                None
            }
        }))
    }

    pub(super) async fn known_workspaces(&self) -> Result<&RwLock<WorkspaceMap>> {
        Ok(self
            .known_workspaces
            .get_or_try_init(|| async move {
                let mut workspaces = HashMap::new();
                let workspaces_store = self.global_storage.workspaces_store();
                let restored_workspaces_info = workspaces_store.list_workspaces()?;

                for (name, _info) in restored_workspaces_info {
                    let id = Identifier::new(&self.options.next_workspace_id);
                    let abs_path: Arc<Path> = self.absolutize(&name).into();
                    let display_name = moss_fs::utils::decode_name(&name)?;

                    workspaces.insert(
                        id,
                        WorkspaceInfoEntry {
                            id,
                            name,
                            display_name,
                            abs_path,
                            last_opened_at: None, // TODO: add last opened at
                        }
                        .into(),
                    );
                }

                Ok::<_, anyhow::Error>(RwLock::new(workspaces))
            })
            .await?)
    }

    pub(super) fn absolutize(&self, workspace_name: &str) -> PathBuf {
        self.options.workspaces_abs_path.join(workspace_name)
    }
}
