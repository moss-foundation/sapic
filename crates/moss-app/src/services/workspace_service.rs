pub mod collection_provider;

use anyhow::Result;
use derive_more::{Deref, DerefMut};
use moss_applib::Service;
use moss_fs::FileSystem;
use moss_storage::{
    GlobalStorage, global_storage::entities::WorkspaceInfoEntity, primitives::segkey::SegmentExt,
    storage::operations::ListByPrefix,
};
use moss_workspace::{
    Workspace,
    context::{WorkspaceContext, WorkspaceContextState},
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::{OnceCell, RwLock, RwLockMappedWriteGuard, RwLockReadGuard, RwLockWriteGuard};
use uuid::Uuid;

use crate::{
    context::{AnyAppContext, ctxkeys},
    dirs,
    storage::segments::WORKSPACE_SEGKEY,
};

#[derive(Debug, Clone)]
pub struct WorkspaceDescriptor {
    pub id: Uuid,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

type WorkspaceMap = HashMap<Uuid, Arc<WorkspaceDescriptor>>;

#[derive(Deref)]
pub struct WorkspaceReadGuard<'a, R: TauriRuntime> {
    guard: RwLockReadGuard<'a, Workspace<R>>,
}

#[derive(Deref, DerefMut)]
pub struct WorkspaceWriteGuard<'a, R: TauriRuntime> {
    guard: RwLockMappedWriteGuard<'a, Workspace<R>>,
}

#[derive(Deref, DerefMut)]
pub struct ActiveWorkspace<R: TauriRuntime> {
    #[deref]
    #[deref_mut]
    pub this: Workspace<R>,
    pub context: Arc<RwLock<WorkspaceContextState>>,
}

pub struct WorkspaceService<R: TauriRuntime> {
    /// The absolute path to the workspaces directory
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    global_storage: Arc<dyn GlobalStorage>,
    known_workspaces: OnceCell<RwLock<WorkspaceMap>>,
    pub(crate) active_workspace: RwLock<Option<ActiveWorkspace<R>>>,
}

impl<R: TauriRuntime> Service for WorkspaceService<R> {}

impl<R: TauriRuntime> WorkspaceService<R> {
    pub fn new(
        global_storage: Arc<dyn GlobalStorage>,
        fs: Arc<dyn FileSystem>,
        abs_path: &Path,
    ) -> Self {
        debug_assert!(abs_path.is_absolute());
        let abs_path: Arc<Path> = abs_path.join(dirs::WORKSPACES_DIR).into();
        debug_assert!(abs_path.exists());

        Self {
            fs,
            global_storage,
            abs_path,
            known_workspaces: OnceCell::new(),
            active_workspace: RwLock::new(None),
        }
    }

    pub fn absolutize(&self, path: impl AsRef<Path>) -> PathBuf {
        self.abs_path.join(path)
    }

    pub(crate) async fn active_workspace(
        &self,
        app_handle: AppHandle<R>,
    ) -> Option<(WorkspaceReadGuard<'_, R>, WorkspaceContext<R>)> {
        let guard = self.active_workspace.read().await;
        if guard.is_none() {
            return None;
        }

        let context_state = guard.as_ref()?.context.clone();
        let workspace_guard = RwLockReadGuard::map(guard, |opt| match opt.as_ref() {
            Some(active) => &active.this,
            None => unreachable!("Already checked for None above"),
        });

        let context = WorkspaceContext::new(app_handle, context_state);
        Some((
            WorkspaceReadGuard {
                guard: workspace_guard,
            },
            context,
        ))
    }

