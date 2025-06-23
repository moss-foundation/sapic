use derive_more::Deref;
use hcl::{Block, Body};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{
    primitives::{EntryClass, EntryProtocol, HttpMethod},
    types::configuration::common::ConfigurationMetadata,
};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HttpRequestParts {
    pub method: HttpMethod,
}

// ########################################################
// ###                      Request                     ###
// ########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HttpRequestItemConfiguration {
    pub request_parts: HttpRequestParts,
}

impl HttpRequestItemConfiguration {
    pub fn to_hcl(&self) -> Block {
        Block::builder("request")
            .add_label("http")
            .add_attribute(("method", self.request_parts.method.to_string()))
            .build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestItemConfigurationModel {
    Http(HttpRequestItemConfiguration),
}

impl RequestItemConfigurationModel {
    pub fn to_hcl(&self) -> Block {
        match self {
            RequestItemConfigurationModel::Http(model) => model.to_hcl(),
        }
    }
}

// ########################################################
// ###                      Endpoint                    ###
// ########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HttpEndpointItemConfiguration {
    pub request_parts: HttpRequestParts,
}

impl HttpEndpointItemConfiguration {
    pub fn to_hcl(&self) -> Block {
        Block::builder("endpoint")
            .add_label("http")
            .add_attribute(("method", self.request_parts.method.to_string()))
            .build()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum EndpointItemConfigurationModel {
    Http(HttpEndpointItemConfiguration),
}

impl EndpointItemConfigurationModel {
    pub fn to_hcl(&self) -> Block {
        match self {
            EndpointItemConfigurationModel::Http(model) => model.to_hcl(),
        }
    }
}

// ########################################################
// ###                      Component                   ###
// ########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ComponentItemConfigurationModel {}

// ########################################################
// ###                      Schema                      ###
// ########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum SchemaItemConfigurationModel {}

// ########################################################
// ###                      Item                        ###
// ########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ItemConfigurationModel {
    Request(RequestItemConfigurationModel),
    Endpoint(EndpointItemConfigurationModel),
    Component(ComponentItemConfigurationModel),
    Schema(SchemaItemConfigurationModel),
}

impl ItemConfigurationModel {
    pub fn to_hcl(&self) -> Block {
        match self {
            ItemConfigurationModel::Request(model) => model.to_hcl(),
            ItemConfigurationModel::Endpoint(model) => model.to_hcl(),
            ItemConfigurationModel::Component(_) => unimplemented!(),
            ItemConfigurationModel::Schema(_) => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Deref, TS)]
#[serde(rename_all = "camelCase")]
pub struct CompositeItemConfigurationModel {
    pub metadata: ConfigurationMetadata,
    #[serde(flatten)]
    #[deref]
    pub inner: ItemConfigurationModel,
}

impl CompositeItemConfigurationModel {
    pub fn to_hcl(&self) -> Body {
        Body::builder()
            .add_block(self.metadata.to_hcl())
            .add_block(self.inner.to_hcl())
            .build()
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
        match &self.inner {
            ItemConfigurationModel::Request(model) => match model {
                RequestItemConfigurationModel::Http(model) => {
                    Some(EntryProtocol::from(&model.request_parts.method))
                }
            },
            ItemConfigurationModel::Endpoint(_) => Some(EntryProtocol::Get), // FIXME: hardcoded for now
            _ => None,
        }
    }
}
