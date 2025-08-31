use moss_activity_broadcaster::ActivityBroadcaster;
use moss_applib::AppRuntime;
use moss_asp::AppSecretsProvider;
use moss_fs::FileSystem;
use moss_keyring::KeyringClientImpl;
use std::{path::PathBuf, sync::Arc};
use tauri::AppHandle;
use tokio::sync::RwLock;

use crate::{
    app::{App, AppCommands, AppDefaults, AppPreferences},
    command::CommandDecl,
    dirs,
    services::{profile_service::ProfileService, *},
};

pub struct BuildAppParams {
    pub app_dir: PathBuf,
    pub themes_dir: PathBuf,
    pub locales_dir: PathBuf,
    pub logs_dir: PathBuf,
}

pub struct AppBuilder<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    app_handle: AppHandle<R::EventLoop>,
    commands: AppCommands<R::EventLoop>,
}

impl<R: AppRuntime> AppBuilder<R> {
    pub fn new(app_handle: AppHandle<R::EventLoop>, fs: Arc<dyn FileSystem>) -> Self {
        Self {
            fs,
            app_handle,
            commands: Default::default(),
        }
    }

    pub fn with_command(mut self, command: CommandDecl<R::EventLoop>) -> Self {
        self.commands.insert(command.name, command.callback);
        self
    }

    pub async fn build(self, ctx: &R::AsyncContext, params: BuildAppParams) -> App<R> {
        for dir in &[dirs::WORKSPACES_DIR, dirs::GLOBALS_DIR] {
            let dir_path = params.app_dir.join(dir);
            if dir_path.exists() {
                continue;
            }

            self.fs
                .create_dir(&dir_path)
                .await
                .expect("Failed to create app directories");
        }

        let keyring_client = Arc::new(KeyringClientImpl::new());
        let reqwest_client = reqwest::ClientBuilder::new()
            .user_agent("SAPIC")
            .build()
            .expect("failed to build reqwest client");

        // TODO: Fetch OAuth APP secrets from our server in production build

        dotenv::dotenv().ok();
        let github_client_id = dotenv::var("GITHUB_CLIENT_ID").unwrap_or_default();
        let github_client_secret = dotenv::var("GITHUB_CLIENT_SECRET").unwrap_or_default();
        let gitlab_client_id = dotenv::var("GITLAB_CLIENT_ID").unwrap_or_default();
        let gitlab_client_secret = dotenv::var("GITLAB_CLIENT_SECRET").unwrap_or_default();

        // TODO: Probably we should use have it as a global resource instead of creating it here
        let app_secrets = AppSecretsProvider::new(
            github_client_secret.clone(),
            gitlab_client_secret.clone(),
            keyring_client.clone(),
        )
        .await
        .expect("Failed to create app secrets provider");

        let theme_service = ThemeService::new(self.fs.clone(), params.themes_dir)
            .await
            .expect("Failed to create theme service");
        let locale_service = LocaleService::new(self.fs.clone(), params.locales_dir)
            .await
            .expect("Failed to create locale service");
        let session_service = SessionService::new();
        let storage_service: Arc<StorageService<R>> =
            StorageService::<R>::new(&params.app_dir.join(dirs::GLOBALS_DIR))
                .expect("Failed to create storage service")
                .into();
        let log_service = LogService::new(
            self.fs.clone(),
            self.app_handle.clone(),
            &params.logs_dir,
            session_service.session_id(),
            storage_service.clone(),
        )
        .expect("Failed to create log service");
        let profile_service = ProfileService::new(
            self.fs.clone(),
            app_secrets.clone(),
            keyring_client.clone(),
            profile_service::ServiceConfig::new(
                params.app_dir.join(dirs::PROFILES_DIR),
                github_client_id,
                gitlab_client_id,
            )
            .expect("Failed to create profile service config"),
        )
        .await
        .expect("Failed to create profile service");
        let workspace_service = WorkspaceService::<R>::new(
            ctx,
            storage_service.clone(),
            self.fs.clone(),
            &params.app_dir,
        )
        .await
        .expect("Failed to create workspace service");

        let defaults = AppDefaults {
            theme: theme_service.default_theme().await,
            locale: locale_service.default_locale().await,
        };

        App {
            app_dir: params.app_dir,
            app_handle: self.app_handle.clone(),
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
            broadcaster: ActivityBroadcaster::new(self.app_handle),

            // _github_client: github_client,
            // _gitlab_client: gitlab_client,
            _reqwest_client: reqwest_client,
            _keyring_client: keyring_client,
        }
    }
}
