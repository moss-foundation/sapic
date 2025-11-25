use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_storage2::Storage;
use reqwest::Client as HttpClient;
use sapic_platform::{
    github::{AppGitHubApiClient, auth::AppGitHubAuthAdapter},
    gitlab::{AppGitLabApiClient, auth::AppGitLabAuthAdapter},
    server::HttpServerApiClient,
    theme::loader::ThemeLoader,
    workspace::{
        workspace_edit_backend::WorkspaceFsEditBackend, workspace_service_fs::WorkspaceServiceFs,
    },
};
use sapic_runtime::{extension_point::ExtensionPoint, globals::GlobalThemeRegistry};
use sapic_system::{
    application::extensions_service::ExtensionsApiService,
    ports::{github_api::GitHubAuthAdapter, gitlab_api::GitLabAuthAdapter},
    theme::theme_service::ThemeService,
    workspace::{
        workspace_edit_service::WorkspaceEditService, workspace_service::WorkspaceService,
    },
};
use std::sync::Arc;

use crate::{
    App, AppCommands, AppServices, command::CommandDecl, extension::ExtensionService,
    windows::WindowManager,
};

pub struct AppBuilder<R: AppRuntime> {
    commands: AppCommands<R::EventLoop>,
    fs: Arc<dyn FileSystem>,
    keyring: Arc<dyn KeyringClient>,
    auth_api_client: Arc<AccountAuthGatewayApiClient>,
    extension_points: Vec<Box<dyn ExtensionPoint<R>>>,
    server_api_endpoint: String,
    http_client: HttpClient,
}

impl<R: AppRuntime> AppBuilder<R> {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        keyring: Arc<dyn KeyringClient>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
        extension_points: Vec<Box<dyn ExtensionPoint<R>>>,
        server_api_endpoint: String,
        http_client: HttpClient,
    ) -> Self {
        Self {
            commands: Default::default(),
            fs,
            keyring,
            auth_api_client,
            extension_points,
            server_api_endpoint,
            http_client,
        }
    }

    pub fn with_command(mut self, command: CommandDecl<R::EventLoop>) -> Self {
        self.commands.insert(command.name, command.callback);
        self
    }

    pub async fn build(self, _ctx: &R::AsyncContext, delegate: &AppDelegate<R>) -> App<R> {
        let server_api_client: Arc<HttpServerApiClient> =
            HttpServerApiClient::new(self.server_api_endpoint, self.http_client.clone()).into();

        let github_api_client = Arc::new(AppGitHubApiClient::new(self.http_client.clone()));
        let gitlab_api_client = Arc::new(AppGitLabApiClient::new(self.http_client.clone()));

        let auth_gateway_url: Arc<String> = server_api_client.base_url().to_string().into();

        let github_auth_adapter: Arc<dyn GitHubAuthAdapter> = Arc::new(AppGitHubAuthAdapter::new(
            server_api_client.clone(),
            auth_gateway_url.clone(),
            8080,
        ));
        let gitlab_auth_adapter: Arc<dyn GitLabAuthAdapter> = Arc::new(AppGitLabAuthAdapter::new(
            server_api_client.clone(),
            auth_gateway_url,
            8081,
        ));

        let extension_service =
            ExtensionService::<R>::new(&delegate, self.fs.clone(), self.extension_points)
                .await
                .expect("Failed to create extension service");

        let storage = <dyn Storage>::global(&delegate);

        let workspace_edit_backend =
            WorkspaceFsEditBackend::new(self.fs.clone(), delegate.workspaces_dir());
        let workspace_edit_service = WorkspaceEditService::new(Arc::new(workspace_edit_backend));
        let workspace_service = WorkspaceService::new(
            Arc::new(WorkspaceServiceFs::new(
                self.fs.clone(),
                delegate.workspaces_dir(),
            )),
            storage.clone(),
        );

        let services = AppServices {
            workspace_service: workspace_service.into(),
            workspace_edit_service: workspace_edit_service.into(),
            theme_service: ThemeService::new(
                GlobalThemeRegistry::get(delegate),
                ThemeLoader::new(
                    self.fs.clone(),
                    delegate.resource_dir().join("policies/theme.rego"),
                ),
            )
            .await
            .expect("Failed to create theme service")
            .into(),
            extension_api_service: ExtensionsApiService::new(server_api_client.clone()).into(),
        };

        App {
            tao_handle: delegate.handle(),
            fs: self.fs,
            keyring: self.keyring,
            server_api_client,
            github_api_client,
            gitlab_api_client,
            github_auth_adapter,
            gitlab_auth_adapter,
            extension_service,
            commands: self.commands,
            windows: WindowManager::new(),
            services,
        }
    }
}
