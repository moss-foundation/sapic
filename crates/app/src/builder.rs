use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_storage2::KvStorage;
use sapic_platform::{
    extension::unpacker::ExtensionUnpackerImpl,
    language::loader::LanguagePackLoader,
    theme::loader::ColorThemeLoader,
    workspace::{
        workspace_edit_backend::WorkspaceFsEditBackend, workspace_service_fs::WorkspaceServiceFs,
    },
};
use sapic_runtime::extension_point::ExtensionPoint;
use sapic_system::{
    application::extensions_service::ExtensionsApiService,
    language::{LanguagePackRegistry, language_service::LanguageService},
    ports::{
        github_api::GitHubApiClient, gitlab_api::GitLabApiClient, server_api::ServerApiClient,
    },
    theme::{ThemeRegistry, theme_service::ThemeService},
    user::User,
    workspace::{
        workspace_edit_service::WorkspaceEditService, workspace_service::WorkspaceService,
    },
};
use std::sync::Arc;

use crate::{
    App, AppCommands, AppServices, command::CommandDecl,
    services::extension_service::ExtensionService, windows::WindowManager,
};

pub struct AppBuilder<R: AppRuntime> {
    user: Arc<dyn User>,
    commands: AppCommands<R::EventLoop>,
    fs: Arc<dyn FileSystem>,
    keyring: Arc<dyn KeyringClient>,
    extension_points: Vec<Box<dyn ExtensionPoint<R>>>,
    server_api_client: Arc<dyn ServerApiClient>,
    github_api_client: Arc<dyn GitHubApiClient>,
    gitlab_api_client: Arc<dyn GitLabApiClient>,
    storage: Arc<dyn KvStorage>,
    theme_registry: Arc<dyn ThemeRegistry>,
    language_registry: Arc<dyn LanguagePackRegistry>,
}

impl<R: AppRuntime> AppBuilder<R> {
    pub fn new(
        user: Arc<dyn User>,
        fs: Arc<dyn FileSystem>,
        keyring: Arc<dyn KeyringClient>,
        extension_points: Vec<Box<dyn ExtensionPoint<R>>>,
        server_api_client: Arc<dyn ServerApiClient>,
        github_api_client: Arc<dyn GitHubApiClient>,
        gitlab_api_client: Arc<dyn GitLabApiClient>,
        storage: Arc<dyn KvStorage>,
        theme_registry: Arc<dyn ThemeRegistry>,
        language_registry: Arc<dyn LanguagePackRegistry>,
    ) -> Self {
        Self {
            user,
            commands: Default::default(),
            fs,
            keyring,
            extension_points,
            server_api_client,
            github_api_client,
            gitlab_api_client,
            storage,
            theme_registry,
            language_registry,
        }
    }

    pub fn with_command(mut self, command: CommandDecl<R::EventLoop>) -> Self {
        self.commands.insert(command.name, command.callback);
        self
    }

    pub async fn build(self, _ctx: &R::AsyncContext, delegate: &AppDelegate<R>) -> App<R> {
        let extension_unpacker = ExtensionUnpackerImpl::new(self.fs.clone());
        let extension_service = ExtensionService::<R>::new(
            &delegate,
            self.fs.clone(),
            self.extension_points,
            delegate.user_dir().join("extensions"),
            delegate.tmp_dir(),
            Arc::new(extension_unpacker),
        )
        .await
        .expect("Failed to create extension service");

        let workspace_edit_backend =
            WorkspaceFsEditBackend::new(self.fs.clone(), delegate.workspaces_dir());
        let workspace_edit_service = WorkspaceEditService::new(workspace_edit_backend).into();
        let workspace_service = WorkspaceService::new(
            WorkspaceServiceFs::new(self.fs.clone(), delegate.workspaces_dir()),
            self.storage.clone(),
        )
        .into();

        let theme_service = ThemeService::new(
            self.theme_registry,
            ColorThemeLoader::new(
                self.fs.clone(),
                delegate.resource_dir().join("policies/theme.rego"),
            ),
        )
        .await
        .expect("Failed to create theme service")
        .into();

        let language_service = LanguageService::new(
            self.language_registry,
            LanguagePackLoader::new(self.fs.clone()),
        )
        .expect("Failed to create language service")
        .into();

        let extension_api_service =
            ExtensionsApiService::new(self.server_api_client.clone()).into();

        let services = AppServices {
            workspace_service,
            workspace_edit_service,
            theme_service,
            language_service,
            extension_api_service,
        };

        let windows = WindowManager::new(self.storage.clone());

        App {
            tao_handle: delegate.handle(),
            fs: self.fs,
            keyring: self.keyring,
            storage: self.storage,
            server_api_client: self.server_api_client,
            github_api_client: self.github_api_client,
            gitlab_api_client: self.gitlab_api_client,
            extension_service,
            user: self.user,
            commands: self.commands,
            windows,
            services,
        }
    }
}
