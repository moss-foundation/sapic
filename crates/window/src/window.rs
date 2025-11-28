use derive_more::Deref;
use moss_applib::AppRuntime;
use moss_workspace::Workspace;
use std::sync::Arc;
use tauri::AppHandle;

use crate::{
    language::LanguageService, logging::LogService, models::primitives::SessionId,
    profile::ProfileService, session::SessionService, workspace::OldWorkspaceService,
};

#[derive(Deref)]
pub struct OldSapicWindow<R: AppRuntime> {
    #[deref]
    pub(super) app_handle: AppHandle<R::EventLoop>,
    pub(super) session_service: SessionService,
    pub(super) log_service: LogService,
    pub(super) workspace_service: OldWorkspaceService<R>,
    pub(super) language_service: LanguageService,
    // pub(super) theme_service: ThemeService,
    pub(super) profile_service: ProfileService,
    // pub(super) configuration_service: ConfigurationServiceOld,
    // #[allow(unused)]
    // pub(super) extension_service: ExtensionService<R>,

    // Store cancellers by the id of API requests
    // pub(super) tracked_cancellations: Arc<RwLock<HashMap<String, Canceller>>>,
}

impl<R: AppRuntime> OldSapicWindow<R> {
    pub fn session_id(&self) -> &SessionId {
        self.session_service.session_id()
    }

    pub fn handle(&self) -> AppHandle<R::EventLoop> {
        self.app_handle.clone()
    }

    pub async fn workspace(&self) -> Option<Arc<Workspace<R>>> {
        self.workspace_service.workspace().await
    }

    // pub async fn track_cancellation(&self, request_id: &str, canceller: Canceller) -> () {
    //     let mut write = self.tracked_cancellations.write().await;

    //     write.insert(request_id.to_string(), canceller);
    // }

    // pub async fn release_cancellation(&self, request_id: &str) -> () {
    //     let mut write = self.tracked_cancellations.write().await;

    //     write.remove(request_id);
    // }
}

#[cfg(feature = "integration-tests")]
impl<R: AppRuntime> OldSapicWindow<R> {
    // pub fn cancellation_map(&self) -> Arc<RwLock<HashMap<String, Canceller>>> {
    //     self.tracked_cancellations.clone()
    // }

    pub async fn active_profile(&self) -> Option<Arc<sapic_system::user::profile::Profile>> {
        self.profile_service.active_profile().await
    }
}
