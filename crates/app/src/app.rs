mod services;

pub mod builder;
pub mod command;
pub mod operations;
pub mod windows;

use derive_more::Deref;
use joinerror::ResultExt;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_fs::FileSystem;
use moss_keyring::KeyringClient;
use moss_storage2::KvStorage;
use moss_text::ReadOnlyStr;
use rustc_hash::FxHashMap;
use sapic_base::workspace::types::primitives::WorkspaceId;
use sapic_main::{MainWindow, workspace::RuntimeWorkspace, workspace_ops::MainWindowWorkspaceOps};
use sapic_onboarding::OnboardingWindow;
use sapic_system::{
    application::extensions_service::ExtensionsApiService,
    configuration::configuration_registry::RegisterConfigurationContribution,
    language::language_service::LanguageService,
    ports::{
        github_api::GitHubApiClient, gitlab_api::GitLabApiClient, server_api::ServerApiClient,
    },
    theme::theme_service::ThemeService,
    user::User,
    workspace::{
        workspace_edit_service::WorkspaceEditService, workspace_service::WorkspaceService,
    },
};
use sapic_welcome::{WelcomeWindow, workspace_ops::WelcomeWindowWorkspaceOps};
use sapic_window::OldSapicWindowBuilder;
use sapic_window2::AppWindowApi;
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};
use tauri::{AppHandle as TauriAppHandle, Runtime as TauriRuntime};

use crate::{
    command::CommandCallback, services::extension_service::ExtensionService, windows::WindowManager,
};

