mod component;
mod endpoint;
mod request;
mod schema;

pub use component::*;
pub use endpoint::*;
pub use request::*;
pub use schema::*;

use hcl::Expression;
use moss_hcl::{Block, Object};
use serde::{Deserialize, Serialize};

use crate::models::primitives::{EntryClass, EntryProtocol};

pub type HeaderName = String;
pub type Protocol = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlDetails {
    pub raw: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UrlParts {
    Get(Block<UrlDetails>),
    Post(Block<UrlDetails>),
    Put(Block<UrlDetails>),
    Delete(Block<UrlDetails>),
}

impl UrlParts {
    pub fn protocol(&self) -> Option<EntryProtocol> {
        match self {
            UrlParts::Get(_) => Some(EntryProtocol::Get),
            UrlParts::Post(_) => Some(EntryProtocol::Post),
            UrlParts::Put(_) => Some(EntryProtocol::Put),
            UrlParts::Delete(_) => Some(EntryProtocol::Delete),
        }
    }

    pub fn details(&self) -> &UrlDetails {
        match self {
            UrlParts::Get(details) => details,
            UrlParts::Post(details) => details,
            UrlParts::Put(details) => details,
            UrlParts::Delete(details) => details,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawHeaderParameter {
    pub value: Expression,
    pub disabled: bool,
    pub description: String,
    pub options: Object<HeaderParameterOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderParameterOptions {
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMetadata {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawItemConfiguration {
    Request(Block<RawItemRequestConfiguration>),
    Endpoint(Block<RawItemEndpointConfiguration>),
    Component(Block<RawItemComponentConfiguration>),
    Schema(Block<RawItemSchemaConfiguration>),
}

impl RawItemConfiguration {
    pub fn id(&self) -> &str {
        match self {
            RawItemConfiguration::Request(block) => block.metadata.id.as_str(),
            RawItemConfiguration::Endpoint(block) => block.metadata.id.as_str(),
            RawItemConfiguration::Component(block) => block.metadata.id.as_str(),
            RawItemConfiguration::Schema(block) => block.metadata.id.as_str(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawDirConfiguration {
    Request(Block<RawDirRequestConfiguration>),
    Endpoint(Block<RawDirEndpointConfiguration>),
    Component(Block<RawDirComponentConfiguration>),
    Schema(Block<RawDirSchemaConfiguration>),
}

impl RawDirConfiguration {
    pub fn id(&self) -> &str {
        match self {
            RawDirConfiguration::Request(block) => block.metadata.id.as_str(),
            RawDirConfiguration::Endpoint(block) => block.metadata.id.as_str(),
            RawDirConfiguration::Component(block) => block.metadata.id.as_str(),
            RawDirConfiguration::Schema(block) => block.metadata.id.as_str(),
        }
    }

    pub fn classification(&self) -> EntryClass {
        match self {
            RawDirConfiguration::Request(_) => EntryClass::Request,
            RawDirConfiguration::Endpoint(_) => EntryClass::Endpoint,
            RawDirConfiguration::Component(_) => EntryClass::Component,
            RawDirConfiguration::Schema(_) => EntryClass::Schema,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hcl::{Expression as HclExpression, ser::LabeledBlock};
    use indexmap::indexmap;
    use moss_common::nanoid::new_nanoid;

    #[test]
    fn test_dir() {
        let config = RawDirRequestConfiguration {
            metadata: Block::new(RawMetadata { id: new_nanoid() }),
            headers: None,
        };

        let item = RawDirConfiguration::Request(Block::new(config));

        let str = hcl::to_string(&item).unwrap();
        println!("{}", str);

        let new = hcl::from_str::<RawDirConfiguration>(&str).unwrap();

        println!("{:?}", new);
    }

    #[test]
    fn test_item() {
        let config = RawItemRequestConfiguration {
            metadata: Block::new(RawMetadata { id: new_nanoid() }),
            url: Block::new(UrlParts::Get(Block::new(UrlDetails {
                raw: "https://example.com".to_string(),
            }))),
            headers: Some(LabeledBlock::new(indexmap! {
                    "Content-Type".to_string() => RawHeaderParameter {
                        value: HclExpression::String("application/json".to_string()),
                        disabled: false,
                        description: "The content type of the request".to_string(),
                        options: Object::new(HeaderParameterOptions { propagate: true }),
                    },
                    "Accept".to_string() => RawHeaderParameter {
                        value: HclExpression::String("application/json, application/xml".to_string()),
                        disabled: false,
                        description: "The accept type of the request".to_string(),
                    options: Object::new(HeaderParameterOptions { propagate: true }),
                }
            })),
        };

        let item = RawItemConfiguration::Request(Block::new(config));

        let str = hcl::to_string(&item).unwrap();
        println!("{}", str);

        let new = hcl::from_str::<RawItemConfiguration>(&str).unwrap();

        println!("{:?}", new);
    }
}
