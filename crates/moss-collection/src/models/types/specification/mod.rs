// mod common;
// mod component;
// mod endpoint;
// mod request;
// mod schema;

// pub use common::*;
// pub use component::*;
// pub use endpoint::*;
// use hcl::ser::Block;
// pub use request::*;
// pub use schema::*;

// use derive_more::{Deref, DerefMut};
// use serde::{Deserialize, Serialize};
// use ts_rs::TS;

// use crate::{
//     models::{
//         primitives::{EntryClass, HttpMethod},
//         types::EntryProtocol,
//     },
//     spec::*,
// };

// // #########################################################
// // ###                      Dir                          ###
// // #########################################################

// /// @category Type
// #[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "types.ts")]
// pub enum DirConfigurationModel {
//     Request(RequestDirConfigurationModel),
//     Endpoint(EndpointDirConfigurationModel),
//     Component(ComponentDirConfigurationModel),
//     Schema(SchemaDirConfigurationModel),
// }

// // /// @category Type
// // #[derive(Debug, Clone, Serialize, Deserialize, Deref, DerefMut, TS)]
// // #[serde(rename_all = "camelCase")]
// // #[ts(export, export_to = "types.ts")]
// // pub struct CompositeDirConfigurationModel {
// //     pub metadata: ConfigurationMetadata,

// //     #[serde(flatten)]
// //     #[deref]
// //     #[deref_mut]
// //     pub inner: DirConfigurationModel,
// // }

// // impl From<DirSpec> for CompositeDirConfigurationModel {
// //     fn from(value: DirSpec) -> Self {
// //         match value {
// //             DirSpec::Request(block) => {
// //                 let metadata = ConfigurationMetadata::from(block.metadata.clone());

// //                 CompositeDirConfigurationModel {
// //                     metadata,
// //                     inner: DirConfigurationModel::Request(RequestDirConfigurationModel::Http(
// //                         DirHttpConfigurationModel {},
// //                     )),
// //                 }
// //             }
// //             DirSpec::Endpoint(block) => {
// //                 let metadata = ConfigurationMetadata::from(block.metadata.clone());

// //                 CompositeDirConfigurationModel {
// //                     metadata,
// //                     inner: DirConfigurationModel::Endpoint(EndpointDirConfigurationModel::Http(
// //                         HttpEndpointDirConfiguration {},
// //                     )),
// //                 }
// //             }
// //             DirSpec::Component(block) => {
// //                 let metadata = ConfigurationMetadata::from(block.metadata.clone());

// //                 CompositeDirConfigurationModel {
// //                     metadata,
// //                     inner: DirConfigurationModel::Component(ComponentDirConfigurationModel {}),
// //                 }
// //             }
// //             DirSpec::Schema(block) => {
// //                 let metadata = ConfigurationMetadata::from(block.metadata.clone());

// //                 CompositeDirConfigurationModel {
// //                     metadata,
// //                     inner: DirConfigurationModel::Schema(SchemaDirConfigurationModel {}),
// //                 }
// //             }
// //         }
// //     }
// // }

// // impl Into<DirSpec> for CompositeDirConfigurationModel {
// //     fn into(self) -> DirSpec {
// //         match self.inner {
// //             DirConfigurationModel::Request(model) => {
// //                 let configuration = match model {
// //                     RequestDirConfigurationModel::Http(_http_model) => DirRequestSpec {
// //                         metadata: self.metadata.into(),
// //                         headers: None,
// //                     },
// //                 };

// //                 DirSpec::Request(Block::new(configuration))
// //             }
// //             DirConfigurationModel::Endpoint(model) => {
// //                 let configuration = match model {
// //                     EndpointDirConfigurationModel::Http(_http_model) => DirEndpointSpec {
// //                         metadata: self.metadata.into(),
// //                         headers: None,
// //                     },
// //                 };

// //                 DirSpec::Endpoint(Block::new(configuration))
// //             }
// //             DirConfigurationModel::Component(_model) => {
// //                 let configuration = DirComponentSpec {
// //                     metadata: self.metadata.into(),
// //                 };

// //                 DirSpec::Component(Block::new(configuration))
// //             }
// //             DirConfigurationModel::Schema(_model) => {
// //                 let configuration = DirSchemaSpec {
// //                     metadata: self.metadata.into(),
// //                 };

// //                 DirSpec::Schema(Block::new(configuration))
// //             }
// //         }
// //     }
// // }

// // impl CompositeDirConfigurationModel {
// //     pub fn classification(&self) -> EntryClass {
// //         match self.inner {
// //             DirConfigurationModel::Request(_) => EntryClass::Request,
// //             DirConfigurationModel::Endpoint(_) => EntryClass::Endpoint,
// //             DirConfigurationModel::Component(_) => EntryClass::Component,
// //             DirConfigurationModel::Schema(_) => EntryClass::Schema,
// //         }
// //     }
// // }

