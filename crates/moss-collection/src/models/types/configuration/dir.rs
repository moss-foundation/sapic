use std::ops::Deref;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{primitives::EntryClass, types::configuration::ConfigurationMetadata};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HttpDirConfigurationModel {}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestDirConfigurationModel {
    Http(HttpDirConfigurationModel),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum DirConfigurationModel {
    Request(RequestDirConfigurationModel),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompositeDirConfigurationModel {
    pub metadata: ConfigurationMetadata,
    pub inner: DirConfigurationModel,
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
            // ItemConfigurationModel::Endpoint(_) => Classification::Endpoint,
            // ItemConfigurationModel::Component(_) => Classification::Component,
            // ItemConfigurationModel::Schema(_) => Classification::Schema,
        }
    }
}
