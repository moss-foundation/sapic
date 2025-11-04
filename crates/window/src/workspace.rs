use chrono::Utc;
use joinerror::{Error, OptionExt, ResultExt};
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_fs::{FileSystem, FsResultExt, RemoveOptions};
use moss_logging::session;
use moss_storage2::Storage;
use moss_user::profile::Profile;
use moss_workspace::{
    builder::{CreateWorkspaceParams, LoadWorkspaceParams, WorkspaceBuilder},
    workspace::{WorkspaceModifyParams, WorkspaceSummary},
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::sync::RwLock;

use crate::{
    ActiveWorkspace, dirs,
    models::primitives::WorkspaceId,
    storage::{
        StorageService,
        segments::{SEGKEY_WORKSPACE, segkey_last_opened_at, segkey_workspace},
    },
};

pub(crate) struct WorkspaceItemCreateParams {
    pub name: String,
}

pub(crate) struct WorkspaceItemUpdateParams {
    pub name: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct WorkspaceItem {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

#[derive(Debug)]
pub(crate) struct WorkspaceDetails {
    pub id: WorkspaceId,
    pub name: String,
    pub abs_path: Arc<Path>,
    pub last_opened_at: Option<i64>,
}

type WorkspaceMap = HashMap<WorkspaceId, WorkspaceItem>;

#[derive(Default)]
struct ServiceState<R: AppRuntime> {
    known_workspaces: WorkspaceMap,
    active_workspace: Option<Arc<ActiveWorkspace<R>>>,
}

pub struct WorkspaceService<R: AppRuntime> {
    /// The absolute path to the workspaces directory
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    storage: Arc<StorageService<R>>,
    state: Arc<RwLock<ServiceState<R>>>,
}

impl<R: AppRuntime> WorkspaceService<R> {
    pub async fn new(
        ctx: &R::AsyncContext,
        storage_service: Arc<StorageService<R>>,
        fs: Arc<dyn FileSystem>,
        abs_path: &Path,
    ) -> joinerror::Result<Self> {
        debug_assert!(abs_path.is_absolute());
        let abs_path: Arc<Path> = abs_path.join(dirs::WORKSPACES_DIR).into();
        debug_assert!(abs_path.exists());

        let known_workspaces =
            restore_known_workspaces::<R>(ctx, &abs_path, &fs, &storage_service).await?;

        Ok(Self {
            fs,
            storage: storage_service,
            abs_path,
            state: Arc::new(RwLock::new(ServiceState {
                known_workspaces,
                active_workspace: None,
            })),
        })
    }

    pub fn absolutize(&self, path: impl AsRef<Path>) -> PathBuf {
        self.abs_path.join(path)
    }

    pub(crate) async fn workspace_details(&self, id: &WorkspaceId) -> Option<WorkspaceDetails> {
        let state_lock = self.state.read().await;
        state_lock
            .known_workspaces
            .get(id)
            .map(|item| WorkspaceDetails {
                id: item.id.clone(),
                name: item.name.clone(),
                abs_path: item.abs_path.clone(),
                last_opened_at: item.last_opened_at,
            })
    }

    pub(crate) async fn list_workspaces(&self) -> joinerror::Result<Vec<WorkspaceDetails>> {
        let state_lock = self.state.read().await;
        let workspaces = state_lock
            .known_workspaces
            .values()
            .map(|item| WorkspaceDetails {
                id: item.id.clone(),
                name: item.name.clone(),
                abs_path: item.abs_path.clone(),
                last_opened_at: item.last_opened_at,
            })
            .collect();
        Ok(workspaces)
    }

    pub(crate) async fn update_workspace(
        &self,
        params: WorkspaceItemUpdateParams,
    ) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;
        let workspace = state_lock
            .active_workspace
            .as_ref()
            .ok_or_join_err::<()>("no active workspace")?;

        let mut descriptor = state_lock
            .known_workspaces
            .get(&workspace.id)
            .ok_or_join_err_with::<()>(|| format!("workspace `{}` not found", workspace.id))?
            .clone();

        workspace
            .modify(WorkspaceModifyParams {
                name: params.name.clone(),
            })
            .await?;

        if let Some(new_name) = params.name {
            descriptor.name = new_name;
        }

        state_lock
            .known_workspaces
            .insert(descriptor.id.clone(), descriptor);

        Ok(())
    }

    pub(crate) async fn delete_workspace(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        id: &WorkspaceId,
    ) -> joinerror::Result<()> {
        let (active_workspace_id, item) = {
            let state_lock = self.state.read().await;

            let active_workspace_id = state_lock.active_workspace.as_ref().map(|a| a.id.clone());
            let item = state_lock.known_workspaces.get(&id).cloned();

            (active_workspace_id, item)
        };

        let item = item.ok_or_join_err_with::<()>(|| format!("workspace `{}` not found", id))?;
        if active_workspace_id == Some(item.id.clone()) {
            self.deactivate_workspace(ctx, app_delegate).await?
        }

        if item.abs_path.exists() {
            self.fs
                .remove_dir(
                    &item.abs_path,
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .join_err_with::<()>(|| {
                    format!(
                        "failed to delete workspace `{}` directory",
                        item.id.as_str()
                    )
                })?;
        }

        {
            let mut state_lock = self.state.write().await;
            state_lock.known_workspaces.remove(&id);
        }

        {
            // Try to remove database entries for the workspace (log error if db operation fails)
            if let Err(e) = self
                .storage
                .remove_all_by_prefix(ctx, &segkey_workspace(&id).to_string())
                .await
            {
                session::warn!(format!(
                    "failed to remove database entries for workspace `{}`: {}",
                    id,
                    e.to_string()
                ));
            }
        }

        Ok(())
    }

    pub(crate) async fn create_workspace(
        &self,
        id: &WorkspaceId,
        params: WorkspaceItemCreateParams,
    ) -> joinerror::Result<WorkspaceDetails> {
        let mut state_lock = self.state.write().await;

        let id_str = id.to_string();

        let abs_path: Arc<Path> = self.absolutize(&id_str).into();

        let mut rb = self.fs.start_rollback().await?;

        self.fs
            .create_dir_with_rollback(&mut rb, abs_path.as_ref())
            .await
            .join_err::<()>("failed to create workspace directory")?;

        if let Err(e) = WorkspaceBuilder::<R>::initialize(
            self.fs.clone(),
            CreateWorkspaceParams {
                name: params.name.clone(),
                abs_path: abs_path.clone(),
            },
        )
        .await
        {
            let _ = rb.rollback().await.map_err(|e| {
                session::warn!(format!("failed to rollback fs changes: {}", e.to_string()))
            });
            return Err(e.join::<()>("failed to initialize workspace"));
        }

        state_lock.known_workspaces.insert(
            id.clone(),
            WorkspaceItem {
                id: id.clone(),
                name: params.name.clone(),
                last_opened_at: None,
                abs_path: Arc::clone(&abs_path),
            },
        );

        Ok(WorkspaceDetails {
            id: id.to_owned(),
            name: params.name,
            abs_path: Arc::clone(&abs_path),
            last_opened_at: None,
        })
    }

    pub(crate) async fn workspace(&self) -> Option<Arc<ActiveWorkspace<R>>> {
        let state_lock = self.state.read().await;
        if state_lock.active_workspace.is_none() {
            return None;
        }

        Some(state_lock.active_workspace.as_ref()?.clone())
    }

    pub(crate) async fn activate_workspace(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        id: &WorkspaceId,
        active_profile: Arc<Profile<R>>,
    ) -> joinerror::Result<WorkspaceDetails> {
        let (name, already_active) = {
            let state_lock = self.state.read().await;
            let item = state_lock
                .known_workspaces
                .get(&id)
                .ok_or_join_err_with::<()>(|| format!("workspace `{}` not found", id))?;

            let already_active = state_lock
                .active_workspace
                .as_ref()
                .map(|active| active.id == *id)
                .unwrap_or(false);

            (item.name.clone(), already_active)
        };

        if already_active {
            return Err(Error::new::<()>(format!(
                "workspace `{}` is already loaded",
                id
            )));
        }

        {
            let mut state_lock = self.state.write().await;
            if let Some(previous_workspace) = state_lock.active_workspace.take() {
                previous_workspace.dispose().await;
                drop(previous_workspace);
            }
        }

        let last_opened_at = Utc::now().timestamp();
        let abs_path: Arc<Path> = self.absolutize(&id.to_string()).into();
        let workspace = WorkspaceBuilder::new(self.fs.clone(), active_profile)
            .load(
                ctx,
                app_delegate,
                LoadWorkspaceParams {
                    abs_path: abs_path.clone(),
                },
            )
            .await
            .join_err_with::<()>(|| {
                format!("failed to load the workspace, {}", abs_path.display())
            })?;

        {
            let mut state_lock = self.state.write().await;
            let item = state_lock
                .known_workspaces
                .get_mut(&id)
                .expect("Workspace should still exist"); // We already checked it exists above

            item.last_opened_at = Some(last_opened_at);
            state_lock.active_workspace = Some(
                ActiveWorkspace {
                    id: id.clone(),
                    handle: workspace,
                }
                .into(),
            );
        }

        if let Err(e) = <dyn Storage>::global(app_delegate)
            .add_workspace(id.inner())
            .await
        {
            return Err(e.join::<()>("failed to add workspace to the storage"));
        }

        // We don't want database error to fail the operation
        match self.storage.begin_write_with_context(ctx).await {
            Ok(mut txn) => {
                if let Err(e) = self
                    .storage
                    .put_last_active_workspace_txn(ctx, &mut txn, &id)
                    .await
                {
                    session::error!(format!(
                        "failed to put last active workspace to the database: {}",
                        e.to_string()
                    ));
                }

                if let Err(e) = self
                    .storage
                    .put_last_opened_at_txn(ctx, &mut txn, &id, last_opened_at)
                    .await
                {
                    session::error!(format!(
                        "failed to put workspace last opened at to the database: {}",
                        e.to_string()
                    ))
                }

                if let Err(e) = txn.commit() {
                    session::error!(format!("failed to commit transaction: {}", e.to_string()))
                }
            }
            Err(e) => {
                session::error!(format!(
                    "failed to start write transaction: {}",
                    e.to_string()
                ))
            }
        }

        Ok(WorkspaceDetails {
            id: id.to_owned(),
            name,
            abs_path: Arc::clone(&abs_path),
            last_opened_at: Some(last_opened_at),
        })
    }

    pub(crate) async fn deactivate_workspace(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
    ) -> joinerror::Result<()> {
        let mut state_lock = self.state.write().await;
        let current_workspace = state_lock.active_workspace.take();
        if let Some(workspace) = current_workspace {
            workspace.dispose().await;

            <dyn Storage>::global(app_delegate)
                .remove_workspace(workspace.id.inner())
                .await
                .join_err::<()>("failed to remove workspace from the storage")?;
        }

        if let Err(e) = self.storage.remove_last_active_workspace(ctx).await {
            session::error!(format!(
                "failed to remove last active workspace from database: {}",
                e.to_string()
            ));
        }

        Ok(())
    }
}

async fn restore_known_workspaces<R: AppRuntime>(
    ctx: &R::AsyncContext,
    abs_path: &Path,
    fs: &Arc<dyn FileSystem>,
    storage_service: &Arc<StorageService<R>>,
) -> joinerror::Result<WorkspaceMap> {
    let mut workspaces = HashMap::new();

    // Log the error when we failed to restore workspace cache
    let restored_items = storage_service
        .list_all_by_prefix(ctx, SEGKEY_WORKSPACE.as_str().expect("invalid utf-8"))
        .await
        .unwrap_or_else(|e| {
            session::error!(format!(
                "failed to restore workspace cache: {}",
                e.to_string()
            ));
            HashMap::new()
        });

    let mut read_dir = fs.read_dir(&abs_path).await?;

    while let Some(entry) = read_dir.next_entry().await? {
        if !entry.file_type().await?.is_dir() {
            continue;
        }

        let id_str = entry.file_name().to_string_lossy().to_string();
        let id: WorkspaceId = id_str.into();

        // Log the error and skip when encountering a workspace with invalid manifest
        let summary = match WorkspaceSummary::new(fs, &entry.path()).await {
            Ok(summary) => summary,
            Err(e) => {
                session::error!(format!(
                    "failed to parse workspace `{}` manifest: {}",
                    id.as_str(),
                    e.to_string()
                ));
                continue;
            }
        };

        let filtered_items = restored_items
            .iter()
            .filter(|(key, _)| key.starts_with(&segkey_workspace(&id)))
            .collect::<HashMap<_, _>>();

        // Leave `last_opened_at` empty if we failed to fetch it from the database

        let last_opened_at = filtered_items
            .get(&segkey_last_opened_at(&id))
            .map(|v| v.deserialize::<i64>())
            .transpose()
            .unwrap_or_else(|e| {
                session::error!(format!(
                    "failed to get last_opened_at time from the database for workspace `{}`: {}",
                    id.as_str(),
                    e.to_string()
                ));
                None
            });

        workspaces.insert(
            id.clone(),
            WorkspaceItem {
                id,
                name: summary.name,
                abs_path: entry.path().into(),
                last_opened_at,
            }
            .into(),
        );
    }

    Ok(workspaces)
}