// // #########################################################
// // ###                      Item                         ###
// // #########################################################

// // /// @category Type
// // #[derive(Debug, Clone, Serialize, Deserialize, TS)]
// // #[serde(rename_all = "camelCase")]
// // #[ts(export, export_to = "types.ts")]
// // pub enum ItemConfigurationModel {
// //     // FIXME: This should be `RequestItemConfigurationModel` for consistency
// //     Request(ItemRequestConfigurationModel),
// //     Endpoint(EndpointItemConfigurationModel),
// //     Component(ComponentItemConfigurationModel),
// //     Schema(SchemaItemConfigurationModel),
// // }

// // /// @category Type
// // #[derive(Debug, Clone, Serialize, Deserialize, Deref, DerefMut, TS)]
// // #[serde(rename_all = "camelCase")]
// // #[ts(export, export_to = "types.ts")]
// // pub struct CompositeItemConfigurationModel {
// //     pub metadata: ConfigurationMetadata,

// //     #[serde(flatten)]
// //     #[deref]
// //     #[deref_mut]
// //     pub inner: ItemConfigurationModel,
// // }

// // impl Into<ItemSpec> for CompositeItemConfigurationModel {
// //     fn into(self) -> ItemSpec {
// //         match self.inner {
// //             ItemConfigurationModel::Request(model) => match model {
// //                 ItemRequestConfigurationModel::Http(http_configuration) => {
// //                     let url_details = UrlDetails {
// //                         raw: "".to_string(), // TODO:
// //                     };
// //                     let request_parts = http_configuration.request_parts.clone();
// //                     let url_part = match request_parts.method {
// //                         HttpMethod::Get => UrlParts::Get(Block::new(url_details)),
// //                         HttpMethod::Post => UrlParts::Post(Block::new(url_details)),
// //                         HttpMethod::Put => UrlParts::Put(Block::new(url_details)),
// //                         HttpMethod::Delete => UrlParts::Delete(Block::new(url_details)),
// //                     };

// //                     ItemSpec::Request(Block::new(ItemRequestSpec {
// //                         metadata: self.metadata.into(),
// //                         url: Block::new(url_part),
// //                         headers: None,
// //                     }))
// //                 }
// //             },
// //             ItemConfigurationModel::Endpoint(model) => match model {
// //                 EndpointItemConfigurationModel::Http(http_configuration) => {
// //                     let url_details = UrlDetails {
// //                         raw: "".to_string(), // TODO:
// //                     };
// //                     let request_parts = http_configuration.request_parts.clone();
// //                     let url_part = match request_parts.method {
// //                         HttpMethod::Get => UrlParts::Get(Block::new(url_details)),
// //                         HttpMethod::Post => UrlParts::Post(Block::new(url_details)),
// //                         HttpMethod::Put => UrlParts::Put(Block::new(url_details)),
// //                         HttpMethod::Delete => UrlParts::Delete(Block::new(url_details)),
// //                     };

// //                     ItemSpec::Endpoint(Block::new(ItemEndpointSpec {
// //                         metadata: self.metadata.into(),
// //                         url: Block::new(url_part),
// //                         headers: None,
// //                     }))
// //                 }
// //             },
// //             ItemConfigurationModel::Component(_model) => {
// //                 ItemSpec::Component(Block::new(ItemComponentSpec {
// //                     metadata: self.metadata.into(),
// //                 }))
// //             }
// //             ItemConfigurationModel::Schema(_model) => {
// //                 ItemSpec::Schema(Block::new(ItemSchemaSpec {
// //                     metadata: self.metadata.into(),
// //                 }))
// //             }
// //         }
// //     }
// // }

// impl From<ItemSpec> for CompositeItemConfigurationModel {
//     fn from(value: ItemSpec) -> Self {
//         match value {
//             ItemSpec::Request(block) => {
//                 let metadata = ConfigurationMetadata::from(block.metadata.clone());

//                 match block.url.clone().into_inner() {
//                     UrlParts::Get(block) => {
//                         let _details = block.into_inner();

//                         CompositeItemConfigurationModel {
//                             metadata,
//                             inner: ItemConfigurationModel::Request(
//                                 ItemRequestConfigurationModel::Http(ItemHttpRequestConfiguration {
//                                     request_parts: HttpRequestParts {
//                                         method: HttpMethod::Get,
//                                     },
//                                 }),
//                             ),
//                         }
//                     }
//                     UrlParts::Post(block) => {
//                         let _details = block.into_inner();

//                         CompositeItemConfigurationModel {
//                             metadata,
//                             inner: ItemConfigurationModel::Request(
//                                 ItemRequestConfigurationModel::Http(ItemHttpRequestConfiguration {
//                                     request_parts: HttpRequestParts {
//                                         method: HttpMethod::Post,
//                                     },
//                                 }),
//                             ),
//                         }
//                     }
//                     UrlParts::Put(block) => {
//                         let _details = block.into_inner();

