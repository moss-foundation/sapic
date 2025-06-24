mod component;
mod endpoint;
mod request;
mod schema;

pub use component::*;
pub use endpoint::*;
pub use request::*;
pub use schema::*;

use std::ops::Deref;

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::models::{
    primitives::{EntryClass, EntryKind},
    types::configuration::common::ConfigurationMetadata,
};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum DirConfigurationModel {
    Request(RequestDirConfigurationModel),
    Endpoint(EndpointDirConfigurationModel),
    Component(ComponentDirConfigurationModel),
    Schema(SchemaDirConfigurationModel),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositeDirConfigurationModel {
    pub metadata: ConfigurationMetadata,
    #[serde(flatten)]
    pub inner: DirConfigurationModel,
}

// TODO: remove this
impl Default for CompositeDirConfigurationModel {
    fn default() -> Self {
        Self {
            metadata: ConfigurationMetadata { id: Uuid::new_v4() },
            inner: DirConfigurationModel::Request(RequestDirConfigurationModel::Http(
                HttpDirConfigurationModel {},
            )),
        }
    }
}

impl Deref for CompositeDirConfigurationModel {
    type Target = DirConfigurationModel;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl CompositeDirConfigurationModel {
    pub fn classification(&self) -> EntryClass {
        match self.inner {
            DirConfigurationModel::Request(_) => EntryClass::Request,
            DirConfigurationModel::Endpoint(_) => EntryClass::Endpoint,
            DirConfigurationModel::Component(_) => EntryClass::Component,
            DirConfigurationModel::Schema(_) => EntryClass::Schema,
        }
    }
}
