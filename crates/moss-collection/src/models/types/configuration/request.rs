use anyhow::Result;
use hcl::{Block, BlockLabel, Expression as HclExpression};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::models::{
    primitives::HttpMethod,
    types::configuration::{HeaderParamItem, HttpRequestParts},
};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HttpRequestItemConfiguration {
    pub request_parts: HttpRequestParts,
}

// impl HttpRequestItemConfiguration {
//     pub fn to_hcl(&self) -> Block {
//         Block::builder("request")
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

//         let mut headers = Vec::new();
//         for block in block.body.blocks() {
//             if block.identifier.as_str() == "header" {
//                 let header = HeaderParamItem::from_hcl(&block)?;
//                 headers.push(header);
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
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestItemConfigurationModel {
    Http(HttpRequestItemConfiguration),
}

// impl RequestItemConfigurationModel {
//     pub fn to_hcl(&self) -> Block {
//         match self {
//             RequestItemConfigurationModel::Http(model) => model.to_hcl(),
//         }
//     }

//     pub fn from_hcl(block: &Block) -> Result<Self> {
//         if let Some(label) = block.labels.get(0) {
//             let label_protocol = match label {
//                 BlockLabel::String(s) => s.as_str(),
//                 BlockLabel::Identifier(id) => id.as_str(),
//             };

//             match label_protocol {
//                 "http" => {
//                     let http_config = HttpRequestItemConfiguration::from_hcl(block)?;
//                     Ok(RequestItemConfigurationModel::Http(http_config))
//                 }
//                 _ => Err(anyhow::anyhow!("Unknown request type: {}", label_protocol)),
//             }
//         } else {
//             Err(anyhow::anyhow!("Missing label in request block"))
//         }
//     }
// }

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct HttpDirConfigurationModel {}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum RequestDirConfigurationModel {
    Http(HttpDirConfigurationModel),
}
