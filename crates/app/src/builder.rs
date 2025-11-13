use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_extension::ExtensionPoint;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_storage2::Storage;
use sapic_runtime::globals::GlobalThemeRegistry;
use sapic_system::{
    theme::theme_service::ThemeService, workspace::workspace_service::WorkspaceService,
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
}

impl<R: AppRuntime> AppBuilder<R> {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        keyring: Arc<dyn KeyringClient>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
        extension_points: Vec<Box<dyn ExtensionPoint<R>>>,
    ) -> Self {
        Self {
            commands: Default::default(),
            fs,
            keyring,
            auth_api_client,
            extension_points,
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

        let services = AppServices {
            workspace_service: WorkspaceService::new(
                self.fs.clone(),
                storage.clone(),
                delegate.workspaces_dir(),
            )
            .await
            .into(),
            theme_service: ThemeService::new(
                delegate.resource_dir(),
                self.fs.clone(),
                GlobalThemeRegistry::get(delegate),
            )
            .await
            .expect("Failed to create theme service")
            .into(),
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
