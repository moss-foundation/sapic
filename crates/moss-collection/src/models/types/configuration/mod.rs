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
    primitives::{EntryClass, HttpMethod},
    types::{EntryProtocol, configuration::docschema::RawItemConfiguration},
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

impl From<RawItemConfiguration> for CompositeItemConfigurationModel {
    fn from(value: RawItemConfiguration) -> Self {
        match value {
            RawItemConfiguration::Request(block) => {
                let metadata = ConfigurationMetadata::from(block.metadata.clone());

                match block.url.clone().into_inner() {
                    docschema::UrlParts::Get(block) => {
                        let _details = block.into_inner();

                        CompositeItemConfigurationModel {
                            metadata,
                            inner: ItemConfigurationModel::Request(
                                ItemRequestConfigurationModel::Http(ItemHttpRequestConfiguration {
                                    request_parts: HttpRequestParts {
                                        method: HttpMethod::Get,
                                    },
                                }),
                            ),
                        }
                    }
                    docschema::UrlParts::Post(block) => {
                        let _details = block.into_inner();

                        CompositeItemConfigurationModel {
                            metadata,
                            inner: ItemConfigurationModel::Request(
                                ItemRequestConfigurationModel::Http(ItemHttpRequestConfiguration {
                                    request_parts: HttpRequestParts {
                                        method: HttpMethod::Post,
                                    },
                                }),
                            ),
                        }
                    }
                    docschema::UrlParts::Put(block) => {
                        let _details = block.into_inner();

                        CompositeItemConfigurationModel {
                            metadata,
                            inner: ItemConfigurationModel::Request(
                                ItemRequestConfigurationModel::Http(ItemHttpRequestConfiguration {
                                    request_parts: HttpRequestParts {
                                        method: HttpMethod::Put,
                                    },
                                }),
                            ),
                        }
                    }
                    docschema::UrlParts::Delete(block) => {
                        let _details = block.into_inner();

                        CompositeItemConfigurationModel {
                            metadata,
                            inner: ItemConfigurationModel::Request(
                                ItemRequestConfigurationModel::Http(ItemHttpRequestConfiguration {
                                    request_parts: HttpRequestParts {
                                        method: HttpMethod::Delete,
                                    },
                                }),
                            ),
                        }
                    }
                }
            }
            RawItemConfiguration::Endpoint(block) => {
                let metadata = ConfigurationMetadata::from(block.metadata.clone());

                match block.url.clone().into_inner() {
                    docschema::UrlParts::Get(block) => {
                        let _details = block.into_inner();

                        CompositeItemConfigurationModel {
                            metadata,
                            inner: ItemConfigurationModel::Endpoint(
                                EndpointItemConfigurationModel::Http(
                                    HttpEndpointItemConfiguration {
                                        request_parts: HttpRequestParts {
                                            method: HttpMethod::Get,
                                        },
                                    },
                                ),
                            ),
                        }
                    }
                    docschema::UrlParts::Post(block) => {
                        let _details = block.into_inner();

                        CompositeItemConfigurationModel {
                            metadata,
                            inner: ItemConfigurationModel::Endpoint(
                                EndpointItemConfigurationModel::Http(
                                    HttpEndpointItemConfiguration {
                                        request_parts: HttpRequestParts {
                                            method: HttpMethod::Post,
                                        },
                                    },
                                ),
                            ),
                        }
                    }
                    docschema::UrlParts::Put(block) => {
                        let _details = block.into_inner();

                        CompositeItemConfigurationModel {
                            metadata,
                            inner: ItemConfigurationModel::Endpoint(
                                EndpointItemConfigurationModel::Http(
                                    HttpEndpointItemConfiguration {
                                        request_parts: HttpRequestParts {
                                            method: HttpMethod::Put,
                                        },
                                    },
                                ),
                            ),
                        }
                    }
                    docschema::UrlParts::Delete(block) => {
                        let _details = block.into_inner();

                        CompositeItemConfigurationModel {
                            metadata,
                            inner: ItemConfigurationModel::Endpoint(
                                EndpointItemConfigurationModel::Http(
                                    HttpEndpointItemConfiguration {
                                        request_parts: HttpRequestParts {
                                            method: HttpMethod::Delete,
                                        },
                                    },
                                ),
                            ),
                        }
                    }
                }
            }
            RawItemConfiguration::Component(block) => {
                let metadata = ConfigurationMetadata::from(block.metadata.clone());

                CompositeItemConfigurationModel {
                    metadata,
                    inner: ItemConfigurationModel::Component(ComponentItemConfigurationModel {}),
                }
            }
            RawItemConfiguration::Schema(block) => {
                let metadata = ConfigurationMetadata::from(block.metadata.clone());

                CompositeItemConfigurationModel {
                    metadata,
                    inner: ItemConfigurationModel::Schema(SchemaItemConfigurationModel {}),
                }
            }
        }
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
