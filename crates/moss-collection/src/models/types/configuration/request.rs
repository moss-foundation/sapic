use anyhow::Result;
use hcl::{BlockLabel, Expression as HclExpression, ser::Block};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::models::{
    primitives::HttpMethod,
    types::configuration::{HeaderParamItem, HttpRequestParts, docschema},
};

// #########################################################
// ###                      Item                         ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ItemHttpRequestConfiguration {
    pub request_parts: HttpRequestParts,
}

// impl From<Block<docschema::HttpRequestParts>> for ItemHttpRequestConfiguration {
//     fn from(value: Block<docschema::HttpRequestParts>) -> Self {
//         let inner = value.into_inner();
//         Self {
//             request_parts: HttpRequestParts {
//                 method: inner.method,
//             },
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ItemRequestConfigurationModel {
    Http(ItemHttpRequestConfiguration),
}

// #########################################################
// ###                      Dir                          ###
// #########################################################

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
// #[serde(rename_all = "snake_case")]
// #[ts(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct DirHttpConfigurationModel {}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum DirRequestConfigurationModel {
    Http(DirHttpConfigurationModel),
}
