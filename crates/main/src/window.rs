pub mod operations;

use async_trait::async_trait;
use derive_more::Deref;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, context::Canceller, errors::TauriResultExt};
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_workspace::models::primitives::WorkspaceId;
use sapic_window::WindowBuilder;
use sapic_window2::{
    AppWindowApi, WindowHandle,
    constants::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH},
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

const MAIN_WINDOW_LABEL_PREFIX: &str = "main_";
const MAIN_WINDOW_ENTRY_POINT: &str = "workspace.html";

/// Main window controller. This window is used to display a workspace.
#[derive(Deref)]
pub struct MainWindow<R: AppRuntime> {
    #[deref]
    pub handle: WindowHandle<R::EventLoop>,

    // HACK: this is a temporary solution until we migrate all the necessary
    // functionality and fully get rid of the separate `window` crate.
    w: Arc<sapic_window::Window<R>>,

    // Store cancellers by the id of API requests
    pub(crate) tracked_cancellations: Arc<RwLock<HashMap<String, Canceller>>>,
}

impl<R: AppRuntime> Clone for MainWindow<R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            w: self.w.clone(),
            tracked_cancellations: self.tracked_cancellations.clone(),
        }
    }
}

impl<R: AppRuntime> MainWindow<R> {
    pub async fn new(
        ctx: &R::AsyncContext,
        delegate: &AppDelegate<R>,
        fs: Arc<dyn FileSystem>,
        keyring: Arc<dyn KeyringClient>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
        window_id: usize,
        workspace_id: WorkspaceId,
    ) -> joinerror::Result<Self> {
        let tao_handle = delegate.handle();
        let w = WindowBuilder::new(fs, keyring, auth_api_client, workspace_id.clone())
            .build(ctx, delegate)
            .await?;

        let label = format!("{MAIN_WINDOW_LABEL_PREFIX}{}", window_id);
        let win_builder = tauri::WebviewWindowBuilder::new(
            &tao_handle,
            label,
            tauri::WebviewUrl::App(format!("{}#/{}", MAIN_WINDOW_ENTRY_POINT, workspace_id).into()),
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

        #[cfg(target_os = "macos")]
        let win_builder = win_builder
            .hidden_title(true)
            .title_bar_style(tauri::TitleBarStyle::Overlay)
            .decorations(true);

        let webview_window = win_builder
            .build()
            .join_err::<()>("failed to build main window")?;

        Ok(Self {
            handle: WindowHandle::new(webview_window),
            w: Arc::new(w),
            tracked_cancellations: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    // HACK: this is a temporary solution until we migrate all the necessary
    // functionality and fully get rid of the separate `window` crate.
    pub fn inner(&self) -> &sapic_window::Window<R> {
        &self.w
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
