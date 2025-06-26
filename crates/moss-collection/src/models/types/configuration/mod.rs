mod common;
mod component;
mod endpoint;
mod request;
mod schema;

pub mod docschema;

pub use common::*;
pub use component::*;
pub use endpoint::*;
pub use request::*;
pub use schema::*;

use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use uuid::Uuid;

use crate::models::{
    primitives::EntryClass,
    types::{EntryProtocol, configuration::docschema::ItemRequestConfiguration},
};

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum DirConfigurationModel {
    Request(DirRequestConfigurationModel),
    Endpoint(EndpointDirConfigurationModel),
    Component(ComponentDirConfigurationModel),
    Schema(SchemaDirConfigurationModel),
}

#[derive(Debug, Deref, DerefMut, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositeDirConfigurationModel {
    pub metadata: ConfigurationMetadata,
    #[serde(flatten)]
    #[deref]
    #[deref_mut]
    pub inner: DirConfigurationModel,
}

// TODO: remove this
impl Default for CompositeDirConfigurationModel {
    fn default() -> Self {
        Self {
            metadata: ConfigurationMetadata { id: Uuid::new_v4() },
            inner: DirConfigurationModel::Request(DirRequestConfigurationModel::Http(
                DirHttpConfigurationModel {},
            )),
        }
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

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ItemConfigurationModel {
    Request(ItemRequestConfigurationModel),
    Endpoint(EndpointItemConfigurationModel),
    Component(ComponentItemConfigurationModel),
    Schema(SchemaItemConfigurationModel),
}

#[derive(Debug, Clone, Serialize, Deserialize, Deref, TS)]
#[serde(rename_all = "camelCase")]
pub struct CompositeItemConfigurationModel {
    pub metadata: ConfigurationMetadata,
    #[serde(flatten)]
    #[deref]
    pub inner: ItemConfigurationModel,
}

impl TryFrom<ItemRequestConfiguration> for CompositeItemConfigurationModel {
    type Error = anyhow::Error;

    fn try_from(value: ItemRequestConfiguration) -> Result<Self, Self::Error> {
        // if let Some(http_request_parts) = value.http_request_parts {
        //     let configuration = ItemHttpRequestConfiguration {
        //         request_parts: HttpRequestParts::from(http_request_parts),
        //     };
        //     let model = ItemRequestConfigurationModel::Http(configuration);

        //     return Ok(Self {
        //         metadata: ConfigurationMetadata::from(value.metadata),
        //         inner: ItemConfigurationModel::Request(model),
        //     });
        // }

        todo!()

        // let metadata = ConfigurationMetadata::from(value.metadata);
        // if let Some(request_parts) = value.request_parts.get("http") {}
        // if request_parts.

        // Self {
        //     metadata: ConfigurationMetadata::from(value.metadata),
        //     // inner: ItemConfigurationModel::Request(ItemRequestConfigurationModel::Http(
        //     //     ItemHttpRequestConfiguration {
        //     //         request_parts: value.request_parts,
        //     //     },
        //     // )),
        // }
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
                ItemRequestConfigurationModel::Http(model) => {
                    Some(EntryProtocol::from(&model.request_parts.method))
                }
            },
            ItemConfigurationModel::Endpoint(_) => Some(EntryProtocol::Get),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConfigurationModel {
    Item(CompositeItemConfigurationModel),
    Dir(CompositeDirConfigurationModel),
}

impl ConfigurationModel {
    pub fn id(&self) -> Uuid {
        match self {
            ConfigurationModel::Item(item) => item.metadata.id,
            ConfigurationModel::Dir(dir) => dir.metadata.id,
        }
    }
}
