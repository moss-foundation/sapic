pub mod operations;

use async_trait::async_trait;
use derive_more::Deref;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, context::Canceller, errors::TauriResultExt};
use std::{collections::HashMap, sync::Arc};
use tauri::WebviewWindow;
use tokio::sync::RwLock;

use sapic_window2::{
    WindowApi,
    constants::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH},
    defaults::{DEFAULT_WINDOW_POSITION_X, DEFAULT_WINDOW_POSITION_Y},
};

pub const WELCOME_WINDOW_LABEL: &str = "welcome";
const WELCOME_WINDOW_ENTRY_POINT: &str = "welcome.html";

/// Welcome window controller
#[derive(Deref)]
pub struct WelcomeWindow<R: AppRuntime> {
    #[deref]
    pub window: WebviewWindow<R::EventLoop>,

    // Store cancellers by the id of API requests
    pub(crate) tracked_cancellations: Arc<RwLock<HashMap<String, Canceller>>>,
}

impl<R: AppRuntime> Clone for WelcomeWindow<R> {
    fn clone(&self) -> Self {
        Self {
            window: self.window.clone(),
            tracked_cancellations: self.tracked_cancellations.clone(),
        }
    }
}

impl<R: AppRuntime> WelcomeWindow<R> {
    pub async fn new(delegate: &AppDelegate<R>) -> joinerror::Result<Self> {
        let tao_handle = delegate.handle();
        let win_builder = tauri::WebviewWindowBuilder::new(
            &tao_handle,
            WELCOME_WINDOW_LABEL,
            tauri::WebviewUrl::App(WELCOME_WINDOW_ENTRY_POINT.into()),
        )
        .title("Welcome")
        .center()
        .resizable(true)
        .visible(false)
        .disable_drag_drop_handler()
        .inner_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT)
        .position(DEFAULT_WINDOW_POSITION_X, DEFAULT_WINDOW_POSITION_Y)
        .min_inner_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT)
        .zoom_hotkeys_enabled(true);

        #[cfg(target_os = "windows")]
        let win_builder = win_builder
            .transparent(false)
            .shadow(true)
            .decorations(false);

        #[cfg(target_os = "macos")]
        let win_builder = win_builder
            .hidden_title(false)
            .title_bar_style(tauri::TitleBarStyle::Transparent)
            .decorations(true);

        let webview_window = win_builder
            .build()
            .join_err::<()>("failed to build welcome window")?;

        Ok(Self {
            window: webview_window,
            tracked_cancellations: Default::default(),
        })
    }

    pub async fn track_cancellation(&self, request_id: &str, canceller: Canceller) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.insert(request_id.to_string(), canceller);
    }

    pub async fn release_cancellation(&self, request_id: &str) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.remove(request_id);
    }
}

#[async_trait]
impl<R: AppRuntime> WindowApi for WelcomeWindow<R> {
    async fn track_cancellation(&self, request_id: &str, canceller: Canceller) -> () {
        self.track_cancellation(request_id, canceller).await;
    }

    async fn release_cancellation(&self, request_id: &str) -> () {
        self.release_cancellation(request_id).await;
    }
}
