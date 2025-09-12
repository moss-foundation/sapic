use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_fs::{CreateOptions, FileSystem, utils};
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_user::models::{primitives::ProfileId, types::ProfileInfo};
use std::{cell::LazyCell, path::PathBuf, sync::Arc};
use tauri::{AppHandle as TauriAppHandle, Manager};
use tokio::sync::RwLock;

use crate::{
    app::{App, AppCommands, AppDefaults, AppPreferences},
    command::CommandDecl,
    dirs,
    profile::ProfileFile,
    services::{profile_service::ProfileService, *},
};

const DEFAULT_PROFILE: LazyCell<ProfileInfo> = LazyCell::new(|| ProfileInfo {
    id: ProfileId::new(),
    name: "Default".to_string(),
    accounts: vec![],
});

pub struct BuildAppParams {
    pub themes_dir: PathBuf,
    pub locales_dir: PathBuf,
    pub logs_dir: PathBuf,
}

pub struct AppBuilder<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    keyring: Arc<dyn KeyringClient>,
    tao_handle: TauriAppHandle<R::EventLoop>,
    commands: AppCommands<R::EventLoop>,
    auth_api_client: Arc<AccountAuthGatewayApiClient>,
}

impl<R: AppRuntime> AppBuilder<R> {
    pub fn new(
        tao_handle: TauriAppHandle<R::EventLoop>,
        fs: Arc<dyn FileSystem>,
        keyring: Arc<dyn KeyringClient>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
    ) -> Self {
        Self {
            fs,
            keyring,
            tao_handle,
            commands: Default::default(),
            auth_api_client,
        }
    }

    pub fn with_command(mut self, command: CommandDecl<R::EventLoop>) -> Self {
        self.commands.insert(command.name, command.callback);
        self
    }

    pub async fn build(self, ctx: &R::AsyncContext, params: BuildAppParams) -> App<R> {
        let delegate = self.tao_handle.state::<AppDelegate<R>>().inner().clone();
        let app_dir = delegate.app_dir();

        self.create_app_dirs_if_not_exists(app_dir.clone()).await;

        self.create_default_profile_if_not_exists(app_dir.join(dirs::PROFILES_DIR))
            .await;

        let theme_service = ThemeService::new(self.fs.clone(), params.themes_dir)
            .await
            .expect("Failed to create theme service");
        let locale_service = LocaleService::new(self.fs.clone(), params.locales_dir)
            .await
            .expect("Failed to create locale service");
        let session_service = SessionService::new();
        let storage_service: Arc<StorageService<R>> =
            StorageService::<R>::new(&app_dir.join(dirs::GLOBALS_DIR))
                .expect("Failed to create storage service")
                .into();
        let log_service = LogService::new(
            self.fs.clone(),
            self.tao_handle.clone(),
            &params.logs_dir,
            session_service.session_id(),
            storage_service.clone(),
        )
        .expect("Failed to create log service");
        let profile_service = ProfileService::new(
            &app_dir.join(dirs::PROFILES_DIR),
            self.fs.clone(),
            self.auth_api_client.clone(),
            self.keyring.clone(),
        )
        .await
        .expect("Failed to create profile service");
        let workspace_service =
            WorkspaceService::<R>::new(ctx, storage_service.clone(), self.fs.clone(), &app_dir)
                .await
                .expect("Failed to create workspace service");

        let defaults = AppDefaults {
            theme: theme_service.default_theme().await,
            locale: locale_service.default_locale().await,
        };
        App {
            app_dir,
            app_handle: self.tao_handle.clone(),
            commands: self.commands,

            // FIXME: hardcoded for now
            preferences: AppPreferences {
                theme: RwLock::new(None),
                locale: RwLock::new(None),
            },

            defaults,
            session_service,
            log_service,
            storage_service,
            workspace_service,
            locale_service,
            theme_service,
            profile_service,
            tracked_cancellations: Default::default(),
        }
    }

    async fn create_app_dirs_if_not_exists(&self, app_dir: PathBuf) {
        for dir in &[dirs::WORKSPACES_DIR, dirs::GLOBALS_DIR, dirs::PROFILES_DIR] {
            let dir_path = app_dir.join(dir);
            if dir_path.exists() {
                continue;
            }

            self.fs
                .create_dir(&dir_path)
                .await
                .expect("Failed to create app directories");
        }
    }

    async fn create_default_profile_if_not_exists(&self, profile_dir_abs: PathBuf) {
        if !utils::is_dir_empty(&profile_dir_abs)
            .await
            .expect("Failed to check if profiles directory is empty")
        {
            return;
        }

        let content = serde_json::to_string_pretty(&ProfileFile {
            name: DEFAULT_PROFILE.name.clone(),
            is_default: Some(true),
            theme: None,
            locale: None,
            zoom_level: None,
            accounts: vec![],
        })
        .expect("Failed to serialize default profile");
        let path = profile_dir_abs.join(format!("{}.json", DEFAULT_PROFILE.id));

        self.fs
            .create_file_with(
                &path,
                content.as_bytes(),
                CreateOptions {
                    overwrite: false,
                    ignore_if_exists: true,
                },
            )
            .await
            .expect("Failed to create default profile");
    }
}
