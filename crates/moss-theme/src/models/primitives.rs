use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// @category Primitive
#[derive(Deserialize, Serialize, TS, Clone, Debug)]
#[ts(export, export_to = "primitives.ts")]
pub enum ThemeMode {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
}

/// @category Primitive
#[derive(Deserialize, Serialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
#[ts(export, export_to = "types.ts")]
pub enum CssValue {
    #[serde(rename = "String")]
    StringValue { value: String },
    #[serde(rename = "Variable")]
    VariableValue { value: String },
}
