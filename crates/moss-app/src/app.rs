use derive_more::Deref;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, context::Canceller};
use moss_logging::session;
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

use crate::{
    ActiveWorkspace,
    command::CommandCallback,
    configuration::ConfigurationService,
    extension::ExtensionService,
    locale::LocaleService,
    logging::LogService,
    models::{
        primitives::SessionId,
        types::{ColorThemeInfo, LocaleInfo},
    },
    profile::ProfileService,
    session::SessionService,
    storage::StorageService,
    theme::ThemeService,
    workspace::WorkspaceService,
};

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

pub struct OnAppReadyOptions {
    pub restore_last_workspace: bool,
}

#[derive(Deref)]
pub struct App<R: AppRuntime> {
    #[deref]
    pub(super) app_handle: AppHandle<R::EventLoop>,
    pub(super) app_dir: PathBuf,
    pub(super) commands: AppCommands<R::EventLoop>,
    pub(super) preferences: AppPreferences,

    #[allow(unused)]
    pub(super) session_service: SessionService,
    pub(super) log_service: LogService<R>,
    pub(super) storage_service: Arc<StorageService<R>>,
    pub(super) workspace_service: WorkspaceService<R>,
    pub(super) locale_service: LocaleService,
    pub(super) theme_service: ThemeService,
    pub(super) profile_service: ProfileService<R>,
    pub(super) configuration_service: ConfigurationService,
    pub(super) extension_service: ExtensionService<R>,

    // Store cancellers by the id of API requests
    pub(super) tracked_cancellations: Arc<RwLock<HashMap<String, Canceller>>>,
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

    pub async fn on_app_ready(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        options: OnAppReadyOptions,
    ) -> joinerror::Result<()> {
        let profile = self.profile_service.activate_profile().await?;

        if !options.restore_last_workspace {
            return Ok(());
        }

        match self.storage_service.get_last_active_workspace(ctx).await {
            Ok(id) => {
                self.workspace_service
                    .activate_workspace(ctx, app_delegate, &id, profile)
                    .await?;
            }
            Err(err) => {
                session::warn!(format!(
                    "failed to restore last active workspace: {}",
                    err.to_string()
                ));
            }
        };

        Ok(())
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

    pub async fn active_profile(&self) -> Option<Arc<moss_user::profile::Profile<R>>> {
        self.profile_service.active_profile().await
    }
}
