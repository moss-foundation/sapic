use joinerror::ResultExt;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_extension::ExtensionPoint;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_language::registry::LanguageRegistry;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_storage2::Storage;
use moss_theme::registry::ThemeRegistry;
use std::{marker::PhantomData, path::PathBuf, sync::Arc};
use tauri::{AppHandle as TauriAppHandle, Manager, Runtime as TauriRuntime};

#[cfg(target_os = "macos")]
use crate::window::TitleBarStyle;
use crate::{
    configuration::ConfigurationService,
    dirs,
    internal::events::{OnDidChangeConfiguration, OnDidChangeProfile, OnDidChangeWorkspace},
    language::LanguageService,
    logging::LogService,
    profile::ProfileService,
    session::SessionService,
    theme::ThemeService,
    window::Window,
    workspace::WorkspaceService,
};

pub const MIN_WINDOW_WIDTH: f64 = 800.0;
pub const MIN_WINDOW_HEIGHT: f64 = 600.0;

pub struct WindowBuilder {
    fs: Arc<dyn FileSystem>,
    keyring: Arc<dyn KeyringClient>,
    auth_api_client: Arc<AccountAuthGatewayApiClient>,
}

impl WindowBuilder {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        keyring: Arc<dyn KeyringClient>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
    ) -> Self {
        Self {
            fs,
            keyring,
            auth_api_client,
        }
    }

    pub async fn build<R: AppRuntime>(
        self,
        ctx: &R::AsyncContext,
        delegate: &AppDelegate<R>,
        url: &str,
        label: &str,
        title: &str,
        inner_size: (f64, f64),
        position: (f64, f64),

        #[cfg(target_os = "macos")] title_bar_style: TitleBarStyle,
    ) -> joinerror::Result<Window<R>> {
        let tao_handle = delegate.app_handle();
        let user_dir = delegate.user_dir();

        self.create_user_dirs_if_not_exists(user_dir.clone()).await;

        let on_did_change_profile_emitter = EventEmitter::<OnDidChangeProfile>::new();
        let on_did_change_profile_event = on_did_change_profile_emitter.event();

        let on_did_change_workspace_emitter = EventEmitter::<OnDidChangeWorkspace>::new();
        let on_did_change_workspace_event = on_did_change_workspace_emitter.event();

        let on_did_change_configuration_emitter = EventEmitter::<OnDidChangeConfiguration>::new();
        let _on_did_change_configuration_event = on_did_change_configuration_emitter.event();

        let configuration_service = ConfigurationService::new(
            &delegate,
            self.fs.clone(),
            on_did_change_configuration_emitter,
            &on_did_change_profile_event,
            &on_did_change_workspace_event,
        )
        .await
        .expect("Failed to create configuration service");

        let theme_service = ThemeService::new(
            &delegate,
            self.fs.clone(),
            <dyn ThemeRegistry>::global(&delegate),
        )
        .await
        .expect("Failed to create theme service");

        let language_service =
            LanguageService::new::<R>(self.fs.clone(), <dyn LanguageRegistry>::global(&delegate))
                .await
                .expect("Failed to create language service");
        let session_service = SessionService::new();

        let storage = <dyn Storage>::global(&delegate);
        let log_service = LogService::new::<R>(
            self.fs.clone(),
            tao_handle.clone(),
            &delegate.logs_dir(),
            session_service.session_id(),
        )
        .expect("Failed to create log service");
        let profile_service = ProfileService::new(
            &user_dir.join(dirs::PROFILES_DIR),
            self.fs.clone(),
            self.auth_api_client.clone(),
            self.keyring.clone(),
            on_did_change_profile_emitter,
        )
        .await
        .expect("Failed to create profile service");
        let workspace_service =
            WorkspaceService::<R>::new(ctx, storage.clone(), self.fs.clone(), &user_dir)
                .await
                .expect("Failed to create workspace service");

        let webview = create_window(
            &tao_handle,
            url,
            label,
            title,
            inner_size,
            position,
            title_bar_style,
        )
        .join_err::<()>("failed to create webview window")?;

        Ok(Window {
            webview,
            app_handle: tao_handle.clone(),
            session_service,
            log_service,
            workspace_service,
            language_service,
            theme_service,
            profile_service,
            configuration_service,
            tracked_cancellations: Default::default(),
        })
    }

    async fn create_user_dirs_if_not_exists(&self, user_dir: PathBuf) {
        for dir in &[
            dirs::WORKSPACES_DIR,
            dirs::GLOBALS_DIR,
            dirs::PROFILES_DIR,
            dirs::TMP_DIR,
        ] {
            let dir_path = user_dir.join(dir);
            if dir_path.exists() {
                continue;
            }

            self.fs
                .create_dir(&dir_path)
                .await
                .expect("Failed to create app directories");
        }
    }
}

pub fn create_window<R: TauriRuntime>(
    app_handle: &TauriAppHandle<R>,
    url: &str,
    label: &str,
    title: &str,
    inner_size: (f64, f64),
    position: (f64, f64),
    #[cfg(target_os = "macos")] title_bar_style: TitleBarStyle,
) -> joinerror::Result<tauri::WebviewWindow<R>> {
    let win_builder =
        tauri::WebviewWindowBuilder::new(app_handle, label, tauri::WebviewUrl::App(url.into()))
            .title(title)
            .center()
            .resizable(true)
            .visible(false)
            .disable_drag_drop_handler()
            .inner_size(inner_size.0, inner_size.1)
            .position(position.0, position.1)
            .min_inner_size(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT)
            .zoom_hotkeys_enabled(true);

    #[cfg(target_os = "windows")]
    let win_builder = win_builder
        .transparent(false)
        .shadow(true)
        .decorations(false);

    #[cfg(target_os = "macos")]
    let win_builder = win_builder
        .hidden_title(match title_bar_style {
            TitleBarStyle::Visible => false,
            TitleBarStyle::Overlay => true,
        })
        .title_bar_style(match title_bar_style {
            TitleBarStyle::Visible => tauri::TitleBarStyle::Visible,
            TitleBarStyle::Overlay => tauri::TitleBarStyle::Overlay,
        })
        .transparent(false)
        .decorations(true);

    let webview_window = win_builder.build()?;

    if let Err(err) = webview_window.set_focus() {
        // warn!(
        //     "Failed to set focus to window {} when creating it: {}",
        //     input.label, err
        // );
    }

    Ok(webview_window)
}
