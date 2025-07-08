mod common;
mod component;
mod endpoint;
mod request;
mod schema;

pub mod docschema;

pub use common::*;
pub use component::*;
pub use endpoint::*;
use hcl::ser::Block;
pub use request::*;
pub use schema::*;

use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{
    primitives::{EntryClass, HttpMethod},
    types::{
        EntryProtocol,
        configuration::docschema::{
            RawDirComponentConfiguration, RawDirConfiguration, RawDirEndpointConfiguration,
            RawDirRequestConfiguration, RawDirSchemaConfiguration, RawItemComponentConfiguration,
            RawItemConfiguration, RawItemEndpointConfiguration, RawItemRequestConfiguration,
            RawItemSchemaConfiguration, UrlDetails, UrlParts,
        },
    },
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

#[derive(Debug, Clone, Serialize, Deserialize, Deref, DerefMut, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct CompositeDirConfigurationModel {
    pub metadata: ConfigurationMetadata,

    #[serde(flatten)]
    #[deref]
    #[deref_mut]
    pub inner: DirConfigurationModel,
}

impl From<RawDirConfiguration> for CompositeDirConfigurationModel {
    fn from(value: RawDirConfiguration) -> Self {
        match value {
            RawDirConfiguration::Request(block) => {
                let metadata = ConfigurationMetadata::from(block.metadata.clone());

                CompositeDirConfigurationModel {
                    metadata,
                    inner: DirConfigurationModel::Request(DirRequestConfigurationModel::Http(
                        DirHttpConfigurationModel {},
                    )),
                }
            }
            RawDirConfiguration::Endpoint(block) => {
                let metadata = ConfigurationMetadata::from(block.metadata.clone());

                CompositeDirConfigurationModel {
                    metadata,
                    inner: DirConfigurationModel::Endpoint(EndpointDirConfigurationModel::Http(
                        HttpEndpointDirConfiguration {},
                    )),
                }
            }
            RawDirConfiguration::Component(block) => {
                let metadata = ConfigurationMetadata::from(block.metadata.clone());

                CompositeDirConfigurationModel {
                    metadata,
                    inner: DirConfigurationModel::Component(ComponentDirConfigurationModel {}),
                }
            }
            RawDirConfiguration::Schema(block) => {
                let metadata = ConfigurationMetadata::from(block.metadata.clone());

                CompositeDirConfigurationModel {
                    metadata,
                    inner: DirConfigurationModel::Schema(SchemaDirConfigurationModel {}),
                }
            }
        }
    }
}

impl Into<RawDirConfiguration> for CompositeDirConfigurationModel {
    fn into(self) -> RawDirConfiguration {
        match self.inner {
            DirConfigurationModel::Request(model) => {
                let configuration = match model {
                    DirRequestConfigurationModel::Http(_http_model) => RawDirRequestConfiguration {
                        metadata: self.metadata.into(),
                        headers: None,
                    },
                };

                RawDirConfiguration::Request(Block::new(configuration))
            }
            DirConfigurationModel::Endpoint(model) => {
                let configuration = match model {
                    EndpointDirConfigurationModel::Http(_http_model) => {
                        RawDirEndpointConfiguration {
                            metadata: self.metadata.into(),
                            headers: None,
                        }
                    }
                };

                RawDirConfiguration::Endpoint(Block::new(configuration))
            }
            DirConfigurationModel::Component(_model) => {
                let configuration = RawDirComponentConfiguration {
                    metadata: self.metadata.into(),
                };

                RawDirConfiguration::Component(Block::new(configuration))
            }
            DirConfigurationModel::Schema(_model) => {
                let configuration = RawDirSchemaConfiguration {
                    metadata: self.metadata.into(),
                };

                RawDirConfiguration::Schema(Block::new(configuration))
            }
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

#[derive(Debug, Clone, Serialize, Deserialize, Deref, DerefMut, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct CompositeItemConfigurationModel {
    pub metadata: ConfigurationMetadata,

    #[serde(flatten)]
    #[deref]
    #[deref_mut]
    pub inner: ItemConfigurationModel,
}

impl Into<RawItemConfiguration> for CompositeItemConfigurationModel {
    fn into(self) -> RawItemConfiguration {
        match self.inner {
            ItemConfigurationModel::Request(model) => match model {
                ItemRequestConfigurationModel::Http(http_configuration) => {
                    let url_details = UrlDetails {
                        raw: "".to_string(), // TODO:
                    };
                    let request_parts = http_configuration.request_parts.clone();
                    let url_part = match request_parts.method {
                        HttpMethod::Get => UrlParts::Get(Block::new(url_details)),
                        HttpMethod::Post => UrlParts::Post(Block::new(url_details)),
                        HttpMethod::Put => UrlParts::Put(Block::new(url_details)),
                        HttpMethod::Delete => UrlParts::Delete(Block::new(url_details)),
                    };

                    RawItemConfiguration::Request(Block::new(RawItemRequestConfiguration {
                        metadata: self.metadata.into(),
                        url: Block::new(url_part),
                        headers: None,
                    }))
                }
            },
            ItemConfigurationModel::Endpoint(model) => match model {
                EndpointItemConfigurationModel::Http(http_configuration) => {
                    let url_details = UrlDetails {
                        raw: "".to_string(), // TODO:
                    };
                    let request_parts = http_configuration.request_parts.clone();
                    let url_part = match request_parts.method {
                        HttpMethod::Get => UrlParts::Get(Block::new(url_details)),
                        HttpMethod::Post => UrlParts::Post(Block::new(url_details)),
                        HttpMethod::Put => UrlParts::Put(Block::new(url_details)),
                        HttpMethod::Delete => UrlParts::Delete(Block::new(url_details)),
                    };

                    RawItemConfiguration::Endpoint(Block::new(RawItemEndpointConfiguration {
                        metadata: self.metadata.into(),
                        url: Block::new(url_part),
                        headers: None,
                    }))
                }
            },
            ItemConfigurationModel::Component(_model) => {
                RawItemConfiguration::Component(Block::new(RawItemComponentConfiguration {
                    metadata: self.metadata.into(),
                }))
            }
            ItemConfigurationModel::Schema(_model) => {
                RawItemConfiguration::Schema(Block::new(RawItemSchemaConfiguration {
                    metadata: self.metadata.into(),
                }))
            }
        }
    }
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
    pub fn id(&self) -> &str {
        match self {
            ConfigurationModel::Item(item) => &item.metadata.id,
            ConfigurationModel::Dir(dir) => &dir.metadata.id,
        }
    }
}
