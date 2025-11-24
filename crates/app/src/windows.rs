use async_trait::async_trait;
use chrono::Utc;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, errors::TauriResultExt};
use moss_storage2::{Storage, models::primitives::StorageScope};
use rustc_hash::FxHashMap;
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_core::context::Canceller;
use sapic_main::{MainWindow, workspace::Workspace, workspace_ops::MainWindowWorkspaceOps};
use sapic_welcome::{
    WELCOME_WINDOW_LABEL, WelcomeWindow, workspace_ops::WelcomeWindowWorkspaceOps,
};
use sapic_window::OldSapicWindow;
use sapic_window2::AppWindowApi;
use serde_json::Value as JsonValue;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::sync::RwLock;

static KEY_WORKSPACE_PREFIX: &'static str = "workspace";
static KEY_LAST_ACTIVE_WORKSPACE: &'static str = "lastActiveWorkspace";

pub fn key_workspace_last_opened_at(id: &WorkspaceId) -> String {
    format!("{KEY_WORKSPACE_PREFIX}.{}.lastOpenedAt", id.to_string())
}

pub(crate) type WindowLabel = String;

pub(crate) enum AppWindow<R: AppRuntime> {
    Welcome(WelcomeWindow<R>),
    Main(MainWindow<R>),
}

#[async_trait]
impl<R: AppRuntime> AppWindowApi for AppWindow<R> {
    async fn track_cancellation(&self, request_id: &str, canceller: Canceller) -> () {
        match self {
            AppWindow::Welcome(window) => window.track_cancellation(request_id, canceller).await,
            AppWindow::Main(window) => window.track_cancellation(request_id, canceller).await,
        }
    }

    async fn release_cancellation(&self, request_id: &str) -> () {
        match self {
            AppWindow::Welcome(window) => window.release_cancellation(request_id).await,
            AppWindow::Main(window) => window.release_cancellation(request_id).await,
        }
    }
}

impl<R: AppRuntime> Clone for AppWindow<R> {
    fn clone(&self) -> Self {
        match self {
            AppWindow::Welcome(window) => AppWindow::Welcome(window.clone()),
            AppWindow::Main(window) => AppWindow::Main(window.clone()),
        }
    }
}

impl<R: AppRuntime> AppWindow<R> {
    fn as_welcome(&self) -> Option<&WelcomeWindow<R>> {
        match self {
            AppWindow::Welcome(window) => Some(window),
            _ => None,
        }
    }

    fn as_main(&self) -> Option<&MainWindow<R>> {
        match self {
            AppWindow::Main(window) => Some(window),
            _ => None,
        }
    }
}

pub(crate) struct WindowManager<R: AppRuntime> {
    next_window_id: AtomicUsize,
    windows: RwLock<FxHashMap<WindowLabel, AppWindow<R>>>,
    workspaces: RwLock<FxHashMap<WorkspaceId, Arc<dyn Workspace>>>,
    labels_by_workspace: RwLock<FxHashMap<WorkspaceId, WindowLabel>>,
}

impl<R: AppRuntime> WindowManager<R> {
    pub fn new() -> Self {
        Self {
            next_window_id: AtomicUsize::new(0),
            windows: RwLock::new(FxHashMap::default()),
            workspaces: RwLock::new(FxHashMap::default()),
            labels_by_workspace: RwLock::new(FxHashMap::default()),
        }
    }

    pub async fn window(&self, label: &str) -> Option<AppWindow<R>> {
        self.windows.read().await.get(label).cloned()
    }

    pub async fn main_window_by_workspace_id(
        &self,
        workspace_id: &WorkspaceId,
    ) -> Option<MainWindow<R>> {
        let workspace_id = self
            .labels_by_workspace
            .read()
            .await
            .get(workspace_id)
            .cloned();

        if let Some(label) = workspace_id {
            return self.main_window(&label).await;
        }

        None
    }

    pub async fn welcome_window(&self) -> Option<WelcomeWindow<R>> {
        let window = self.windows.read().await.get(WELCOME_WINDOW_LABEL).cloned();
        if let Some(window) = window {
            // If a window was found for this label, it must be a welcome window, if not,
            // then it's a bug in the code.
            debug_assert!(window.as_welcome().is_some());

            window.as_welcome().cloned()
        } else {
            None
        }
    }

    pub async fn create_welcome_window(
        &self,
        delegate: &AppDelegate<R>,
        workspace_ops: WelcomeWindowWorkspaceOps,
    ) -> joinerror::Result<WelcomeWindow<R>> {
        if let Some(w) = self.welcome_window().await {
            return Ok(w);
        }

        let window = WelcomeWindow::new(delegate, workspace_ops).await?;
        self.windows.write().await.insert(
            WELCOME_WINDOW_LABEL.to_string(),
            AppWindow::Welcome(window.clone()),
        );

        Ok(window)
    }

    pub async fn close_welcome_window(&self) -> joinerror::Result<()> {
        let window = if let Some(window) = self.welcome_window().await {
            window
        } else {
            return Ok(());
        };

        window
            .close()
            .join_err::<()>("failed to close welcome window")?;

        self.windows.write().await.remove(WELCOME_WINDOW_LABEL);

        Ok(())
    }

