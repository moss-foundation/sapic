use moss_activity_indicator::ActivityIndicator;
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_git_hosting_provider::{
    common::ssh_auth_agent::SSHAuthAgentImpl,
    github::{auth::GitHubAuthAgentImpl, client::GitHubClient},
    gitlab::{auth::GitLabAuthAgentImpl, client::GitLabClient},
};
use moss_keyring::KeyringClientImpl;
use std::{path::PathBuf, sync::Arc};
use tauri::AppHandle;
use tokio::sync::RwLock;

use crate::{
    app::{App, AppCommands, AppDefaults, AppPreferences},
    command::CommandDecl,
    dirs,
    services::*,
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

        let theme_service = ThemeService::new(self.fs.clone(), params.themes_dir);
        let locale_service = LocaleService::new(self.fs.clone(), params.locales_dir);
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
        let workspace_service = WorkspaceService::<R>::new(
            ctx,
            storage_service.clone(),
            self.fs.clone(),
            &params.app_dir,
            keyring_client.clone(),
        )
        .await
        .expect("Failed to create workspace service");

        let default_theme = theme_service
            .default_theme()
            .await
            .cloned()
            .expect("Failed to get default theme");

        let default_locale = locale_service
            .default_locale()
            .await
            .cloned()
            .expect("Failed to get default locale");

        let defaults = AppDefaults {
            theme: default_theme,
            locale: default_locale,
        };

        // FIXME: Use actual OAuth App id and secret
        let keyring_client = Arc::new(KeyringClientImpl::new());
        let reqwest_client = reqwest::ClientBuilder::new()
            .user_agent("SAPIC")
            .build()
            .expect("failed to build reqwest client");

        let github_client = {
            let github_auth_agent =
                GitHubAuthAgentImpl::new(keyring_client.clone(), "".to_string(), "".to_string());
            Arc::new(GitHubClient::new(
                reqwest_client.clone(),
                github_auth_agent,
                None as Option<SSHAuthAgentImpl>,
            ))
        };
        let gitlab_client = {
            let gitlab_auth_agent =
                GitLabAuthAgentImpl::new(keyring_client.clone(), "".to_string(), "".to_string());
            Arc::new(GitLabClient::new(
                reqwest_client.clone(),
                gitlab_auth_agent,
                None as Option<SSHAuthAgentImpl>,
            ))
        };

        App {
            app_dir: params.app_dir,
            fs: self.fs,
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
            tracked_cancellations: Default::default(),
            activity_indicator: ActivityIndicator::new(self.app_handle),

            github_client,
            gitlab_client,
            _reqwest_client: reqwest_client,
            keyring_client,
        }
    }
}
