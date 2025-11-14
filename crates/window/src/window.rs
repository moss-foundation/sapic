use derive_more::Deref;
use moss_applib::AppRuntime;
use std::sync::Arc;
use tauri::AppHandle;

use crate::{
    ActiveWorkspace, configuration::ConfigurationService, language::LanguageService,
    logging::LogService, models::primitives::SessionId, profile::ProfileService,
    session::SessionService, workspace::WorkspaceService,
};

#[derive(Default)]
pub enum TitleBarStyle {
    #[default]
    Visible,
    Overlay,
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
    // pub(super) tracked_cancellations: Arc<RwLock<HashMap<String, Canceller>>>,
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
impl<R: AppRuntime> Window<R> {
    // pub fn cancellation_map(&self) -> Arc<RwLock<HashMap<String, Canceller>>> {
    //     self.tracked_cancellations.clone()
    // }

    pub async fn active_profile(&self) -> Option<Arc<moss_user::profile::Profile<R>>> {
        self.profile_service.active_profile().await
    }
}
