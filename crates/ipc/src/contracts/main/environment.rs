use moss_bindingutils::primitives::ChangeString;
use moss_environment::models::types::{AddVariableParams, UpdateVariableParams};
use sapic_base::{
    environment::types::{
        VariableInfo,
        primitives::{EnvironmentId, VariableId},
    },
    project::types::primitives::ProjectId,
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use ts_rs::TS;
use validator::Validate;

// Stream Environment
/// @category Event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "events.ts")]
pub struct StreamEnvironmentsEvent {
    pub id: EnvironmentId,

    /// The id of the project that the environment belongs to.
    /// If the environment is global, this will be `None`.
    pub project_id: Option<ProjectId>,
    pub is_active: bool,

    pub name: String,
    pub color: Option<String>,

    pub order: Option<isize>,
    pub total_variables: usize,
}

// Describe Environment
/// @category Operation
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEnvironmentInput {
    pub project_id: Option<ProjectId>,
    pub environment_id: EnvironmentId,
}

/// @category Operation
#[derive(Debug, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEnvironmentOutput {
    pub name: String,
    pub color: Option<String>,
    #[ts(type = "VariableInfo")]
    pub variables: Vec<VariableInfo>,
}

// Activate Environment
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct ActivateEnvironmentInput {
    pub project_id: Option<ProjectId>,
    pub environment_id: EnvironmentId,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ActivateEnvironmentOutput {
    pub environment_id: EnvironmentId,
}

// Create Environment
// FIXME: Should this be refactored to use an inner params?

/// @category Operation
#[derive(Clone, Debug, Deserialize, Serialize, Validate, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateEnvironmentInput {
    pub project_id: Option<ProjectId>,
    #[validate(length(min = 1))]
    pub name: String,
    pub order: isize,
    pub color: Option<String>,
    pub variables: Vec<AddVariableParams>,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateEnvironmentOutput {
    pub id: EnvironmentId,
    pub project_id: Option<ProjectId>,
    pub name: String,
    pub order: Option<isize>,
    pub color: Option<String>,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: PathBuf,
}

// Update Environment

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateEnvironmentGroupParams {
    pub project_id: ProjectId,
    pub expanded: Option<bool>,
    pub order: Option<isize>,
}

/// @category Type
#[derive(Clone, Debug, Deserialize, Validate, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct UpdateEnvironmentParams {
    pub project_id: Option<ProjectId>,
    pub id: EnvironmentId,
    pub name: Option<String>,
    pub order: Option<isize>,
    #[ts(optional, type = "ChangeString")]
    pub color: Option<ChangeString>,
    pub expanded: Option<bool>,
    pub vars_to_add: Vec<AddVariableParams>,
    pub vars_to_update: Vec<UpdateVariableParams>,
    pub vars_to_delete: Vec<VariableId>,
}

/// @category Operation
#[derive(Clone, Debug, Deserialize, Validate, TS)]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEnvironmentInput {
    #[serde(flatten)]
    pub inner: UpdateEnvironmentParams,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEnvironmentOutput {
    pub id: EnvironmentId,
}
// Batch Update Environment

/// @category Operation
#[derive(Debug, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateEnvironmentInput {
    #[validate(nested)]
    pub items: Vec<UpdateEnvironmentParams>,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateEnvironmentOutput {
    pub ids: Vec<EnvironmentId>,
}

// Delete Environment
/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEnvironmentInput {
    pub project_id: Option<ProjectId>,
    pub id: EnvironmentId,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEnvironmentOutput {
    pub id: EnvironmentId,
}

// Stream Environments

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamEnvironmentsOutput {
    pub groups: Vec<EnvironmentGroup>,

    #[serde(skip)]
    #[ts(skip)]
    pub total_returned: usize,
}

// Stream Project Environments

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamProjectEnvironmentsInput {
    pub project_id: ProjectId,
}

#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamProjectEnvironmentsOutput {
    #[serde(skip)]
    #[ts(skip)]
    pub total_returned: usize,
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentGroup {
    pub project_id: Arc<String>,
    pub expanded: bool,
    pub order: Option<isize>,
}

// Update Environment Group
/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEnvironmentGroupInput {
    #[serde(flatten)]
    pub inner: UpdateEnvironmentGroupParams,
}

// Batch Update Environment Group
/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateEnvironmentGroupInput {
    #[validate(nested)]
    pub items: Vec<UpdateEnvironmentGroupParams>,
}
