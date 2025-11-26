use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ParsedValue {
    String(String),
    Variable(String),
    PathVariable(String),
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

pub type ValueList = Vec<ParsedValue>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct QueryParam {
    pub key: ValueList,
    pub value: Option<ValueList>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ParsedUrl {
    pub scheme_part: ValueList,
    // TODO: No need to split the host by dot
    pub host_part: ValueList,
    pub path_part: ValueList,
    pub query_part: Vec<QueryParam>,
    pub fragment_part: Option<ValueList>,
    pub raw: ValueList,
}
