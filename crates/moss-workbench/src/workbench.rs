use anyhow::Result;
use derive_more::{Deref, DerefMut};
use moss_activity_indicator::ActivityIndicator;
use moss_applib::context::Context;
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

use crate::{dirs, storage::segments::WORKSPACE_SEGKEY};

#[derive(Debug, Clone)]
pub struct WorkspaceDescriptor {
    pub id: Uuid,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

type WorkspaceMap = HashMap<Uuid, Arc<WorkspaceDescriptor>>;

#[derive(Deref, DerefMut)]
pub struct ActiveWorkspace<R: TauriRuntime> {
    pub id: Uuid,
    #[deref]
    #[deref_mut]
    pub this: Workspace<R>,
    pub context: Arc<RwLock<WorkspaceContextState>>,
}

#[derive(Deref)]
pub struct WorkspaceReadGuard<'a, R: TauriRuntime> {
    guard: RwLockReadGuard<'a, Workspace<R>>,
}

#[derive(Deref, DerefMut)]
pub struct WorkspaceWriteGuard<'a, R: TauriRuntime> {
    guard: RwLockMappedWriteGuard<'a, Workspace<R>>,
}

#[derive(Debug)]
pub struct Options {
    // The absolute path of the app directory
    pub abs_path: Arc<Path>,
}

pub struct Workbench<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    pub(super) activity_indicator: ActivityIndicator<R>,
    // FIXME: This is a hack, will not be public in the future
    pub active_workspace: RwLock<Option<ActiveWorkspace<R>>>,
    pub(super) known_workspaces: OnceCell<RwLock<WorkspaceMap>>,
    pub(super) global_storage: Arc<dyn GlobalStorage>,
    pub(crate) options: Options,
}

impl<R: TauriRuntime> Workbench<R> {
    pub fn new(
        app_handle: AppHandle<R>,
        global_storage: Arc<dyn GlobalStorage>,
        options: Options,
    ) -> Self {
        Self {
            app_handle: app_handle.clone(),
            activity_indicator: ActivityIndicator::new(app_handle),
            active_workspace: RwLock::new(None),
            known_workspaces: OnceCell::new(),
            global_storage,
            options,
        }
    }

    pub async fn active_workspace_id(&self) -> Option<Uuid> {
        let guard = self.active_workspace.read().await;
        if guard.is_none() {
            return None;
        }

        let active = guard.as_ref()?;
        Some(active.id)
    }

    pub async fn active_workspace(
        &self,
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

        let context = WorkspaceContext::new(self.app_handle.clone(), context_state);
        Some((
            WorkspaceReadGuard {
                guard: workspace_guard,
            },
            context,
        ))
    }

    pub async fn active_workspace_mut(
        &self,
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

        let context = WorkspaceContext::new(self.app_handle.clone(), context_state);
        Some((
            WorkspaceWriteGuard {
                guard: workspace_guard,
            },
            context,
        ))
    }

    pub(super) async fn activate_workspace(&self, id: Uuid, workspace: Workspace<R>) {
        let mut active_workspace = self.active_workspace.write().await;
        *active_workspace = Some(ActiveWorkspace {
            id,
            this: workspace,
            context: Arc::new(RwLock::new(WorkspaceContextState::new())),
        });
    }

    pub(super) async fn deactivate_workspace(&self) {
        let mut active_workspace = self.active_workspace.write().await;
        *active_workspace = None;
    }

    pub(super) async fn workspaces<C: Context<R>>(&self, ctx: &C) -> Result<&RwLock<WorkspaceMap>> {
        Ok(self
            .known_workspaces
            .get_or_try_init(|| async move {
                let mut workspaces: WorkspaceMap = HashMap::new();

                let dir_abs_path = self.absolutize(dirs::WORKSPACES_DIR);
                if !dir_abs_path.exists() {
                    return Ok(RwLock::new(workspaces));
                }

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

                let fs = <dyn FileSystem>::global::<R, C>(ctx);

                let mut read_dir = fs.read_dir(&dir_abs_path).await?;
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

                    let summary = Workspace::<R>::summary(ctx, &entry.path()).await?;

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

    pub(super) fn absolutize(&self, path: impl AsRef<Path>) -> PathBuf {
        self.options.abs_path.join(path)
    }

    // Test only utility, not feature-flagged for easier CI setup
    pub fn __storage(&self) -> Arc<dyn GlobalStorage> {
        self.global_storage.clone()
    }
}