    pub async fn main_window(&self, label: &str) -> Option<MainWindow<R>> {
        let window = self.windows.read().await.get(label).cloned();
        if let Some(window) = window {
            // If a window was found for this label, it must be a main window, if not,
            // then it's a bug in the code.
            debug_assert!(window.as_main().is_some());

            window.as_main().cloned()
        } else {
            None
        }
    }

    pub async fn create_main_window(
        &self,
        delegate: &AppDelegate<R>,
        old_window: OldSapicWindow<R>,
        workspace: Arc<dyn Workspace>,
        workspace_ops: MainWindowWorkspaceOps,
    ) -> joinerror::Result<MainWindow<R>> {
        let storage = <dyn Storage>::global(delegate);
        let window = MainWindow::new(
            delegate,
            self.next_window_id.fetch_add(1, Ordering::Relaxed),
            old_window,
            workspace.clone(),
            workspace_ops,
        )
        .await?;

        let label = window.label().to_string();
        self.windows
            .write()
            .await
            .insert(label.clone(), AppWindow::Main(window.clone()));

        self.workspaces
            .write()
            .await
            .insert(workspace.id(), workspace.clone());
        self.labels_by_workspace
            .write()
            .await
            .insert(workspace.id(), label.clone());

        joinerror::ResultExt::join_err::<()>(
            storage.add_workspace(workspace.id().inner()).await,
            "failed to add workspace to storage",
        )?;

        Ok(window)
    }

    pub async fn close_main_window(
        &self,
        delegate: &AppDelegate<R>,
        label: &str,
    ) -> joinerror::Result<()> {
        let window = if let Some(window) = self.main_window(label).await {
            window
        } else {
            return Ok(());
        };

        let storage = <dyn Storage>::global(delegate);
        if let Err(e) = self
            .clean_up_before_workspace_close(&window, storage.as_ref())
            .await
        {
            tracing::warn!(
                "failed to clean up before closing main window: {}",
                e.to_string()
            );
        }

        window
            .close()
            .join_err::<()>("failed to close main window")?;

        self.windows.write().await.remove(label);

        Ok(())
    }

    pub async fn swap_main_window_workspace(
        &self,
        delegate: &AppDelegate<R>,
        label: &str,
        workspace: Arc<dyn Workspace>,
        old_window: OldSapicWindow<R>,
    ) -> joinerror::Result<()> {
        // INFO:
        // We need to think through what should happen if we've already dropped everything for the old workspace but then fail to open the new one.
        // The simplest option for now would be to just crash the window or display an error saying that we couldn't open the new workspace.

        let storage = <dyn Storage>::global(delegate);
        let window = if let Some(window) = self.main_window(label).await {
            window
        } else {
            return Err(joinerror::Error::new::<()>(format!(
                "main window with label `{}` not found",
                label
            )));
        };

        if window.workspace().id() == workspace.id() {
            return Ok(());
        }

        // Dispose the current workspace
        self.clean_up_before_workspace_close(&window, storage.as_ref())
            .await?;

        // Activate the new workspace
        {
            joinerror::ResultExt::join_err::<()>(
                storage.add_workspace(workspace.id().inner()).await,
                "failed to add workspace to storage",
            )?;

            let new_workspace_id = workspace.id();

            self.workspaces
                .write()
                .await
                .insert(new_workspace_id.clone(), workspace.clone());
            self.labels_by_workspace
                .write()
                .await
                .insert(new_workspace_id.clone(), label.to_string());

            let last_opened_at = Utc::now().timestamp();
            if let Err(e) = storage
                .put_batch(
                    StorageScope::Application,
                    &[
                        (
                            KEY_LAST_ACTIVE_WORKSPACE,
                            JsonValue::String(new_workspace_id.to_string()),
                        ),
                        (
                            &key_workspace_last_opened_at(&new_workspace_id),
                            JsonValue::Number(last_opened_at.into()),
                        ),
                    ],
                )
                .await
            {
                tracing::warn!(
                    "failed to update last active workspace in storage: {}",
                    e.to_string()
                );
            }
        }

        Ok(window.swap_workspace(workspace, old_window).await?)
    }
}

impl<R: AppRuntime> WindowManager<R> {
    async fn clean_up_before_workspace_close(
        &self,
        window: &MainWindow<R>,
        storage: &dyn Storage,
    ) -> joinerror::Result<()> {
        if let Err(e) = storage
            .remove(StorageScope::Application, KEY_LAST_ACTIVE_WORKSPACE)
            .await
        {
            tracing::warn!(
                "failed to remove last active workspace from storage: {}",
                e.to_string()
            );
        }
        joinerror::ResultExt::join_err::<()>(
            storage
                .remove_workspace(window.workspace().id().inner())
                .await,
            "failed to remove workspace from storage",
        )?;

        self.workspaces
            .write()
            .await
            .remove(&window.workspace().id());
        self.labels_by_workspace
            .write()
            .await
            .remove(&window.workspace().id());

        Ok(())
    }
}
