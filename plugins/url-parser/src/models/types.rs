use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ParsedValue {
    String(String),
    Variable(String),
}

// "hostPart": [
//     {
//     "composite": [
//         {
//         "string": "api-"
//         },
//         {
//         "variable": "env"
//         },
//         {
//         "string": "-sapic"
//         }
//     ]

// ["api-", {var: env}, ".sapic.dev"], ["path1/", {path_var: path2}, "/path3"]

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ParsedUrl {
    pub scheme_part: Option<ParsedValue>,
    // TODO: No need to split the host by dot
    pub host_part: Vec<ParsedValue>,
    pub path_part: Vec<ParsedValue>,
    pub query_part: Vec<(String, Option<ParsedValue>)>,
    pub fragment_part: Option<ParsedValue>,
}
