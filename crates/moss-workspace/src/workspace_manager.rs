pub mod api;

mod error;
pub use error::*;

use anyhow::Result;
use arc_swap::ArcSwapOption;
use moss_app::service::prelude::AppService;
use moss_common::leased_slotmap::{LeasedSlotMap, ResourceKey};
use moss_fs::FileSystem;
use moss_workbench::activity_indicator::ActivityIndicator;
use std::{path::PathBuf, sync::Arc};
use tauri::AppHandle;
use tokio::sync::{OnceCell, RwLock};

use crate::{models::types::WorkspaceInfo, workspace::Workspace};

type WorkspaceInfoMap = LeasedSlotMap<ResourceKey, WorkspaceInfo>;

pub struct WorkspaceManager<R: tauri::Runtime> {
    activity_indicator: ActivityIndicator<R>,
    fs: Arc<dyn FileSystem>,
    workspaces_dir: PathBuf,
    current_workspace: ArcSwapOption<(ResourceKey, Workspace)>,
    known_workspaces: OnceCell<RwLock<WorkspaceInfoMap>>,
}

impl<R: tauri::Runtime> WorkspaceManager<R> {
    pub fn new(
        app_handle: AppHandle<R>,
        fs: Arc<dyn FileSystem>,
        workspaces_dir: PathBuf,
    ) -> Result<Self> {
        Ok(Self {
            activity_indicator: ActivityIndicator::new(app_handle),
            fs,
            workspaces_dir,
            current_workspace: ArcSwapOption::new(None),
            known_workspaces: Default::default(),
        })
    }

    async fn known_workspaces(&self) -> Result<&RwLock<WorkspaceInfoMap>> {
        Ok(self
            .known_workspaces
            .get_or_try_init(|| async move {
                dbg!(&self.workspaces_dir);
                let mut workspaces = LeasedSlotMap::new();
                let mut dir = self.fs.read_dir(&self.workspaces_dir).await?;

                while let Some(entry) = dir.next_entry().await? {
                    let file_type = entry.file_type().await?;
                    if file_type.is_file() {
                        continue;
                    }

                    let path = entry.path();
                    let file_name_str = entry.file_name().to_string_lossy().to_string();
                    workspaces.insert(WorkspaceInfo {
                        path,
                        name: file_name_str,
                    });
                }

                Ok::<_, anyhow::Error>(RwLock::new(workspaces))
            })
            .await?)
    }

    pub fn current_workspace(&self) -> Result<Arc<(ResourceKey, Workspace)>> {
        self.current_workspace
            .load()
            .clone()
            .ok_or(anyhow::anyhow!("Current workspace not set"))
    }
}

impl<R: tauri::Runtime> AppService for WorkspaceManager<R> {}
