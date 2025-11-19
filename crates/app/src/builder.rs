use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_storage2::Storage;
use reqwest::Client as HttpClient;
use sapic_platform::{
    server::HttpServerApiClient, theme::loader::ThemeLoader,
    workspace::workspace_discoverer::WorkspaceDiscoverer,
};
use sapic_runtime::{extension_point::ExtensionPoint, globals::GlobalThemeRegistry};
use sapic_system::{
    application::extensions_service::ExtensionsApiService, theme::theme_service::ThemeService,
    workspace::workspace_service::WorkspaceService,
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
        let extension_service =
            ExtensionService::<R>::new(&delegate, self.fs.clone(), self.extension_points)
                .await
                .expect("Failed to create extension service");

        let storage = <dyn Storage>::global(&delegate);

        let server_api_client: Arc<HttpServerApiClient> =
            HttpServerApiClient::new(self.server_api_endpoint, self.http_client).into();

        let services = AppServices {
            workspace_service: WorkspaceService::new(
                Arc::new(WorkspaceDiscoverer::new(
                    self.fs.clone(),
                    delegate.workspaces_dir(),
                )),
                storage.clone(),
            )
            .await
            .into(),
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
            extension_api_service: ExtensionsApiService::new(server_api_client).into(),
        };

        App {
            tao_handle: delegate.handle(),
            fs: self.fs,
            keyring: self.keyring,
            auth_api_client: self.auth_api_client,
            extension_service,
            commands: self.commands,
            windows: WindowManager::new(),
            services,
        }
    }
}
