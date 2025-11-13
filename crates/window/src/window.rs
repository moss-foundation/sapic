use derive_more::Deref;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, context::Canceller};
use moss_logging::session;
use moss_storage2::{Storage, models::primitives::StorageScope};
use moss_workspace::models::primitives::WorkspaceId;
use std::{collections::HashMap, sync::Arc};
use tauri::AppHandle;
use tokio::sync::RwLock;

use crate::{
    ActiveWorkspace, configuration::ConfigurationService, language::LanguageService,
    logging::LogService, models::primitives::SessionId, profile::ProfileService,
    session::SessionService, storage::KEY_LAST_ACTIVE_WORKSPACE, workspace::WorkspaceService,
};

#[derive(Default)]
pub enum TitleBarStyle {
    #[default]
    Visible,
    Overlay,
}

pub struct OnWindowReadyOptions {
    pub restore_last_workspace: bool,
}

#[derive(Deref)]
pub struct Window<R: AppRuntime> {
    #[deref]
    pub(super) app_handle: AppHandle<R::EventLoop>,
    pub(super) session_service: SessionService,
    pub(super) log_service: LogService,
    pub(super) workspace_service: WorkspaceService<R>,
    pub(super) language_service: LanguageService,
    // pub(super) theme_service: ThemeService,
    pub(super) profile_service: ProfileService<R>,
    pub(super) configuration_service: ConfigurationService,

    // #[allow(unused)]
    // pub(super) extension_service: ExtensionService<R>,

    // Store cancellers by the id of API requests
    pub(super) tracked_cancellations: Arc<RwLock<HashMap<String, Canceller>>>,
}

impl<R: AppRuntime> Window<R> {
    pub fn session_id(&self) -> &SessionId {
        self.session_service.session_id()
    }

    pub fn handle(&self) -> AppHandle<R::EventLoop> {
        self.app_handle.clone()
    }

    pub async fn workspace(&self) -> Option<Arc<ActiveWorkspace<R>>> {
        self.workspace_service.workspace().await
    }

    pub async fn track_cancellation(&self, request_id: &str, canceller: Canceller) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.insert(request_id.to_string(), canceller);
    }

    pub async fn release_cancellation(&self, request_id: &str) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.remove(request_id);
    }

    pub async fn on_window_ready(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        options: OnWindowReadyOptions,
    ) -> joinerror::Result<()> {
        let profile = self.profile_service.activate_profile().await?;

        let storage = <dyn Storage>::global(app_delegate);
        if options.restore_last_workspace {
            let last_active_workspace_result = storage
                .get(StorageScope::Application, KEY_LAST_ACTIVE_WORKSPACE)
                .await;
            if last_active_workspace_result.is_err() {
                session::warn!(format!(
                    "failed to restore last active workspace: {}",
                    last_active_workspace_result.unwrap_err().to_string()
                ));
                return Ok(());
            }
            if let Some(value) = last_active_workspace_result? {
                let id_str = value.as_str();
                if id_str.is_none() {
                    session::warn!(format!(
                        "failed to parse last active workspace from database"
                    ));
                    return Ok(());
                }
                let id = WorkspaceId::from(id_str.unwrap().to_string());
                if let Err(err) = self
                    .workspace_service
                    .activate_workspace(ctx, app_delegate, &id, profile)
                    .await
                {
                    session::warn!(format!(
                        "failed to activate last active workspace: {}",
                        err.to_string()
                    ));
                }
            }
        }

        Ok(())
    }
}

#[cfg(feature = "integration-tests")]
impl<R: AppRuntime> Window<R> {
    pub fn cancellation_map(&self) -> Arc<RwLock<HashMap<String, Canceller>>> {
        self.tracked_cancellations.clone()
    }

    pub async fn active_profile(&self) -> Option<Arc<moss_user::profile::Profile<R>>> {
        self.profile_service.active_profile().await
    }
}
