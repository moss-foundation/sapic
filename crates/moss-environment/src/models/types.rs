use moss_bindingutils::primitives::{ChangeJsonValue, ChangeString};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;

use sapic_base::environment::types::primitives::{VariableId, VariableName};

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "types.ts")]
#[serde(rename_all = "camelCase")]
pub struct VariableOptions {
    pub disabled: bool,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct AddVariableParams {
    pub name: VariableName,
    #[ts(type = "JsonValue")]
    pub global_value: JsonValue,
    #[ts(type = "JsonValue")]
    pub local_value: JsonValue,
    // pub kind: Option<VariableKind>,
    pub order: isize,
    pub desc: Option<String>,
    pub options: VariableOptions,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export, export_to = "types.ts")]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVariableParams {
    pub id: VariableId,
    pub name: Option<VariableName>,
    #[ts(optional, type = "ChangeJsonValue")]
    pub global_value: Option<ChangeJsonValue>,
    #[ts(optional, type = "ChangeJsonValue")]
    pub local_value: Option<ChangeJsonValue>,
    pub order: Option<isize>,
    #[ts(optional, type = "ChangeString")]
    pub desc: Option<ChangeString>,
    pub options: Option<VariableOptions>,
}

// INFO: moved to sapic-base
// /// @category Type
// #[derive(Clone, Debug, Deserialize, Serialize, TS, PartialEq, Eq)]
// #[ts(export, export_to = "types.ts")]
// pub enum VariableKind {
//     #[serde(rename = "secret")]
//     Secret,
//     #[serde(rename = "default")]
//     Default,
// }

// INFO: moved to sapic-base
// /// @category Type
// #[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(optional_fields)]
// #[ts(export, export_to = "types.ts")]
// pub struct VariableInfo {
//     pub id: VariableId,
//     pub name: VariableName,
//     #[ts(optional, type = "JsonValue")]
//     pub global_value: Option<JsonValue>,
//     #[ts(optional, type = "JsonValue")]
//     pub local_value: Option<JsonValue>,
//     pub disabled: bool,
//     // pub kind: VariableKind,
//     pub order: Option<isize>,
//     pub desc: Option<String>,
// }
