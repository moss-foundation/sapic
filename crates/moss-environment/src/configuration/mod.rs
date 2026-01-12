use hcl::Expression as HclExpression;
use indexmap::IndexMap;
use moss_hcl::{Block, LabeledBlock, deserialize_expression, expression, serialize_expression};
use sapic_base::environment::types::primitives::{EnvironmentId, VariableId, VariableName};
use serde::{Deserialize, Serialize};

use crate::models::types::VariableOptions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataDecl {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDecl {
    pub name: VariableName,
    #[serde(
        serialize_with = "serialize_expression",
        deserialize_with = "deserialize_expression",
        skip_serializing_if = "expression::is_null"
    )]
    pub value: HclExpression,
    // pub kind: Option<VariableKind>,
    pub description: Option<String>,
    pub options: VariableOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFile {
    pub metadata: Block<MetadataDecl>,

    #[serde(rename = "variable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<LabeledBlock<IndexMap<VariableId, VariableDecl>>>,
}
