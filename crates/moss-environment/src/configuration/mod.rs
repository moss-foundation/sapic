use hcl::Expression;
use indexmap::IndexMap;
use moss_hcl::{Block, LabeledBlock};
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
    pub value: Expression,
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
