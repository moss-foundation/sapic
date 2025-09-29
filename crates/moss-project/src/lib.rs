pub mod api;
pub mod builder;
mod config;
mod edit;
pub mod git;
mod manifest;
pub mod models;
pub mod project;
pub mod vcs;
mod worktree;

mod set_icon;
#[cfg(feature = "integration-tests")]
pub mod storage;
#[cfg(not(feature = "integration-tests"))]
mod storage;

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
pub use builder::ProjectBuilder;
use derive_more::Deref;
use moss_addon::{ExtensionInfo, ExtensionPoint};
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_contrib::ContributionKey;
pub use project::{Project, ProjectModifyParams};
use serde_json::Value as JsonValue;
use tokio::sync::RwLock;

pub use moss_contrib::include::{IncludeHttpHeaders, IncludeResourceStatuses};

inventory::submit! {
    IncludeResourceStatuses(include_str!(concat!(env!("OUT_DIR"), "/resourceStatuses.json")))
}

inventory::submit! {
    IncludeHttpHeaders(include_str!(concat!(env!("OUT_DIR"), "/httpHeaders.json")))
}

pub struct ResourceParamsExtensionPoint {}

impl ResourceParamsExtensionPoint {
    pub fn new() -> Self {
        Self {}
    }
}

const RESOURCE_PARAMS_KEY: ContributionKey = ContributionKey::new("resource_params");

#[async_trait]
impl<R: AppRuntime> ExtensionPoint<R> for ResourceParamsExtensionPoint {
    fn key(&self) -> ContributionKey {
        RESOURCE_PARAMS_KEY
    }

    async fn handle(
        &self,
        app_delegate: &AppDelegate<R>,
        info: &ExtensionInfo,
        contribution: JsonValue,
    ) -> joinerror::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ResourceStatusItem {
    pub name: String,
    pub description: Option<String>,
    pub color: String,
}

#[derive(Debug, Clone)]
pub struct ResourceHeaderItem {
    pub name: String,
    pub description: Option<String>,
    pub value: String,
    pub protected: bool,
    pub disabled: bool,
}

#[async_trait]
pub trait ResourceParamsRegistry: Send + Sync {
    async fn statuses(&self) -> Vec<ResourceStatusItem>;
    async fn headers(&self) -> HashMap<String, ResourceHeaderItem>;
}

pub struct AppResourceParamsRegistry {
    statuses: RwLock<Vec<ResourceStatusItem>>,
    headers: RwLock<HashMap<String, ResourceHeaderItem>>,
}

impl AppResourceParamsRegistry {
    pub fn new() -> Self {
        Self {
            statuses: RwLock::new(vec![]),
            headers: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl ResourceParamsRegistry for AppResourceParamsRegistry {
    async fn statuses(&self) -> Vec<ResourceStatusItem> {
        self.statuses.read().await.clone()
    }

    async fn headers(&self) -> HashMap<String, ResourceHeaderItem> {
        self.headers.read().await.clone()
    }
}

#[derive(Deref, Clone)]
struct GlobalResourceParamsRegistry(Arc<dyn ResourceParamsRegistry>);

impl dyn ResourceParamsRegistry {
    pub fn global<R: AppRuntime>(delegate: &AppDelegate<R>) -> Arc<dyn ResourceParamsRegistry> {
        delegate.global::<GlobalResourceParamsRegistry>().0.clone()
    }

    pub fn set_global<R: AppRuntime>(
        delegate: &AppDelegate<R>,
        v: Arc<dyn ResourceParamsRegistry>,
    ) {
        delegate.set_global(GlobalResourceParamsRegistry(v));
    }
}

pub mod constants {
    pub const ITEM_CONFIG_FILENAME: &str = "config.sap";
    pub const DIR_CONFIG_FILENAME: &str = "config-folder.sap";
}

mod defaults {
    pub(crate) const DEFAULT_PROJECT_NAME: &str = "New Project";
}

pub mod dirs {
    pub const RESOURCES_DIR: &str = "resources";
    pub const ENVIRONMENTS_DIR: &str = "environments";
    pub const ASSETS_DIR: &str = "assets";
}

pub mod errors {
    use joinerror::error::ErrorMarker;

    pub struct ErrorInvalidInput;
    impl ErrorMarker for ErrorInvalidInput {
        const MESSAGE: &'static str = "invalid_input";
    }

    pub struct ErrorInvalidKind;
    impl ErrorMarker for ErrorInvalidKind {
        const MESSAGE: &'static str = "invalid_kind";
    }

    pub struct ErrorAlreadyExists;
    impl ErrorMarker for ErrorAlreadyExists {
        const MESSAGE: &'static str = "already_exists";
    }

    pub struct ErrorNotFound;
    impl ErrorMarker for ErrorNotFound {
        const MESSAGE: &'static str = "not_found";
    }

    pub struct ErrorIo;
    impl ErrorMarker for ErrorIo {
        const MESSAGE: &'static str = "io";
    }

    pub struct ErrorInternal;
    impl ErrorMarker for ErrorInternal {
        const MESSAGE: &'static str = "internal";
    }

    pub struct ErrorUnknown;
    impl ErrorMarker for ErrorUnknown {
        const MESSAGE: &'static str = "unknown";
    }
}
