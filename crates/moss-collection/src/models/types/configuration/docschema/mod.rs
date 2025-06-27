mod component;
mod endpoint;
mod object;
mod request;
mod schema;

pub use component::*;
pub use endpoint::*;
pub use object::*;
pub use request::*;
pub use schema::*;

use hcl::{Expression, ser::Block};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct RawHeaderParameter {
    pub value: Expression,
    pub disabled: bool,
    pub description: String,
    pub options: Object<HeaderParameterOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeaderParameterOptions {
    pub propagate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawMetadata {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RawItemConfiguration {
    Request(Block<RawItemRequestConfiguration>),
    Endpoint(Block<RawItemEndpointConfiguration>),
    Component(Block<RawItemComponentConfiguration>),
    Schema(Block<RawItemRequestConfiguration>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use hcl::{Expression as HclExpression, ser::LabeledBlock};
    use indexmap::indexmap;

    #[test]
    fn test_item() {
        let config = RawItemRequestConfiguration {
            metadata: Block::new(RawMetadata { id: Uuid::new_v4() }),
            url: Block::new(UrlParts::Get(Block::new(UrlDetails {
                raw: "https://example.com".to_string(),
            }))),
            headers: LabeledBlock::new(indexmap! {
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
            }),
        };

        let item = RawItemConfiguration::Request(Block::new(config));

        let str = hcl::to_string(&item).unwrap();
        println!("{}", str);

        let new = hcl::from_str::<RawItemConfiguration>(&str).unwrap();

        println!("{:?}", new);
    }
}
