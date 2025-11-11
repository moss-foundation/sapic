mod extension;
mod language;
mod profile;
mod theme;

pub mod builder;
pub mod command;
pub mod types;

use derive_more::Deref;
use joinerror::ResultExt;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, errors::TauriResultExt as _};
use moss_extension::ExtensionPoint;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_text::ReadOnlyStr;
use moss_workspace::models::primitives::WorkspaceId;
use rustc_hash::FxHashMap;
use sapic_window::{
    Window, WindowBuilder,
    window::{OnWindowReadyOptions, TitleBarStyle},
};
use std::{
    ops::{Deref, DerefMut},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};
use tauri::{AppHandle as TauriAppHandle, Runtime as TauriRuntime};
use tokio::sync::RwLock;
use tracing::instrument;

use crate::{command::CommandCallback, extension::ExtensionService};

pub struct AppCommands<R: TauriRuntime>(FxHashMap<ReadOnlyStr, CommandCallback<R>>);

impl<R: TauriRuntime> Default for AppCommands<R> {
    fn default() -> Self {
        Self(FxHashMap::default())
    }
}

impl<R: TauriRuntime> Deref for AppCommands<R> {
    type Target = FxHashMap<ReadOnlyStr, CommandCallback<R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: TauriRuntime> DerefMut for AppCommands<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub enum CreateWindowParams {
    WelcomeWindow,
    WorkspaceWindow { id: WorkspaceId, name: String },
}

#[derive(Deref)]
pub struct App<R: AppRuntime> {
    #[deref]
    pub(crate) tao_handle: TauriAppHandle<R::EventLoop>,
    pub(crate) fs: Arc<dyn FileSystem>,
    pub(crate) keyring: Arc<dyn KeyringClient>,
    pub(crate) auth_api_client: Arc<AccountAuthGatewayApiClient>,
    pub(crate) commands: AppCommands<R::EventLoop>,
    pub(crate) windows: RwLock<FxHashMap<String, Arc<Window<R>>>>,
    pub(crate) next_window_id: AtomicUsize,

    #[allow(unused)]
    pub(crate) extension_service: ExtensionService<R>,
}

impl<R: AppRuntime> App<R> {
    pub fn handle(&self) -> TauriAppHandle<R::EventLoop> {
        self.tao_handle.clone()
    }

    pub async fn window(&self, id: &str) -> Option<Arc<Window<R>>> {
        self.windows.read().await.get(id).cloned()
    }

    pub async fn create_window(
        &self,
        ctx: &R::AsyncContext,
        delegate: &AppDelegate<R>,
        params: CreateWindowParams,
    ) -> joinerror::Result<tauri::WebviewWindow<R::EventLoop>> {
        let label = format!(
            "main_{}",
            self.next_window_id.fetch_add(1, Ordering::Relaxed)
        );

        let (url, title) = match &params {
            CreateWindowParams::WelcomeWindow => {
                ("welcome.html".to_string(), "Welcome".to_string())
            }
            CreateWindowParams::WorkspaceWindow { id, name, .. } => {
                // (format!("/workspace/{}", id), name.clone())
                (format!("workspace.html?id={}", id), name.to_string())
            }
        };

        let window = WindowBuilder::new(
            self.fs.clone(),
            self.keyring.clone(),
            self.auth_api_client.clone(),
        )
        .build(
            ctx,
            delegate,
            url.as_str(),
            label.as_str(),
            title.as_str(),
            (800.0, 600.0),
            (100.0, 100.0),
        )
        .await?;
        window
            .on_window_ready(
                &ctx,
                &delegate,
                OnWindowReadyOptions {
                    restore_last_workspace: false, // FIXME: HARDCODE
                },
            )
            .await
            .expect("Failed to prepare the app");

        let webview = window.webview().clone();

        self.windows.write().await.insert(label, Arc::new(window));

        Ok(webview)
    }

    pub fn command(&self, id: &ReadOnlyStr) -> Option<CommandCallback<R::EventLoop>> {
        self.commands.get(id).map(|cmd| Arc::clone(cmd))
    }
}

// pub const MIN_WINDOW_WIDTH: f64 = 800.0;
// pub const MIN_WINDOW_HEIGHT: f64 = 600.0;

// #[instrument(level = "debug", skip(app_handle))]
// pub fn create_window<R: TauriRuntime>(
//     app_handle: &TauriAppHandle<R>,
//     url: &str,
//     label: &str,
//     title: &str,
//     inner_size: (f64, f64),
//     position: (f64, f64),
// ) -> joinerror::Result<tauri::WebviewWindow<R>> {
//     let win_builder =
//         tauri::WebviewWindowBuilder::new(app_handle, label, tauri::WebviewUrl::App(url.into()))
//             .title(title)
//             .center()
//             .resizable(true)
//             .visible(false)
//             .disable_drag_drop_handler()
//             .inner_size(inner_size.0, inner_size.1)
//             .position(position.0, position.1)
//             .min_inner_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT)
//             .zoom_hotkeys_enabled(true);

//     #[cfg(target_os = "windows")]
//     let win_builder = win_builder
//         .transparent(false)
//         .shadow(true)
//         .decorations(false);

//     #[cfg(target_os = "macos")]
//     let win_builder = win_builder
//         .hidden_title(true)
//         .title_bar_style(tauri::TitleBarStyle::Overlay)
//         .transparent(false)
//         .decorations(true);

//     let webview_window = win_builder.build()?;

//     if let Err(err) = webview_window.set_focus() {
//         // warn!(
//         //     "Failed to set focus to window {} when creating it: {}",
//         //     input.label, err
//         // );
//     }

//     Ok(webview_window)
// }
