use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, subscription::EventEmitter};
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use std::sync::Arc;

use crate::{
    Window, dirs, internal::events::OnDidChangeProfile, profile::ProfileService,
    storage::StorageService, workspace::WorkspaceService,
};

pub struct WindowBuilder {
    label: String,
    fs: Arc<dyn FileSystem>,
    keyring: Arc<dyn KeyringClient>,
    auth_api_client: Arc<AccountAuthGatewayApiClient>,
}

impl WindowBuilder {
    pub fn new(
        label: String,
        fs: Arc<dyn FileSystem>,
        keyring: Arc<dyn KeyringClient>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
    ) -> Self {
        Self {
            label,
            fs,
            keyring,
            auth_api_client,
        }
    }

    pub async fn build<R: AppRuntime>(
        self,
        ctx: &R::AsyncContext,
        delegate: &AppDelegate<R>,
        storage_service: Arc<StorageService<R>>, // HACK: should be removed
    ) -> Window<R> {
        let user_dir = delegate.user_dir();
        let on_did_change_profile_emitter = EventEmitter::<OnDidChangeProfile>::new();
        let on_did_change_profile_event = on_did_change_profile_emitter.event();

        let workspace_service =
            WorkspaceService::<R>::new(ctx, storage_service, self.fs.clone(), &user_dir)
                .await
                .expect("Failed to create workspace service");

        let profile_service = ProfileService::new(
            &user_dir.join(dirs::PROFILES_DIR),
            self.fs.clone(),
            self.auth_api_client.clone(),
            self.keyring.clone(),
            on_did_change_profile_emitter,
        )
        .await
        .expect("Failed to create profile service");

        Window {
            label: self.label,
            workspace_service,
            // storage_service,
            profile_service,
            tracked_cancellations: Default::default(),
            on_did_change_profile_event,
        }
    }
}
