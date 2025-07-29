use indexmap::IndexMap;
use moss_hcl::{Block, HclExpression, LabeledBlock, deserialize_expression, serialize_expression};
use serde::{Deserialize, Serialize};

use crate::models::{
    primitives::{EnvironmentId, VariableId},
    types::{VariableKind, VariableName, VariableOptions},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub id: EnvironmentId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDefinition {
    pub name: VariableName,
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression"
    )]
    pub value: HclExpression,
    pub kind: Option<VariableKind>,
    pub description: Option<String>,
    pub options: VariableOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentFile {
    pub metadata: Block<Metadata>,

    #[serde(rename = "variable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<LabeledBlock<IndexMap<VariableId, VariableDefinition>>>,
}
