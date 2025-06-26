use hcl::ser::{Block, LabeledBlock};
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use uuid::Uuid;

use crate::models::primitives::HttpMethod;

mod object;
pub use object::{Object, object, object_with_unquoted_keys};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct HeaderParameterOptionsObject {
//     pub propagate: bool,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct HeaderParameterBlock {
//     pub key: Identifier,
//     pub value: Expression,
//     pub description: Option<String>,
//     pub disabled: bool,
//     pub options: HeaderParameterOptionsObject,
// }

// #[derive(Debug)]
// pub struct MetadataBlock {
//     pub id: Uuid,
// }

// impl TryFrom<Block> for MetadataBlock {
//     type Error = anyhow::Error;

//     fn try_from(block: Block) -> Result<Self, Self::Error> {
//         let mut id = None;

//         for attr in block.body.attributes() {
//             if attr.key.as_str() == "id" {
//                 let value = attr.expr.to_string();
//                 id = Some(Uuid::parse_str(&value)?);
//             }
//         }

//         Ok(Self {
//             id: id.ok_or_else(|| anyhow::anyhow!("Missing id in metadata"))?,
//         })
//     }
// }

// impl Into<Block> for MetadataBlock {
//     fn into(self) -> hcl::Block {
//         Block::builder("metadata")
//             .add_attribute(("id", self.id.to_string()))
//             .build()
//     }
// }

// #[derive(Debug)]
// pub struct RequestBlock {
//     pub method: HttpMethod,
//     pub url: String,
//     // pub header_params: Vec<HeaderParameterBlock>,
// }

// impl From<hcl::Block> for RequestBlock {
//     fn from(block: hcl::Block) -> Self {
//         todo!()
//     }
// }

// impl Into<hcl::Block> for RequestBlock {
//     fn into(self) -> hcl::Block {
//         todo!()
//     }
// }

// #[derive(Debug)]
// pub struct ItemRequestConfigurationDocument {
//     pub metadata: MetadataBlock,
//     // pub request: RequestBlock,
// }

// impl From<hcl::Body> for ItemRequestConfigurationDocument {
//     fn from(body: hcl::Body) -> Self {
//         todo!()
//     }
// }

// impl Into<hcl::Body> for ItemRequestConfigurationDocument {
//     fn into(self) -> hcl::Body {
//         let block: Block = self.metadata.into();
//         Body::builder().add_block(block).build()
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    metadata: Block<Metadata>,
    request: LabeledBlock<IndexMap<String, RequestBlock>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestBlock {
    method: String,
    url: String,
    header: LabeledBlock<IndexMap<String, HeaderParameterBlock>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HeaderParameterBlock {
    value: String,
    disabled: bool,
    description: String,
    options: Object<HeaderParameterOptionsObject>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HeaderParameterOptionsObject {
    propagate: bool,
}

#[cfg(test)]
mod tests {

    use hcl::ser::LabeledBlock;

    use super::*;

    use indexmap::{IndexMap, indexmap};
    use serde::Serialize;

    // #[test]
    // fn test_metadata_block() {
    //     let doc = ItemRequestConfigurationDocument {
    //         metadata: MetadataBlock { id: Uuid::new_v4() },
    //     };

    //     let body: hcl::Body = doc.into();
    //     let str = hcl::to_string(&body).unwrap();
    //     println!("{}", str);
    // }

    #[test]
    fn test_labeled_block() {
        let config = Config {
            metadata: Block::new(Metadata {
                id: "c79646ec-b257-4143-a4ca-438add54e6f7".to_string(),
            }),
            request: LabeledBlock::new(indexmap! {
                "http".to_string() => RequestBlock {
                    method: "GET".to_string(),
                    url: "https://example.com".to_string(),
                    header: LabeledBlock::new(indexmap! {
                        "Content-Type".to_string() => HeaderParameterBlock {
                            value: "application/json".to_string(),
                            disabled: false,
                            description: "The content type of the request".to_string(),
                            options: Object::new(HeaderParameterOptionsObject { propagate: true }),
                        },
                        "Accept".to_string() => HeaderParameterBlock {
                            value: "application/json, application/xml".to_string(),
                            disabled: false,
                            description: "The accept type of the request".to_string(),
                            options: Object::new(HeaderParameterOptionsObject { propagate: true }),
                        }
                    })
                }
            }),
        };

        let str = hcl::to_string(&config).unwrap();
        println!("{}", str);

        let new = hcl::from_str::<Config>(&str).unwrap();

        println!("{:?}", new);
    }
}
