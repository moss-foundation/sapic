use joinerror::ResultExt;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_storage2::KvStorage;
use moss_workspace::builder::{LoadWorkspaceParams, WorkspaceBuilder};
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_system::ports::{
    github_api::GitHubApiClient, gitlab_api::GitLabApiClient, server_api::ServerApiClient,
};
use std::{path::PathBuf, sync::Arc};
use tauri::Manager;

use crate::{
    dirs, logging::LogService, profile::ProfileService, session::SessionService,
    window::OldSapicWindow, workspace::OldWorkspaceService,
};

pub struct OldSapicWindowBuilder {
    workspace_id: WorkspaceId,
    fs: Arc<dyn FileSystem>,
    storage: Arc<dyn KvStorage>,
    keyring: Arc<dyn KeyringClient>,
    server_api_client: Arc<dyn ServerApiClient>,
    github_api_client: Arc<dyn GitHubApiClient>,
    gitlab_api_client: Arc<dyn GitLabApiClient>,
}

impl OldSapicWindowBuilder {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        storage: Arc<dyn KvStorage>,
        keyring: Arc<dyn KeyringClient>,
        server_api_client: Arc<dyn ServerApiClient>,
        github_api_client: Arc<dyn GitHubApiClient>,
        gitlab_api_client: Arc<dyn GitLabApiClient>,
        workspace_id: WorkspaceId,
    ) -> Self {
        Self {
            workspace_id,
            fs,
            storage,
            keyring,
            server_api_client,
            github_api_client,
            gitlab_api_client,
        }
    }

    pub async fn build<R: AppRuntime>(
        self,
        ctx: &R::AsyncContext,
        delegate: &AppDelegate<R>,
    ) -> joinerror::Result<OldSapicWindow<R>> {
        let tao_handle = delegate.app_handle();
        let user_dir = delegate.user_dir();

        self.create_user_dirs_if_not_exists::<R>(ctx, user_dir.clone())
            .await;

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

        let session_service = SessionService::new();

        let log_service = LogService::new::<R>(
            self.fs.clone(),
            tao_handle.clone(),
            &delegate.logs_dir(),
            session_service.session_id(),
        )
        .expect("Failed to create log service");
        let profile_service = ProfileService::new(
            ctx,
            &user_dir.join(dirs::PROFILES_DIR),
            self.fs.clone(),
            self.server_api_client.clone(),
            self.keyring.clone(),
        )
        .await
        .expect("Failed to create profile service");

        // HACK: this is a temporary solution until we migrate all the necessary
        // functionality and fully get rid of the separate `window` crate.
        profile_service.activate_profile().await?;

        let workspace = WorkspaceBuilder::new(
            self.fs.clone(),
            self.storage.clone(),
            profile_service.active_profile().await.unwrap(),
            self.workspace_id.clone(),
            self.github_api_client.clone(),
            self.gitlab_api_client.clone(),
        )
        .load(
            ctx,
            delegate,
            LoadWorkspaceParams {
                abs_path: delegate
                    .workspaces_dir()
                    .join(self.workspace_id.to_string())
                    .into(),
            },
        )
        .await
        .join_err::<()>("failed to load the workspace")?;

        let workspace_service = OldWorkspaceService::new(workspace)
            .await
            .expect("Failed to create workspace service");

        // HACK: this is a temporary solution until we migrate all the necessary
        // functionality and fully get rid of the separate `window` crate.
        // workspace_service
        //     .activate_workspace(
        //         ctx,
        //         delegate,
        //         &self.workspace_id,
        //         profile_service.active_profile().await.unwrap(),
        //     )
        //     .await?;

        Ok(OldSapicWindow {
            app_handle: tao_handle.clone(),
            // user: self.user,
            session_service,
            log_service,
            workspace_service,
            // theme_service,
            // profile_service,
            // configuration_service,
            // tracked_cancellations: Default::default(),
        })
    }

    async fn create_user_dirs_if_not_exists<R: AppRuntime>(
        &self,
        ctx: &R::AsyncContext,
        user_dir: PathBuf,
    ) {
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
                .create_dir(ctx, &dir_path)
                .await
                .expect("Failed to create app directories");
        }
    }
}
