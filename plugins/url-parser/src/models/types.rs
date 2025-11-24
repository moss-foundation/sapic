use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum ParsedValue {
    String(String),
    Variable(String),
    // TODO: Implement this after updating the grammar for composite segment
    Composite(Vec<ParsedValue>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ParsedUrl {
    pub scheme_part: Option<ParsedValue>,
    pub host_part: Vec<ParsedValue>,
    pub path_part: Vec<ParsedValue>,
    pub query_part: Vec<(String, ParsedValue)>,
    pub fragment_part: Option<ParsedValue>,
}
