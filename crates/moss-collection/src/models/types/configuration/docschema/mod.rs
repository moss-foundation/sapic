mod component;
mod endpoint;
mod request;
mod schema;

pub use component::*;
pub use endpoint::*;
pub use request::*;
pub use schema::*;

use crate::{
    dirs,
    models::primitives::{EntryClass, EntryId, EntryProtocol},
};
use hcl::Expression;
use moss_hcl::{Block, Object};
use serde::{Deserialize, Serialize};

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
    pub id: EntryId,
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
    pub fn id(&self) -> &EntryId {
        match self {
            RawItemConfiguration::Request(block) => &block.metadata.id,
            RawItemConfiguration::Endpoint(block) => &block.metadata.id,
            RawItemConfiguration::Component(block) => &block.metadata.id,
            RawItemConfiguration::Schema(block) => &block.metadata.id,
        }
    }

    pub fn classification_folder(&self) -> &str {
        match self {
            RawItemConfiguration::Request(_) => dirs::REQUESTS_DIR,
            RawItemConfiguration::Endpoint(_) => dirs::ENDPOINTS_DIR,
            RawItemConfiguration::Component(_) => dirs::COMPONENTS_DIR,
            RawItemConfiguration::Schema(_) => dirs::SCHEMAS_DIR,
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
    pub fn id(&self) -> &EntryId {
        match self {
            RawDirConfiguration::Request(block) => &block.metadata.id,
            RawDirConfiguration::Endpoint(block) => &block.metadata.id,
            RawDirConfiguration::Component(block) => &block.metadata.id,
            RawDirConfiguration::Schema(block) => &block.metadata.id,
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

    pub fn classification_folder(&self) -> &str {
        match self {
            RawDirConfiguration::Request(_) => dirs::REQUESTS_DIR,
            RawDirConfiguration::Endpoint(_) => dirs::ENDPOINTS_DIR,
            RawDirConfiguration::Component(_) => dirs::COMPONENTS_DIR,
            RawDirConfiguration::Schema(_) => dirs::SCHEMAS_DIR,
        }
    }
}

#[cfg(test)]
mod tests {
    use hcl::{Expression as HclExpression, ser::LabeledBlock};
    use indexmap::indexmap;

    use super::*;

    #[test]
    fn test_dir() {
        let config = RawDirRequestConfiguration {
            metadata: Block::new(RawMetadata { id: EntryId::new() }),
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
            metadata: Block::new(RawMetadata { id: EntryId::new() }),
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
