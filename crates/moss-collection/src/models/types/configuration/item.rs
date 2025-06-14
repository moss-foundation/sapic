use std::ops::Deref;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{
    primitives::{EntryClass, EntryProtocol},
    types::configuration::ConfigurationMetadata,
};

// #[derive(Clone, Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "operations.ts")]
// pub enum CreateRequestProtocolSpecificPayload {
//     Http {
//         method: HttpMethod,
//         query_params: Vec<QueryParamItem>,
//         path_params: Vec<PathParamItem>,
//         headers: Vec<HeaderParamItem>,
//         #[ts(optional)]
//         body: Option<RequestBody>,
//     },
// }

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestItemConfigurationModel {}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum EndpointItemConfigurationModel {}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ComponentItemConfigurationModel {}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum SchemaItemConfigurationModel {}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ItemConfigurationModel {
    Request(RequestItemConfigurationModel),
    Endpoint(EndpointItemConfigurationModel),
    Component(ComponentItemConfigurationModel),
    Schema(SchemaItemConfigurationModel),
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub struct CompositeItemConfigurationModel {
    pub metadata: ConfigurationMetadata,
    pub inner: ItemConfigurationModel,
}

impl Deref for CompositeItemConfigurationModel {
    type Target = ItemConfigurationModel;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl CompositeItemConfigurationModel {
    pub fn classification(&self) -> EntryClass {
        match self.inner {
            ItemConfigurationModel::Request(_) => EntryClass::Request,
            ItemConfigurationModel::Endpoint(_) => EntryClass::Endpoint,
            ItemConfigurationModel::Component(_) => EntryClass::Component,
            ItemConfigurationModel::Schema(_) => EntryClass::Schema,
        }
    }

    pub fn protocol(&self) -> Option<EntryProtocol> {
        match self.inner {
            ItemConfigurationModel::Request(_) => Some(EntryProtocol::Get), // FIXME: hardcoded for now
            ItemConfigurationModel::Endpoint(_) => Some(EntryProtocol::Get), // FIXME: hardcoded for now
            _ => None,
        }
    }
}