//                         CompositeItemConfigurationModel {
//                             metadata,
//                             inner: ItemConfigurationModel::Request(
//                                 ItemRequestConfigurationModel::Http(ItemHttpRequestConfiguration {
//                                     request_parts: HttpRequestParts {
//                                         method: HttpMethod::Put,
//                                     },
//                                 }),
//                             ),
//                         }
//                     }
//                     UrlParts::Delete(block) => {
//                         let _details = block.into_inner();

//                         CompositeItemConfigurationModel {
//                             metadata,
//                             inner: ItemConfigurationModel::Request(
//                                 ItemRequestConfigurationModel::Http(ItemHttpRequestConfiguration {
//                                     request_parts: HttpRequestParts {
//                                         method: HttpMethod::Delete,
//                                     },
//                                 }),
//                             ),
//                         }
//                     }
//                 }
//             }
//             ItemSpec::Endpoint(block) => {
//                 let metadata = ConfigurationMetadata::from(block.metadata.clone());

//                 match block.url.clone().into_inner() {
//                     UrlParts::Get(block) => {
//                         let _details = block.into_inner();

//                         CompositeItemConfigurationModel {
//                             metadata,
//                             inner: ItemConfigurationModel::Endpoint(
//                                 EndpointItemConfigurationModel::Http(
//                                     HttpEndpointItemConfiguration {
//                                         request_parts: HttpRequestParts {
//                                             method: HttpMethod::Get,
//                                         },
//                                     },
//                                 ),
//                             ),
//                         }
//                     }
//                     UrlParts::Post(block) => {
//                         let _details = block.into_inner();

//                         CompositeItemConfigurationModel {
//                             metadata,
//                             inner: ItemConfigurationModel::Endpoint(
//                                 EndpointItemConfigurationModel::Http(
//                                     HttpEndpointItemConfiguration {
//                                         request_parts: HttpRequestParts {
//                                             method: HttpMethod::Post,
//                                         },
//                                     },
//                                 ),
//                             ),
//                         }
//                     }
//                     UrlParts::Put(block) => {
//                         let _details = block.into_inner();

//                         CompositeItemConfigurationModel {
//                             metadata,
//                             inner: ItemConfigurationModel::Endpoint(
//                                 EndpointItemConfigurationModel::Http(
//                                     HttpEndpointItemConfiguration {
//                                         request_parts: HttpRequestParts {
//                                             method: HttpMethod::Put,
//                                         },
//                                     },
//                                 ),
//                             ),
//                         }
//                     }
//                     UrlParts::Delete(block) => {
//                         let _details = block.into_inner();

//                         CompositeItemConfigurationModel {
//                             metadata,
//                             inner: ItemConfigurationModel::Endpoint(
//                                 EndpointItemConfigurationModel::Http(
//                                     HttpEndpointItemConfiguration {
//                                         request_parts: HttpRequestParts {
//                                             method: HttpMethod::Delete,
//                                         },
//                                     },
//                                 ),
//                             ),
//                         }
//                     }
//                 }
//             }
//             ItemSpec::Component(block) => {
//                 let metadata = ConfigurationMetadata::from(block.metadata.clone());

//                 CompositeItemConfigurationModel {
//                     metadata,
//                     inner: ItemConfigurationModel::Component(ComponentItemConfigurationModel {}),
//                 }
//             }
//             ItemSpec::Schema(block) => {
//                 let metadata = ConfigurationMetadata::from(block.metadata.clone());

//                 CompositeItemConfigurationModel {
//                     metadata,
//                     inner: ItemConfigurationModel::Schema(SchemaItemConfigurationModel {}),
//                 }
//             }
//         }
//     }
// }

// impl CompositeItemConfigurationModel {
//     pub fn classification(&self) -> EntryClass {
//         match self.inner {
//             ItemConfigurationModel::Request(_) => EntryClass::Request,
//             ItemConfigurationModel::Endpoint(_) => EntryClass::Endpoint,
//             ItemConfigurationModel::Component(_) => EntryClass::Component,
//             ItemConfigurationModel::Schema(_) => EntryClass::Schema,
//         }
//     }

//     pub fn protocol(&self) -> Option<EntryProtocol> {
//         match &self.inner {
//             ItemConfigurationModel::Request(model) => match model {
//                 ItemRequestConfigurationModel::Http(model) => {
//                     Some(EntryProtocol::from(&model.request_parts.method))
//                 }
//             },
//             ItemConfigurationModel::Endpoint(_) => Some(EntryProtocol::Get),
//             _ => None,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub enum ConfigurationModel {
//     Item(CompositeItemConfigurationModel),
//     Dir(CompositeDirConfigurationModel),
// }

// impl ConfigurationModel {
//     pub fn id(&self) -> &str {
//         match self {
//             ConfigurationModel::Item(item) => &item.metadata.id,
//             ConfigurationModel::Dir(dir) => &dir.metadata.id,
//         }
//     }
// }
