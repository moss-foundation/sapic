use indexmap::IndexMap;
use moss_hcl::{Block, LabeledBlock};
use serde::{Deserialize, Serialize};

use crate::models::{primitives::EnvironmentId, types::VariableName};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub id: EnvironmentId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfiguration {
    pub metadata: Block<Metadata>,

    #[serde(rename = "variable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<LabeledBlock<IndexMap<VariableName, RawHeaderParameter>>>,
}
