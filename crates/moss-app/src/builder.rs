use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_extension::ExtensionPoint;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_locale::registry::LocaleRegistry;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_theme::registry::ThemeRegistry;
use std::{path::PathBuf, sync::Arc};
use tauri::{AppHandle as TauriAppHandle, Manager};
use tokio::sync::RwLock;

use crate::{
    app::{App, AppCommands, AppPreferences},
    command::CommandDecl,
    configuration::ConfigurationService,
    dirs,
    extension::ExtensionService,
    internal::events::{OnDidChangeConfiguration, OnDidChangeProfile, OnDidChangeWorkspace},
    locale::LocaleService,
    logging::LogService,
    profile::ProfileService,
    session::SessionService,
    storage::StorageService,
    theme::ThemeService,
    workspace::WorkspaceService,
};

pub struct BuildAppParams {
    pub locales_dir: PathBuf,
}

pub struct AppBuilder<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    keyring: Arc<dyn KeyringClient>,
    tao_handle: TauriAppHandle<R::EventLoop>,
    extension_points: Vec<Box<dyn ExtensionPoint<R>>>,
    commands: AppCommands<R::EventLoop>,
    auth_api_client: Arc<AccountAuthGatewayApiClient>,
}

impl<R: AppRuntime> AppBuilder<R> {
    pub fn new(
        tao_handle: TauriAppHandle<R::EventLoop>,
        fs: Arc<dyn FileSystem>,
        keyring: Arc<dyn KeyringClient>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
        extension_points: Vec<Box<dyn ExtensionPoint<R>>>,
    ) -> Self {
        Self {
            fs,
            keyring,
            tao_handle,
            extension_points,
            commands: Default::default(),
            auth_api_client,
        }
    }

    pub fn with_command(mut self, command: CommandDecl<R::EventLoop>) -> Self {
        self.commands.insert(command.name, command.callback);
        self
    }

    pub async fn build(self, ctx: &R::AsyncContext) -> App<R> {
        let delegate = self.tao_handle.state::<AppDelegate<R>>().inner().clone();
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

        let locale_service =
            LocaleService::new::<R>(self.fs.clone(), <dyn LocaleRegistry>::global(&delegate))
                .await
                .expect("Failed to create locale service");
        let session_service = SessionService::new();
        let storage_service: Arc<StorageService<R>> =
            StorageService::<R>::new(&user_dir.join(dirs::GLOBALS_DIR))
                .expect("Failed to create storage service")
                .into();
        let log_service = LogService::new(
            self.fs.clone(),
            self.tao_handle.clone(),
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

        App {
            app_handle: self.tao_handle.clone(),
            commands: self.commands,

            // FIXME: hardcoded for now
            preferences: AppPreferences {
                theme: RwLock::new(None),
                locale: RwLock::new(None),
            },

            session_service,
            log_service,
            storage_service,
            workspace_service,
            locale_service,
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
