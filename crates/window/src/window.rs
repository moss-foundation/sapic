pub mod builder;
mod configuration;
mod constants;
pub mod internal;
mod profile;
mod session;
pub mod workspace;

pub mod storage; // HACK: should be removed when SQLite migration is complete

pub mod api;
pub mod types;

use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, context::Canceller, subscription::Event};
use moss_user::profile::Profile;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    internal::events::OnDidChangeProfile,
    profile::{ProfileDetails, ProfileService},
    types::primitives::WorkspaceId,
    workspace::{ActiveWorkspace, WorkspaceDetails, WorkspaceService},
};

// TODO: needs to be cleaned up later (what we need to keep here?)
pub mod dirs {
    pub const WORKSPACES_DIR: &str = "workspaces";
    pub const GLOBALS_DIR: &str = "globals";
    pub const PROFILES_DIR: &str = "profiles";
    pub const TMP_DIR: &str = "tmp";
}

pub struct Window<R: AppRuntime> {
    pub(crate) label: String,
    pub(crate) workspace_service: WorkspaceService<R>,
    pub(crate) profile_service: ProfileService<R>,
    // pub(crate) storage_service: Arc<StorageService<R>>, // HACK: should be removed when SQLite migration is complete

    // Store cancellers by the id of API requests
    pub(crate) tracked_cancellations: Arc<RwLock<HashMap<String, Canceller>>>,

    pub(crate) on_did_change_profile_event: Event<OnDidChangeProfile>,
}

impl<R: AppRuntime> Window<R> {
    pub fn label(&self) -> &str {
        &self.label
    }

    // HACK: tempoary solution to mare App config service work
    pub fn on_did_change_profile_event(&self) -> &Event<OnDidChangeProfile> {
        &self.on_did_change_profile_event
    }

    pub async fn active_profile(&self) -> Option<Arc<Profile<R>>> {
        self.profile_service.active_profile().await
    }

    pub async fn activate_profile(&self) -> joinerror::Result<Arc<Profile<R>>> {
        self.profile_service.activate_profile().await
    }

    pub async fn profile_details(&self) -> Option<ProfileDetails> {
        let profile = if let Some(profile) = self.profile_service.active_profile().await {
            profile
        } else {
            return None;
        };

        self.profile_service.profile_details(&profile.id()).await
    }

    pub async fn workspace(&self) -> Option<Arc<ActiveWorkspace<R>>> {
        self.workspace_service.workspace().await
    }

    pub async fn workspace_details(&self) -> Option<WorkspaceDetails> {
        let workspace = if let Some(workspace) = self.workspace().await {
            workspace
        } else {
            return None;
        };

        self.workspace_service
            .workspace_details(&workspace.id())
            .await
    }

    pub async fn activate_workspace(
        &self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        id: &WorkspaceId,
        active_profile: Arc<Profile<R>>,
    ) -> joinerror::Result<()> {
        self.workspace_service
            .activate_workspace(ctx, app_delegate, id, active_profile)
            .await?;

        Ok(())
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
