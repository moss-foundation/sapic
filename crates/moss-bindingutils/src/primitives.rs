use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::path::PathBuf;
use ts_rs::TS;

// This is a workaround to allow the JsonValue type to be exported to TypeScript.
// Such export should be used as the single place for generating exports for this type.
#[allow(non_camel_case_types)]
#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = "primitives.ts",
    rename = "JsonValue",
    type = r#"number | string | boolean | Array<JsonValue> | { [key in string]?: JsonValue } | null"#
)]
struct JsonValue__TypeExport;

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ChangeUsize {
    Update(usize),
    Remove,
}

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ChangeString {
    Update(String),
    Remove,
}

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ChangeBool {
    Update(bool),
    Remove,
}

/// @category Primitive
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ChangePath {
    Update(PathBuf),
    Remove,
}

/// @category Primitive
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ChangeJsonValue {
    Update(#[ts(type = "JsonValue")] JsonValue),
    Remove,
}
