use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_extension::ExtensionPoint;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_language::registry::LanguageRegistry;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_theme::registry::ThemeRegistry;
use std::{path::PathBuf, sync::Arc};
use tauri::Manager;

use crate::{
    app::Window,
    configuration::ConfigurationService,
    dirs,
    extension::ExtensionService,
    internal::events::{OnDidChangeConfiguration, OnDidChangeProfile, OnDidChangeWorkspace},
    language::LanguageService,
    logging::LogService,
    profile::ProfileService,
    session::SessionService,
    storage::StorageService,
    theme::ThemeService,
    workspace::WorkspaceService,
};

pub struct WindowBuilder<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    keyring: Arc<dyn KeyringClient>,
    extension_points: Vec<Box<dyn ExtensionPoint<R>>>,
    auth_api_client: Arc<AccountAuthGatewayApiClient>,
}

impl<R: AppRuntime> WindowBuilder<R> {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        keyring: Arc<dyn KeyringClient>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
        extension_points: Vec<Box<dyn ExtensionPoint<R>>>,
    ) -> Self {
        Self {
            fs,
            keyring,
            extension_points,
            auth_api_client,
        }
    }

    pub async fn build(self, ctx: &R::AsyncContext, delegate: &AppDelegate<R>) -> Window<R> {
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
        let storage_service: Arc<StorageService<R>> =
            StorageService::<R>::new(&user_dir.join(dirs::GLOBALS_DIR))
                .expect("Failed to create storage service")
                .into();
        let log_service = LogService::new(
            self.fs.clone(),
            tao_handle.clone(),
            &delegate.logs_dir(),
            session_service.session_id(),
            storage_service.clone(),
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
            WorkspaceService::<R>::new(ctx, storage_service.clone(), self.fs.clone(), &user_dir)
                .await
                .expect("Failed to create workspace service");

        let extension_service =
            ExtensionService::<R>::new(&delegate, self.fs.clone(), self.extension_points)
                .await
                .expect("Failed to create extension service");

        Window {
            app_handle: tao_handle.clone(),
            session_service,
            log_service,
            storage_service,
            workspace_service,
            language_service,
            theme_service,
            profile_service,
            configuration_service,
            extension_service,
            tracked_cancellations: Default::default(),
        }
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
