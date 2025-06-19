use anyhow::Result;
use derive_more::{Deref, DerefMut};
use moss_activity_indicator::ActivityIndicator;
use moss_applib::context::{AnyAppContext, Context};
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
use tokio::sync::{OnceCell, RwLock, RwLockReadGuard, RwLockWriteGuard};
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
    inner: RwLock<(Workspace<R>, WorkspaceContextState)>,
}

#[derive(Deref)]
pub struct ActiveWorkspaceReadGuard<'a, R: TauriRuntime> {
    guard: RwLockReadGuard<'a, Option<ActiveWorkspace<R>>>,
}

#[derive(Deref, DerefMut)]
pub struct ActiveWorkspaceWriteGuard<'a, R: TauriRuntime> {
    guard: RwLockWriteGuard<'a, Option<ActiveWorkspace<R>>>,
}

#[derive(Debug)]
pub struct Options {
    // The absolute path of the app directory
    pub abs_path: Arc<Path>,
}

pub struct Workbench<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    pub(super) activity_indicator: ActivityIndicator<R>,
    pub(super) active_workspace: Option<ActiveWorkspace<R>>,
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
            activity_indicator: ActivityIndicator::new(app_handle),
            active_workspace: RwLock::new(None),
            known_workspaces: OnceCell::new(),
            global_storage,
            options,
        }
    }

    pub async fn active_workspace_mut(&self) -> ActiveWorkspaceWriteGuard<'_, R> {
        ActiveWorkspaceWriteGuard {
            guard: self.active_workspace.write().await,
        }
    }

    pub async fn active_workspace(&self) -> ActiveWorkspaceReadGuard<'_, R> {
        ActiveWorkspaceReadGuard {
            guard: self.active_workspace.read().await,
        }
    }

    pub async fn workspace(&self) -> RwLockReadGuard<'_, Workspace<R>> {
        let r = self.active_workspace.as_ref().unwrap();
        r.inner.read().await
    }

    pub(super) async fn activate_workspace<C: AnyAppContext<R>>(
        &self,
        ctx: &C,
        id: Uuid,
        workspace: Workspace<R>,
    ) {
        let mut active_workspace = self.active_workspace.write().await;
        *active_workspace = Some(ActiveWorkspace {
            id,
            inner: workspace,
            context: WorkspaceContext::new(ctx.app_handle()),
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
