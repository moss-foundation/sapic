use std::ops::Deref;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{primitives::EntryClass, types::configuration::common::ConfigurationMetadata};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum EndpointDirConfigurationModel {}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ComponentDirConfigurationModel {}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum SchemaDirConfigurationModel {}

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
// #[serde(untagged)]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum DirConfigurationModel {
    Request(RequestDirConfigurationModel),
    Endpoint(EndpointDirConfigurationModel),
    Component(ComponentDirConfigurationModel),
    Schema(SchemaDirConfigurationModel),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CompositeDirConfigurationModel {
    pub metadata: ConfigurationMetadata,
    #[serde(flatten)]
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
            DirConfigurationModel::Endpoint(_) => EntryClass::Endpoint,
            DirConfigurationModel::Component(_) => EntryClass::Component,
            DirConfigurationModel::Schema(_) => EntryClass::Schema,
        }
    }
}
