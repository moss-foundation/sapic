use derive_more::Deref;
use joinerror::ResultExt;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_workspace::models::primitives::WorkspaceId;
use sapic_window::WindowBuilder;
use sapic_window2::constants::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH};
use std::sync::Arc;
use tauri::WebviewWindow;

const MAIN_WINDOW_LABEL_PREFIX: &str = "main_";
const MAIN_WINDOW_ENTRY_POINT: &str = "workspace.html";

/// Main window controller. This window is used to display a workspace.
#[derive(Deref)]
pub struct MainWindow<R: AppRuntime> {
    #[deref]
    pub window: WebviewWindow<R::EventLoop>,

    // HACK: this is a temporary solution until we migrate all the necessary
    // functionality and fully get rid of the separate `window` crate.
    pub w: Arc<sapic_window::Window<R>>,
}

impl<R: AppRuntime> Clone for MainWindow<R> {
    fn clone(&self) -> Self {
        Self {
            window: self.window.clone(),
            w: self.w.clone(),
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
            .transparent(false)
            .decorations(true);

        let webview_window = win_builder
            .build()
            .join_err::<()>("failed to build main window")?;

        Ok(Self {
            window: webview_window,
            w: Arc::new(w),
        })
    }
}
