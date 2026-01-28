pub mod environment;
pub mod environment_ops;
pub mod operations;
pub mod workspace;
pub mod workspace_ops;

use arc_swap::ArcSwap;
use async_trait::async_trait;
use derive_more::Deref;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, TauriResultExt};
use sapic_core::context::Canceller;
use sapic_window2::{
    AppWindowApi, WindowHandle,
    constants::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    environment_ops::MainWindowEnvironmentOps, workspace::Workspace,
    workspace_ops::MainWindowWorkspaceOps,
};

const MAIN_WINDOW_LABEL_PREFIX: &str = "main_";
const MAIN_WINDOW_ENTRY_POINT: &str = "workspace.html";

/// Newtype wrapper for Workspace to use with ArcSwap.
/// This allows atomic lock-free swapping of workspace instances.
#[derive(Deref, Clone)]
pub struct WorkspaceHandle(Arc<dyn Workspace>);

impl WorkspaceHandle {
    pub fn new(workspace: Arc<dyn Workspace>) -> Self {
        Self(workspace)
    }

    pub fn get(&self) -> Arc<dyn Workspace> {
        self.0.clone()
    }

    pub fn into_inner(self) -> Arc<dyn Workspace> {
        self.0
    }
}

/// Main window controller. This window is used to display a workspace.
#[derive(Deref)]
pub struct MainWindow<R: AppRuntime> {
    #[deref]
    pub handle: WindowHandle<R::EventLoop>,

    pub workspace: Arc<ArcSwap<WorkspaceHandle>>,

    // HACK: this is a temporary solution until we migrate all the necessary
    // functionality and fully get rid of the separate `window` crate.
    w: Arc<ArcSwap<sapic_window::OldSapicWindow<R>>>,

    pub(crate) workspace_ops: MainWindowWorkspaceOps,
    pub(crate) environment_ops: MainWindowEnvironmentOps,

    // Store cancellers by the id of API requests
    pub(crate) tracked_cancellations: Arc<RwLock<HashMap<String, Canceller>>>,
}

impl<R: AppRuntime> Clone for MainWindow<R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            workspace: self.workspace.clone(),
            w: self.w.clone(),
            workspace_ops: self.workspace_ops.clone(),
            environment_ops: self.environment_ops.clone(),
            tracked_cancellations: self.tracked_cancellations.clone(),
        }
    }
}

impl<R: AppRuntime> MainWindow<R> {
    pub async fn new(
        delegate: &AppDelegate<R>,
        window_id: usize,
        old_window: sapic_window::OldSapicWindow<R>,
        workspace: Arc<dyn Workspace>,
        workspace_ops: MainWindowWorkspaceOps,
        environment_ops: MainWindowEnvironmentOps,
    ) -> joinerror::Result<Self> {
        let tao_handle = delegate.handle();
        let label = format!("{MAIN_WINDOW_LABEL_PREFIX}{}", window_id);
        let win_builder = tauri::WebviewWindowBuilder::new(
            &tao_handle,
            label,
            tauri::WebviewUrl::App(
                format!("{}#/{}", MAIN_WINDOW_ENTRY_POINT, workspace.id()).into(),
            ),
        )
        .title("HARDCODED TITLE") // FIXME: HARDCODE
        .center()
        .resizable(true)
        .visible(false)
        .disable_drag_drop_handler()
        .inner_size(800.0, 600.0)
        .position(100.0, 100.0)
        .min_inner_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT)
        .zoom_hotkeys_enabled(true);

        #[cfg(target_os = "windows")]
        let win_builder = win_builder
            .transparent(false)
            .shadow(true)
            .decorations(false);

        #[cfg(target_os = "macos")]
        let win_builder = win_builder
            .hidden_title(true)
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .decorations(true);

        let webview_window = win_builder
            .build()
            .join_err::<()>("failed to build main window")?;

        // Sometimes tauri will ignore builder.decorations(false)
        // Manually set it to make sure that the title bar is hidden on Windows
        #[cfg(target_os = "windows")]
        webview_window.set_decorations(false)?;

        Ok(Self {
            handle: WindowHandle::new(webview_window),
            w: ArcSwap::from_pointee(old_window).into(),
            workspace: ArcSwap::from_pointee(WorkspaceHandle::new(workspace)).into(),
            workspace_ops,
            environment_ops,
            tracked_cancellations: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    // HACK: this is a temporary solution until we migrate all the necessary
    // functionality and fully get rid of the separate `window` crate.
    pub fn inner(&self) -> Arc<sapic_window::OldSapicWindow<R>> {
        self.w.load_full()
    }

    pub fn workspace(&self) -> Arc<dyn Workspace> {
        self.workspace.load().get()
    }

    pub async fn swap_workspace(
        &self,
        new_workspace: Arc<dyn Workspace>,

        // HACK: this is a temporary solution until we migrate all the necessary
        // functionality and fully get rid of the separate `window` crate.
        old_window: sapic_window::OldSapicWindow<R>,
    ) -> joinerror::Result<()> {
        joinerror::ResultExt::join_err::<()>(
            self.workspace.load_full().dispose().await,
            "failed to dispose old workspace",
        )?;

        self.workspace
            .store(Arc::new(WorkspaceHandle::new(new_workspace)));

        self.w.store(Arc::new(old_window));

        Ok(())
    }
}

#[async_trait]
impl<R: AppRuntime> AppWindowApi for MainWindow<R> {
    async fn track_cancellation(&self, request_id: &str, canceller: Canceller) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.insert(request_id.to_string(), canceller);
    }

    async fn release_cancellation(&self, request_id: &str) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.remove(request_id);
    }
}
