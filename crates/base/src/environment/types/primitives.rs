use moss_id_macro::ids;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

ids!([VariableId, EnvironmentId]);

pub type VariableName = String;
pub type EnvironmentName = String;

#[derive(Clone, Debug, Deserialize, Serialize, TS, PartialEq, Eq)]
#[ts(export, export_to = "environment/primitives.ts")]
pub enum VariableKind {
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "default")]
    Default,
}
