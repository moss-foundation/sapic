use crate::{
    command::CommandCallback,
    models::types::{ColorThemeInfo, LocaleInfo},
    services::{session_service::SessionId, workspace_service::ActiveWorkspace, *},
};
use derive_more::Deref;
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{AppRuntime, context::Canceller};
use moss_fs::{FileSystem, model_registry::GlobalModelRegistry};
use moss_git_hosting_provider::{github::client::GitHubClient, gitlab::client::GitLabClient};
use moss_text::ReadOnlyStr;
use rustc_hash::FxHashMap;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::RwLock;

pub struct AppPreferences {
    pub theme: RwLock<Option<ColorThemeInfo>>,
    pub locale: RwLock<Option<LocaleInfo>>,
}

pub struct AppDefaults {
    pub theme: ColorThemeInfo,
    pub locale: LocaleInfo,
}

pub struct AppCommands<R: TauriRuntime>(FxHashMap<ReadOnlyStr, CommandCallback<R>>);

impl<R: TauriRuntime> Default for AppCommands<R> {
    fn default() -> Self {
        Self(FxHashMap::default())
    }
}

impl<R: TauriRuntime> Deref for AppCommands<R> {
    type Target = FxHashMap<ReadOnlyStr, CommandCallback<R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: TauriRuntime> DerefMut for AppCommands<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Deref)]
pub struct App<R: AppRuntime> {
    #[deref]
    pub(super) app_handle: AppHandle<R::EventLoop>,
    pub(super) app_dir: PathBuf,
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) models: Arc<GlobalModelRegistry>,
    pub(super) commands: AppCommands<R::EventLoop>,
    pub(super) preferences: AppPreferences,
    pub(super) defaults: AppDefaults,

    #[allow(unused)]
    pub(super) session_service: SessionService,
    pub(super) log_service: LogService<R>,
    pub(super) storage_service: Arc<StorageService<R>>,
    pub(super) workspace_service: WorkspaceService<R>,
    pub(super) locale_service: LocaleService,
    pub(super) theme_service: ThemeService,

    // Store cancellers by the id of API requests
    pub(super) tracked_cancellations: Arc<RwLock<HashMap<String, Canceller>>>,
    // TODO: This is also might be better to be a service
    pub(super) activity_indicator: ActivityIndicator<R::EventLoop>,

    // TODO: Refine the management of git provider clients
    pub(super) github_client: Arc<GitHubClient>,
    pub(super) gitlab_client: Arc<GitLabClient>,
}

impl<R: AppRuntime> App<R> {
    pub fn app_dir(&self) -> &Path {
        &self.app_dir
    }

    pub fn session_id(&self) -> &SessionId {
        self.session_service.session_id()
    }

    pub fn handle(&self) -> AppHandle<R::EventLoop> {
        self.app_handle.clone()
    }

    pub fn preferences(&self) -> &AppPreferences {
        &self.preferences
    }

    pub fn defaults(&self) -> &AppDefaults {
        &self.defaults
    }

    pub async fn workspace(&self) -> Option<Arc<ActiveWorkspace<R>>> {
        self.workspace_service.workspace().await
    }

    pub fn command(&self, id: &ReadOnlyStr) -> Option<CommandCallback<R::EventLoop>> {
        self.commands.get(id).map(|cmd| Arc::clone(cmd))
    }

    pub async fn track_cancellation(&self, request_id: &str, canceller: Canceller) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.insert(request_id.to_string(), canceller);
    }

    pub async fn release_cancellation(&self, request_id: &str) -> () {
        let mut write = self.tracked_cancellations.write().await;

        write.remove(request_id);
    }
}

#[cfg(feature = "integration-tests")]
impl<R: AppRuntime> App<R> {
    pub fn db(&self) -> Arc<dyn moss_storage::GlobalStorage<R::AsyncContext>> {
        self.storage_service.storage()
    }

    pub fn cancellation_map(&self) -> Arc<RwLock<HashMap<String, Canceller>>> {
        self.tracked_cancellations.clone()
    }
}
