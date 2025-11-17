use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_language::registry::LanguageRegistry;
use moss_server_api::account_auth_gateway::AccountAuthGatewayApiClient;
use moss_storage2::Storage;
// use moss_theme::registry::ThemeRegistry;
use moss_workspace::models::primitives::WorkspaceId;
use std::{path::PathBuf, sync::Arc};
use tauri::Manager;

use crate::{
    dirs,

    language::LanguageService,
    logging::LogService,
    profile::ProfileService,
    session::SessionService,
    // theme::ThemeService,
    window::Window,
    workspace::WorkspaceService,
};

pub struct WindowBuilder {
    workspace_id: WorkspaceId,
    fs: Arc<dyn FileSystem>,
    keyring: Arc<dyn KeyringClient>,
    auth_api_client: Arc<AccountAuthGatewayApiClient>,
}

impl WindowBuilder {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        keyring: Arc<dyn KeyringClient>,
        auth_api_client: Arc<AccountAuthGatewayApiClient>,
        workspace_id: WorkspaceId,
    ) -> Self {
        Self {
            workspace_id,
            fs,
            keyring,
            auth_api_client,
        }
    }

    pub async fn build<R: AppRuntime>(
        self,
        ctx: &R::AsyncContext,
        delegate: &AppDelegate<R>,
    ) -> joinerror::Result<Window<R>> {
        let tao_handle = delegate.app_handle();
        let user_dir = delegate.user_dir();

        self.create_user_dirs_if_not_exists(user_dir.clone()).await;

        // let on_did_change_profile_emitter = EventEmitter::<OnDidChangeProfile>::new();
        // let on_did_change_profile_event = on_did_change_profile_emitter.event();

        // let on_did_change_workspace_emitter = EventEmitter::<OnDidChangeWorkspace>::new();
        // let on_did_change_workspace_event = on_did_change_workspace_emitter.event();

        // let on_did_change_configuration_emitter = EventEmitter::<OnDidChangeConfiguration>::new();
        // let _on_did_change_configuration_event = on_did_change_configuration_emitter.event();

        // let configuration_service = ConfigurationServiceOld::new(
        //     &delegate,
        //     self.fs.clone(),
        //     on_did_change_configuration_emitter,
        //     &on_did_change_profile_event,
        //     &on_did_change_workspace_event,
        // )
        // .await
        // .expect("Failed to create configuration service");

        // let theme_service = ThemeService::new(
        //     &delegate,
        //     self.fs.clone(),
        //     <dyn ThemeRegistry>::global(&delegate),
        // )
        // .await
        // .expect("Failed to create theme service");

        let language_service =
            LanguageService::new::<R>(self.fs.clone(), <dyn LanguageRegistry>::global(&delegate))
                .await
                .expect("Failed to create language service");
        let session_service = SessionService::new();

        let storage = <dyn Storage>::global(&delegate);
        let log_service = LogService::new::<R>(
            self.fs.clone(),
            tao_handle.clone(),
            &delegate.logs_dir(),
            session_service.session_id(),
        )
        .expect("Failed to create log service");
        let profile_service = ProfileService::new(
            &user_dir.join(dirs::PROFILES_DIR),
            self.fs.clone(),
            self.auth_api_client.clone(),
            self.keyring.clone(),
        )
        .await
        .expect("Failed to create profile service");

        // HACK: this is a temporary solution until we migrate all the necessary
        // functionality and fully get rid of the separate `window` crate.
        profile_service.activate_profile().await?;

        let workspace_service =
            WorkspaceService::<R>::new(ctx, storage.clone(), self.fs.clone(), &user_dir)
                .await
                .expect("Failed to create workspace service");

        // HACK: this is a temporary solution until we migrate all the necessary
        // functionality and fully get rid of the separate `window` crate.
        workspace_service
            .activate_workspace(
                ctx,
                delegate,
                &self.workspace_id,
                profile_service.active_profile().await.unwrap(),
            )
            .await?;

        Ok(Window {
            app_handle: tao_handle.clone(),
            session_service,
            log_service,
            workspace_service,
            language_service,
            // theme_service,
            profile_service,
            // configuration_service,
            // tracked_cancellations: Default::default(),
        })
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
