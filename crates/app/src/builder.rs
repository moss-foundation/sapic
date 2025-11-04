use moss_app::WindowBuilder;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_extension::ExtensionPoint;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use rustc_hash::FxHashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{App, AppCommands, command::CommandDecl};

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

    pub async fn build(self, ctx: &R::AsyncContext, app_delegate: &AppDelegate<R>) -> App<R> {
        let default_window = WindowBuilder::new(
            self.fs,
            self.keyring,
            self.auth_api_client,
            self.extension_points,
        )
        .build(ctx, app_delegate)
        .await;

        App {
            tauri_handle: app_delegate.handle(),
            commands: self.commands,
            windows: RwLock::new(FxHashMap::from_iter([(
                "main_0".to_string(),
                Arc::new(default_window),
            )])),
        }
    }
}
