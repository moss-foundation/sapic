pub mod main;
pub mod welcome;

use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_workspace::models::primitives::WorkspaceId;
use rustc_hash::FxHashMap;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::sync::RwLock;

use crate::{
    windows::{
        main::MainWindow,
        welcome::{WELCOME_WINDOW_LABEL, WelcomeWindow},
    },
    workspace::service::WorkspaceService,
};

pub(crate) mod constants {
    pub const MIN_WINDOW_WIDTH: f64 = 800.0;
    pub const MIN_WINDOW_HEIGHT: f64 = 600.0;
}

pub(crate) mod defaults {
    pub const DEFAULT_WINDOW_POSITION_X: f64 = 100.0;
    pub const DEFAULT_WINDOW_POSITION_Y: f64 = 100.0;
}

pub(crate) type WindowLabel = String;

pub(crate) enum AppWindow<R: AppRuntime> {
    Welcome(WelcomeWindow<R>),
    Main(MainWindow<R>),
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
    fn label(&self) -> &str {
        match self {
            AppWindow::Welcome(window) => window.label(),
            AppWindow::Main(window) => window.label(),
        }
    }

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

pub struct WindowManager<R: AppRuntime> {
    next_window_id: AtomicUsize,
    windows: RwLock<FxHashMap<WindowLabel, AppWindow<R>>>,
    labels_by_workspace_id: RwLock<FxHashMap<WorkspaceId, WindowLabel>>,
}

impl<R: AppRuntime> WindowManager<R> {
    pub fn new() -> Self {
        Self {
            next_window_id: AtomicUsize::new(0),
            windows: RwLock::new(FxHashMap::default()),
            labels_by_workspace_id: RwLock::new(FxHashMap::default()),
        }
    }

    pub async fn window_label_for_workspace(
        &self,
        workspace_id: &WorkspaceId,
    ) -> Option<WindowLabel> {
        self.labels_by_workspace_id
            .read()
            .await
            .get(workspace_id)
            .cloned()
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
        workspace_service: Arc<WorkspaceService>,
    ) -> joinerror::Result<WelcomeWindow<R>> {
        let window = WelcomeWindow::new(delegate, workspace_service).await?;
        self.windows.write().await.insert(
            WELCOME_WINDOW_LABEL.to_string(),
            AppWindow::Welcome(window.clone()),
        );

        Ok(window)
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
        ctx: &R::AsyncContext,
        delegate: &AppDelegate<R>,
        fs: Arc<dyn FileSystem>,
        keyring: Arc<dyn KeyringClient>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
        workspace_id: WorkspaceId,
    ) -> joinerror::Result<MainWindow<R>> {
        let window = MainWindow::new(
            ctx,
            delegate,
            fs,
            keyring,
            auth_api_client,
            self.next_window_id.fetch_add(1, Ordering::Relaxed),
            workspace_id.clone(),
        )
        .await?;

        let label = window.label().to_string();
        self.windows
            .write()
            .await
            .insert(label.clone(), AppWindow::Main(window.clone()));
        self.labels_by_workspace_id
            .write()
            .await
            .insert(workspace_id.clone(), label);

        Ok(window)
    }
}
