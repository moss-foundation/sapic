use hcl::ser::{Block, LabeledBlock};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::models::{
    primitives::EntryProtocol,
    types::configuration::docschema::{DirRequestParts, HttpRequestParts, Metadata},
};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemRequestConfiguration {
    pub metadata: Block<Metadata>,
    #[serde(flatten)]
    pub http_request_parts: Option<HttpRequestParts>,
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Serialize, Deserialize)]
pub struct DirRequestConfiguration {
    pub metadata: Block<Metadata>,
    // #[serde(rename = "request")]
    // pub request_parts: LabeledBlock<IndexMap<Protocol, DirRequestParts>>,
}

#[cfg(test)]
mod tests {
    use crate::models::{
        primitives::HttpMethod,
        types::configuration::docschema::{
            HeaderParameter, HeaderParameterOptions, HttpRequestParts, Object,
        },
    };

    use super::*;

    use hcl::{Expression as HclExpression, ser::LabeledBlock};
    use indexmap::indexmap;
    use uuid::Uuid;

    #[test]
    fn test_labeled_block() {
        let config = ItemRequestConfiguration {
            metadata: Block::new(Metadata { id: Uuid::new_v4() }),
            http_request_parts: Some(HttpRequestParts {
                method: HttpMethod::Get,
                url: "https://example.com".to_string(),
                headers: LabeledBlock::new(indexmap! {
                    "Content-Type".to_string() => HeaderParameter {
                        value: HclExpression::String("application/json".to_string()),
                        disabled: false,
                        description: "The content type of the request".to_string(),
                        options: Object::new(HeaderParameterOptions { propagate: true }),
                    },
                    "Accept".to_string() => HeaderParameter {
                        value: HclExpression::String("application/json, application/xml".to_string()),
                        disabled: false,
                        description: "The accept type of the request".to_string(),
                        options: Object::new(HeaderParameterOptions { propagate: true }),
                    }
                }),
            }),
        };

        let str = hcl::to_string(&config).unwrap();
        println!("{}", str);

        let new = hcl::from_str::<ItemRequestConfiguration>(&str).unwrap();

        println!("{:?}", new);
    }
}