    pub(crate) async fn active_workspace_mut(
        &self,
        app_handle: AppHandle<R>,
    ) -> Option<(WorkspaceWriteGuard<'_, R>, WorkspaceContext<R>)> {
        let guard = self.active_workspace.write().await;
        if guard.is_none() {
            return None;
        }

        let context_state = guard.as_ref()?.context.clone();
        let workspace_guard = RwLockWriteGuard::map(guard, |opt| match opt.as_mut() {
            Some(active) => &mut active.this,
            None => unreachable!("Already checked for None above"),
        });

        let context = WorkspaceContext::new(app_handle, context_state);
        Some((
            WorkspaceWriteGuard {
                guard: workspace_guard,
            },
            context,
        ))
    }

    pub(crate) async fn activate_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        id: Uuid,
        workspace: Workspace<R>,
    ) {
        let mut active_workspace = self.active_workspace.write().await;
        *active_workspace = Some(ActiveWorkspace {
            this: workspace,
            context: Arc::new(RwLock::new(WorkspaceContextState::new())),
        });

        let workspace_id: ctxkeys::WorkspaceId = id.into();
        ctx.set_value(workspace_id);
    }

    pub(crate) async fn deactivate_workspace<C: AnyAppContext<R>>(&self, ctx: &C) {
        let mut active_workspace = self.active_workspace.write().await;
        *active_workspace = None;

        ctx.remove_value::<ctxkeys::WorkspaceId>();
    }

    pub(crate) async fn insert_workspace(&self, workspace: WorkspaceDescriptor) -> Result<()> {
        let workspaces = self.workspaces().await?;
        let mut workspaces_lock = workspaces.write().await;
        workspaces_lock.insert(workspace.id, Arc::new(workspace));
        Ok(())
    }

    pub(crate) async fn workspaces(&self) -> Result<&RwLock<WorkspaceMap>> {
        Ok(self
            .known_workspaces
            .get_or_try_init(|| async move {
                let mut workspaces: WorkspaceMap = HashMap::new();

                let restored_items = ListByPrefix::list_by_prefix(
                    self.global_storage.item_store().as_ref(),
                    WORKSPACE_SEGKEY.as_str().expect("invalid utf-8"),
                )?;
                let filtered_restored_items = restored_items.iter().filter_map(|(k, v)| {
                    let path = k.after(&WORKSPACE_SEGKEY);
                    if let Some(path) = path {
                        Some((path, v))
                    } else {
                        None
                    }
                });

                let mut restored_entities = HashMap::with_capacity(restored_items.len());
                for (segkey, value) in filtered_restored_items {
                    let encoded_name = match String::from_utf8(segkey.as_bytes().to_owned()) {
                        Ok(name) => name,
                        Err(_) => {
                            // TODO: logging
                            println!("failed to get the workspace {:?} name", segkey);
                            continue;
                        }
                    };

                    restored_entities.insert(encoded_name, value);
                }

                let mut read_dir = self.fs.read_dir(&self.abs_path).await?;
                while let Some(entry) = read_dir.next_entry().await? {
                    if !entry.file_type().await?.is_dir() {
                        continue;
                    }

                    let id_str = entry.file_name().to_string_lossy().to_string();
                    let id = match Uuid::parse_str(&id_str) {
                        Ok(id) => id,
                        Err(_) => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", id_str);
                            continue;
                        }
                    };

                    let summary = Workspace::<R>::summary(self.fs.clone(), &entry.path()).await?;

                    let restored_entity =
                        match restored_entities.remove(&id_str).map_or(Ok(None), |v| {
                            v.deserialize::<WorkspaceInfoEntity>().map(Some)
                        }) {
                            Ok(value) => value,
                            Err(_err) => {
                                // TODO: logging
                                println!("failed to get the workspace {:?} info", id_str);
                                continue;
                            }
                        };

                    workspaces.insert(
                        id,
                        WorkspaceDescriptor {
                            id,
                            name: summary.manifest.name,
                            abs_path: entry.path().into(),
                            last_opened_at: restored_entity.map(|v| v.last_opened_at),
                        }
                        .into(),
                    );
                }

                Ok::<_, anyhow::Error>(RwLock::new(workspaces))
            })
            .await?)
    }
}
