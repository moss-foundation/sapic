use async_trait::async_trait;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, context::Canceller, errors::TauriResultExt};
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_workspace::models::primitives::WorkspaceId;
use rustc_hash::FxHashMap;
use sapic_main::MainWindow;
use sapic_welcome::{WELCOME_WINDOW_LABEL, WelcomeWindow};
use sapic_window2::AppWindowApi;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::sync::RwLock;

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
}

impl<R: AppRuntime> WindowManager<R> {
    pub fn new() -> Self {
        Self {
            next_window_id: AtomicUsize::new(0),
            windows: RwLock::new(FxHashMap::default()),
        }
    }

    pub async fn window(&self, label: &str) -> Option<AppWindow<R>> {
        self.windows.read().await.get(label).cloned()
    }

    pub async fn main_window_by_workspace_id(
        &self,
        workspace_id: &WorkspaceId,
    ) -> Option<MainWindow<R>> {
        for window in self.windows.read().await.values() {
            let AppWindow::Main(main) = window else {
                continue;
            };

            let Some(workspace) = main.inner().workspace().await else {
                continue;
            };

            if workspace.id() == *workspace_id {
                return Some(main.clone());
            }
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
    ) -> joinerror::Result<WelcomeWindow<R>> {
        let window = WelcomeWindow::new(delegate).await?;
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

        Ok(window)
    }

    pub async fn close_main_window(&self, label: &str) -> joinerror::Result<()> {
        let window = if let Some(window) = self.main_window(label).await {
            window
        } else {
            return Ok(());
        };

        window
            .close()
            .join_err::<()>("failed to close main window")?;

        self.windows.write().await.remove(label);

        Ok(())
    }
}
