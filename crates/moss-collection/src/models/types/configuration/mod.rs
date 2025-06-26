mod common;
mod component;
mod endpoint;
mod request;
mod schema;

mod docschema;

pub use common::*;
pub use component::*;
pub use endpoint::*;
pub use request::*;
pub use schema::*;

use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use uuid::Uuid;

use crate::models::{primitives::EntryClass, types::EntryProtocol};

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum DirConfigurationModel {
    Request(RequestDirConfigurationModel),
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
            inner: DirConfigurationModel::Request(RequestDirConfigurationModel::Http(
                HttpDirConfigurationModel {},
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
    Request(RequestItemConfigurationModel),
    Endpoint(EndpointItemConfigurationModel),
    Component(ComponentItemConfigurationModel),
    Schema(SchemaItemConfigurationModel),
}

// impl ItemConfigurationModel {
//     pub fn to_hcl(&self) -> Block {
//         match self {
//             ItemConfigurationModel::Request(model) => model.to_hcl(),
//             ItemConfigurationModel::Endpoint(model) => model.to_hcl(),
//             ItemConfigurationModel::Component(_) => unimplemented!(),
//             ItemConfigurationModel::Schema(_) => unimplemented!(),
//         }
//     }

//     pub fn from_hcl(block: &Block) -> Result<Self> {
//         match block.identifier.as_str() {
//             "request" => {
//                 let request_model = RequestItemConfigurationModel::from_hcl(block)?;
//                 Ok(ItemConfigurationModel::Request(request_model))
//             }
//             "endpoint" => {
//                 let endpoint_model = EndpointItemConfigurationModel::from_hcl(block)?;
//                 Ok(ItemConfigurationModel::Endpoint(endpoint_model))
//             }
//             "component" => {
//                 unimplemented!("Component configuration not implemented yet")
//             }
//             "schema" => {
//                 unimplemented!("Schema configuration not implemented yet")
//             }
//             _ => Err(anyhow::anyhow!("Unknown block type: {}", block.identifier)),
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, Deref, TS)]
#[serde(rename_all = "camelCase")]
pub struct CompositeItemConfigurationModel {
    pub metadata: ConfigurationMetadata,
    #[serde(flatten)]
    #[deref]
    pub inner: ItemConfigurationModel,
}

// impl CompositeItemConfigurationModel {
//     pub fn to_hcl(&self) -> Body {
//         Body::builder()
//             .add_block(self.metadata.to_hcl())
//             .add_block(self.inner.to_hcl())
//             .build()
//     }

//     pub fn from_hcl(body: Body) -> Result<Self> {
//         let mut metadata = None;
//         let mut inner = None;

//         for block in body.blocks() {
//             match block.identifier.as_str() {
//                 "metadata" => {
//                     metadata = Some(ConfigurationMetadata::from_hcl(block)?);
//                 }
//                 "request" | "endpoint" | "component" | "schema" => {
//                     inner = Some(ItemConfigurationModel::from_hcl(block)?);
//                 }
//                 _ => {}
//             }
//         }

//         Ok(Self {
//             metadata: metadata.ok_or_else(|| anyhow::anyhow!("Missing metadata block"))?,
//             inner: inner.ok_or_else(|| anyhow::anyhow!("Missing configuration block"))?,
//         })
//     }
// }

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
