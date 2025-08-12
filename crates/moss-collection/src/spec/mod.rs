// mod component;
// mod endpoint;
// mod request;
// mod schema;

// pub use component::*;
// pub use endpoint::*;
// pub use request::*;
// pub use schema::*;

use hcl::Expression;
use indexmap::IndexMap;
use moss_hcl::{Block, LabeledBlock, Object};
use serde::{Deserialize, Serialize};

use crate::models::primitives::{EntryClass, EntryId, EntryProtocol, HeaderId};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct UrlDetails {
//     pub protocol: EntryProtocol,
//     pub raw: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum UrlParts {
//     Get(Block<UrlDetails>),
//     Post(Block<UrlDetails>),
//     Put(Block<UrlDetails>),
//     Delete(Block<UrlDetails>),
// }

// impl UrlParts {
//     pub fn protocol(&self) -> Option<EntryProtocol> {
//         match self {
//             UrlParts::Get(_) => Some(EntryProtocol::Get),
//             UrlParts::Post(_) => Some(EntryProtocol::Post),
//             UrlParts::Put(_) => Some(EntryProtocol::Put),
//             UrlParts::Delete(_) => Some(EntryProtocol::Delete),
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct HeaderParamSpec {
//     pub name: String,
//     pub value: Expression,
//     pub disabled: bool,
//     pub description: String,
//     pub options: HeaderParamOptions,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct HeaderParamOptions {
//     pub propagate: bool,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct EntryMetadataSpec {
//     pub id: EntryId,
//     #[serde(rename = "_class")]
//     pub class: EntryClass,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum ItemSpec {
//     Request(Block<ItemRequestSpec>),
//     Endpoint(Block<ItemEndpointSpec>),
//     Component(Block<ItemComponentSpec>),
//     Schema(Block<ItemSchemaSpec>),
// }

// impl ItemSpec {
//     pub fn id(&self) -> &EntryId {
//         match self {
//             ItemSpec::Request(block) => &block.metadata.id,
//             ItemSpec::Endpoint(block) => &block.metadata.id,
//             ItemSpec::Component(block) => &block.metadata.id,
//             ItemSpec::Schema(block) => &block.metadata.id,
//         }
//     }

//     pub fn classification(&self) -> EntryClass {
//         match self {
//             ItemSpec::Request(_) => EntryClass::Request,
//             ItemSpec::Endpoint(_) => EntryClass::Endpoint,
//             ItemSpec::Component(_) => EntryClass::Component,
//             ItemSpec::Schema(_) => EntryClass::Schema,
//         }
//     }

//     pub fn protocol(&self) -> Option<EntryProtocol> {
//         match self {
//             ItemSpec::Request(block) => block.url.protocol(),
//             ItemSpec::Endpoint(block) => block.url.protocol(),
//             ItemSpec::Component(_) => None,
//             ItemSpec::Schema(_) => None,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum DirSpec {
//     Request(Block<DirRequestSpec>),
//     Endpoint(Block<DirEndpointSpec>),
//     Component(Block<DirComponentSpec>),
//     Schema(Block<DirSchemaSpec>),
// }

// impl DirSpec {
//     pub fn id(&self) -> &EntryId {
//         match self {
//             DirSpec::Request(block) => &block.metadata.id,
//             DirSpec::Endpoint(block) => &block.metadata.id,
//             DirSpec::Component(block) => &block.metadata.id,
//             DirSpec::Schema(block) => &block.metadata.id,
//         }
//     }

//     pub fn classification(&self) -> EntryClass {
//         match self {
//             DirSpec::Request(_) => EntryClass::Request,
//             DirSpec::Endpoint(_) => EntryClass::Endpoint,
//             DirSpec::Component(_) => EntryClass::Component,
//             DirSpec::Schema(_) => EntryClass::Schema,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct EntryModel {
//     pub metadata: Block<EntryMetadataSpec>,

//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub url: Option<Block<UrlDetails>>,

//     #[serde(rename = "header")]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub headers: Option<LabeledBlock<IndexMap<HeaderId, HeaderParamSpec>>>,
// }

// impl From<(EntryId, EntryClass)> for EntryModel {
//     fn from((id, class): (EntryId, EntryClass)) -> Self {
//         Self {
//             metadata: Block::new(EntryMetadataSpec { id, class }),
//             url: None,
//             headers: None,
//         }
//     }
// }

// impl EntryModel {
//     pub fn id(&self) -> EntryId {
//         self.metadata.id.clone()
//     }

//     pub fn class(&self) -> EntryClass {
//         self.metadata.class.clone()
//     }

//     pub fn protocol(&self) -> Option<EntryProtocol> {
//         self.url.as_ref().map(|url| url.protocol.clone())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use hcl::{Expression as HclExpression, ser::LabeledBlock};
//     use indexmap::indexmap;

//     use super::*;

//     // #[test]
//     // fn test_dir() {
//     //     let config = DirRequestSpec {
//     //         metadata: Block::new(EntryMetadataSpec { id: EntryId::new() }),
//     //         headers: None,
//     //     };

//     //     let item = DirSpec::Request(Block::new(config));

//     //     let str = hcl::to_string(&item).unwrap();
//     //     println!("{}", str);

//     //     let new = hcl::from_str::<DirSpec>(&str).unwrap();

//     //     println!("{:?}", new);
//     // }

//     #[test]
//     fn test_item() {
//         let model = EntryModel {
//             metadata: Block::new(EntryMetadataSpec {
//                 id: EntryId::new(),
//                 class: EntryClass::Request,
//             }),
//             url: Some(Block::new(UrlDetails {
//                 protocol: EntryProtocol::Get,
//                 raw: "https://example.com".to_string(),
//             })),
//             headers: Some(LabeledBlock::new(indexmap! {
//                     HeaderId::new() => HeaderParamSpec {
//                         name: "Content-Type".to_string(),
//                         value: HclExpression::String("application/json".to_string()),
//                         disabled: false,
//                         description: "The content type of the request".to_string(),
//                         options: HeaderParamOptions { propagate: true },
//                     },
//                     HeaderId::new() => HeaderParamSpec {
//                         name: "Accept".to_string(),
//                         value: HclExpression::String("application/json, application/xml".to_string()),
//                         disabled: false,
//                         description: "The accept type of the request".to_string(),
//                         options: HeaderParamOptions { propagate: true },
//                 }
//             })),
//         };

//         // let item = ItemSpec::Request(Block::new(config));

//         let str = hcl::to_string(&model).unwrap();
//         println!("{}", str);

//         let json = serde_json::to_string(&model).unwrap();
//         println!("{}", json);

//         let new = hcl::from_str::<EntryModel>(&str).unwrap();

//         println!("{:?}", new);
//     }
// }
