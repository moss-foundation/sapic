use anyhow::Result;
use hcl::{Block, BlockLabel, Expression as HclExpression};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::{primitives::HttpMethod, types::configuration::HttpRequestParts};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HttpEndpointItemConfiguration {
    pub request_parts: HttpRequestParts,
}

// impl HttpEndpointItemConfiguration {
//     pub fn to_hcl(&self) -> Block {
//         Block::builder("endpoint")
//             .add_label("http")
//             .add_attribute(("method", self.request_parts.method.to_string()))
//             .build()
//     }

//     pub fn from_hcl(block: &Block) -> Result<Self> {
//         let mut method = None;

//         for attr in block.body.attributes() {
//             if attr.key.as_str() == "method" {
//                 if let HclExpression::String(method_str) = &attr.expr {
//                     method = Some(match method_str.as_str() {
//                         "GET" => HttpMethod::Get,
//                         "POST" => HttpMethod::Post,
//                         "PUT" => HttpMethod::Put,
//                         "DELETE" => HttpMethod::Delete,
//                         _ => return Err(anyhow::anyhow!("Unknown HTTP method: {}", method_str)),
//                     });
//                     break;
//                 }
//             }
//         }

//         Ok(Self {
//             request_parts: HttpRequestParts {
//                 method: method.ok_or_else(|| anyhow::anyhow!("Missing method"))?,
//             },
//         })
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum EndpointItemConfigurationModel {
    Http(HttpEndpointItemConfiguration),
}

// impl EndpointItemConfigurationModel {
//     pub fn to_hcl(&self) -> Block {
//         match self {
//             EndpointItemConfigurationModel::Http(model) => model.to_hcl(),
//         }
//     }

//     pub fn from_hcl(block: &Block) -> Result<Self> {
//         if let Some(label) = block.labels.get(0) {
//             let label_str = match label {
//                 BlockLabel::String(s) => s.as_str(),
//                 BlockLabel::Identifier(id) => id.as_str(),
//             };

//             match label_str {
//                 "http" => {
//                     let http_config = HttpEndpointItemConfiguration::from_hcl(block)?;
//                     Ok(EndpointItemConfigurationModel::Http(http_config))
//                 }
//                 _ => Err(anyhow::anyhow!("Unknown endpoint type: {}", label_str)),
//             }
//         } else {
//             Err(anyhow::anyhow!("Missing label in endpoint block"))
//         }
//     }
// }

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum EndpointDirConfigurationModel {}