inventory::submit! {
    RegisterConfigurationContribution(include_str!(concat!(env!("OUT_DIR"), "/configurations.json")))
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

// We have to make this public so that it can be accessed in integration-tests
pub struct AppServices {
    pub workspace_service: Arc<WorkspaceService>,
    pub workspace_edit_service: Arc<WorkspaceEditService>,
    pub theme_service: Arc<ThemeService>,
    pub language_service: Arc<LanguageService>,
    pub extension_api_service: Arc<ExtensionsApiService>,
}

#[derive(Deref)]
pub struct App<R: AppRuntime> {
    #[deref]
    pub(crate) tao_handle: TauriAppHandle<R::EventLoop>,
    pub(crate) fs: Arc<dyn FileSystem>,
    pub(crate) keyring: Arc<dyn KeyringClient>,
    pub(crate) storage: Arc<dyn KvStorage>,
    pub(crate) server_api_client: Arc<dyn ServerApiClient>,
    pub(crate) github_api_client: Arc<dyn GitHubApiClient>,
    pub(crate) gitlab_api_client: Arc<dyn GitLabApiClient>,

    pub(crate) user: Arc<dyn User>,
    pub(crate) commands: AppCommands<R::EventLoop>,
    pub(crate) services: AppServices,
    pub(crate) windows: WindowManager<R>,

    #[allow(unused)]
    pub(crate) extension_service: ExtensionService<R>,
}

impl<R: AppRuntime> App<R> {
    pub fn handle(&self) -> TauriAppHandle<R::EventLoop> {
        self.tao_handle.clone()
    }

    pub async fn window(&self, label: &str) -> Option<Box<dyn AppWindowApi>> {
        self.windows
            .window(label)
            .await
            .map(|window| Box::new(window) as Box<dyn AppWindowApi>)
    }

    pub async fn ensure_welcome(&self, delegate: &AppDelegate<R>) -> joinerror::Result<()> {
        let maybe_welcome_window = self.windows.welcome_window().await;
        if let Some(welcome_window) = maybe_welcome_window {
            if let Err(err) = welcome_window.set_focus() {
                tracing::warn!("Failed to set focus to welcome window: {}", err);
            }
            return Ok(());
        } else {
            let workspace_ops = WelcomeWindowWorkspaceOps::new(
                self.services.workspace_service.clone(),
                self.services.workspace_edit_service.clone(),
            );
            let welcome_window = self
                .windows
                .create_welcome_window(delegate, workspace_ops)
                .await?;
            if let Err(err) = welcome_window.set_focus() {
                tracing::warn!("Failed to set focus to welcome window: {}", err);
            }

            return Ok(());
        }
    }

    // FIXME: Not sure if onboarding should use the same approach as welcome
    // Since it's likely only be executed once
    // But I'll keep the same approach for now
    pub async fn ensure_onboarding(&self, delegate: &AppDelegate<R>) -> joinerror::Result<()> {
        let maybe_onboarding_window = self.windows.onboarding_window().await;
        if let Some(onboarding_window) = maybe_onboarding_window {
            if let Err(err) = onboarding_window.set_focus() {
                tracing::warn!("Failed to set focus to onboarding window: {}", err);
            }
            return Ok(());
        } else {
            let onboarding_window = self.windows.create_onboarding_window(delegate).await?;
            if let Err(err) = onboarding_window.set_focus() {
                tracing::warn!("Failed to set focus to onboarding window: {}", err);
            }
            return Ok(());
        }
    }

    pub async fn ensure_main_for_workspace(
        &self,
        ctx: &R::AsyncContext,
        delegate: &AppDelegate<R>,
        workspace_id: WorkspaceId,
    ) -> joinerror::Result<()> {
        let maybe_main_window = self
            .windows
            .main_window_by_workspace_id(&workspace_id)
            .await;

        if let Some(main_window) = maybe_main_window {
            if let Err(err) = main_window.handle.set_focus() {
                tracing::warn!("Failed to set focus to main window: {}", err);
            }

            return Ok(());
        }

        let abs_path = delegate
            .workspaces_dir()
            .join(workspace_id.to_string())
            .into();

        joinerror::ResultExt::join_err::<()>(
            self.storage.add_workspace(workspace_id.inner()).await,
            "failed to add workspace to storage",
        )?;

        let workspace = Arc::new(RuntimeWorkspace::new(
            workspace_id.clone(),
            abs_path,
            self.services.workspace_edit_service.clone(),
        ));
        let old_window = OldSapicWindowBuilder::new(
            self.user.clone(),
            self.fs.clone(),
            self.storage.clone(),
            self.keyring.clone(),
            self.server_api_client.clone(),
            self.github_api_client.clone(),
            self.gitlab_api_client.clone(),
            workspace_id.clone(),
            self.services.workspace_service.clone(),
        )
        .build(ctx, delegate)
        .await?;

        let workspace_ops = MainWindowWorkspaceOps::new(self.services.workspace_service.clone());

        let main_window = self
            .windows
            .create_main_window(delegate, old_window, workspace, workspace_ops)
            .await?;

        if let Err(err) = main_window.handle.set_focus() {
            tracing::warn!("Failed to set focus to main window: {}", err);
        }

        Ok(())
    }

    pub async fn swap_main_window_workspace(
        &self,
        ctx: &R::AsyncContext,
        delegate: &AppDelegate<R>,
        workspace_id: WorkspaceId,
        label: &str,
    ) -> joinerror::Result<()> {
        let abs_path = delegate
            .workspaces_dir()
            .join(workspace_id.to_string())
            .into();

        // HACK: We're forced to add the store here instead of in the window creation
        // function because projects are currently loaded right away when an old workspace
        // is created. In the new workspace, since we won't be storing the list of projects on
        // the backend, this problem won't exist (and in the worst case, we can load the projects lazily).

        joinerror::ResultExt::join_err::<()>(
            self.storage.add_workspace(workspace_id.inner()).await,
            "failed to add workspace to storage",
        )?;

        let workspace = Arc::new(RuntimeWorkspace::new(
            workspace_id.clone(),
            abs_path,
            self.services.workspace_edit_service.clone(),
        ));

        let old_window = OldSapicWindowBuilder::new(
            self.user.clone(),
            self.fs.clone(),
            self.storage.clone(),
            self.keyring.clone(),
            self.server_api_client.clone(),
            self.github_api_client.clone(),
            self.gitlab_api_client.clone(),
            workspace_id.clone(),
            self.services.workspace_service.clone(),
        )
        .build(ctx, delegate)
        .await?;

        self.windows
            .swap_main_window_workspace(ctx, label, workspace, old_window)
            .await
    }

    pub async fn main_window(&self, label: &str) -> Option<MainWindow<R>> {
        self.windows.main_window(label).await
    }

    pub async fn close_main_window(
        &self,
        ctx: &R::AsyncContext,
        label: &str,
    ) -> joinerror::Result<()> {
        let closed_window = self
            .windows
            .close_main_window(ctx, label)
            .await
            .join_err::<()>("failed to close main window")?;

        if let Some(closed_window) = closed_window {
            joinerror::ResultExt::join_err::<()>(
                self.storage
                    .remove_workspace(closed_window.workspace().id().inner())
                    .await,
                "failed to remove workspace from storage",
            )?;
        }

        Ok(())
    }

    pub async fn welcome_window(&self) -> Option<WelcomeWindow<R>> {
        self.windows.welcome_window().await
    }

    pub async fn close_welcome_window(&self) -> joinerror::Result<()> {
        self.windows.close_welcome_window().await
    }

    pub async fn onboarding_window(&self) -> Option<OnboardingWindow<R>> {
        self.windows.onboarding_window().await
    }

    pub async fn close_onboarding_window(&self) -> joinerror::Result<()> {
        self.windows.close_onboarding_window().await
    }

    pub fn command(&self, id: &ReadOnlyStr) -> Option<CommandCallback<R::EventLoop>> {
        self.commands.get(id).map(|cmd| Arc::clone(cmd))
    }

    #[cfg(feature = "integration-tests")]
    pub fn services(&self) -> &AppServices {
        &self.services
    }
}

// TEST UPDATE
