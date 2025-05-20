use anyhow::{Context, Result};
use arc_swap::ArcSwapOption;
use moss_activity_indicator::ActivityIndicator;
use moss_app::service::prelude::AppService;
use moss_common::models::primitives::Identifier;
use moss_fs::FileSystem;
use moss_storage::{
    GlobalStorage, global_storage::entities::WorkspaceInfoEntity, primitives::segkey::SegmentExt,
    storage::operations::ListByPrefix,
};
use moss_workspace::Workspace;
use std::{
    collections::HashMap,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::{OnceCell, RwLock};
use uuid::Uuid;

use crate::storage::segments::WORKSPACE_SEGKEY;

pub const WORKSPACES_DIR: &str = "workspaces";

#[derive(Debug, Clone)]
pub struct WorkspaceInfoEntry {
    pub id: Identifier,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

pub(crate) type WorkspaceInfoEntryRef = Arc<WorkspaceInfoEntry>;
type WorkspaceMap = HashMap<Uuid, WorkspaceInfoEntryRef>;

pub struct ActiveWorkspace<R: TauriRuntime> {
    pub id: Uuid,
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
    // The absolute path of the app directory
    pub abs_path: Arc<Path>,
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

impl<R: tauri::Runtime> AppService for Workbench<R> {}

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

    pub fn active_workspace(&self) -> Option<Arc<ActiveWorkspace<R>>> {
        self.active_workspace.load_full()
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
        let workspaces = self.workspaces().await?;
        let workspaces_lock = workspaces.read().await;

        Ok(workspaces_lock.iter().find_map(|(_, entry)| {
            if &entry.name == name {
                Some(Arc::clone(entry))
            } else {
                None
            }
        }))
    }

    pub(super) async fn workspaces(&self) -> Result<&RwLock<WorkspaceMap>> {
        Ok(self
            .known_workspaces
            .get_or_try_init(|| async move {
                let mut workspaces: WorkspaceMap = HashMap::new();

                let dir_abs_path = self.absolutize(WORKSPACES_DIR);
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

                let mut read_dir = self.fs.read_dir(&dir_abs_path).await?;
                while let Some(entry) = read_dir.next_entry().await? {
                    if !entry.file_type().await?.is_dir() {
                        continue;
                    }

                    let id_str = entry.file_name().to_string_lossy().to_string();
                    let id =
                        Uuid::parse_str(&id_str).context("failed to parse the workspace id")?;

                    // let display_name = moss_fs::utils::decode_name(&encoded_name)?;

                    let path = PathBuf::from(WORKSPACES_DIR).join(&id_str);
                    let abs_path: Arc<Path> = self.absolutize(path).into();

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
                        WorkspaceInfoEntry {
                            id,
                            name: encoded_name,
                            display_name,
                            abs_path,
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
}
