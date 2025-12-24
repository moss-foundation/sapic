use moss_environment::models::types::AddVariableParams;
use sapic_base::{
    environment::types::{VariableInfo, primitives::EnvironmentId},
    project::types::primitives::ProjectId,
};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;
use validator::{Validate, ValidationError};

use crate::models::types::*;

// ------------------------------ //
// Project
// ------------------------------ //

/// @category Operation
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEnvironmentInput {
    pub id: ProjectId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEnvironmentOutput {
    #[ts(type = "VariableInfo")]
    pub variables: Vec<VariableInfo>,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamProjectsOutput {
    #[serde(skip)]
    #[ts(skip)]
    pub total_returned: usize,
}

// ------------------------------ //
// Environment
// ------------------------------ //
// Activate Environment
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct ActivateEnvironmentInput {
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
#[derive(Debug, Deserialize, Serialize, Validate, TS)]
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

/// @category Operation
#[derive(Debug, Deserialize, Validate, TS)]
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

// Delete Environment
/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEnvironmentInput {
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

// Describe Project
//
// /// @category Operation
// #[derive(Debug, Deserialize, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(export, export_to = "operations.ts")]
// pub struct DescribeProjectInput {
//     pub id: ProjectId,
// }

// /// @category Operation
// #[derive(Debug, Deserialize, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
// #[ts(optional_fields)]
// #[ts(export, export_to = "operations.ts")]
// pub struct DescribeProjectOutput {
//     pub name: String,
//     pub vcs: Option<VcsInfo>,
//     pub contributors: Vec<Contributor>,
//     pub created_at: String,
// }

// Get File Statuses
/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListChangesOutput {
    pub changes: Vec<EntryChange>,
}

// ------------------------------ //
// Environment Group
// ------------------------------ //

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEnvironmentGroupInput {
    #[serde(flatten)]
    pub inner: UpdateEnvironmentGroupParams,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateEnvironmentGroupInput {
    #[validate(nested)]
    pub items: Vec<UpdateEnvironmentGroupParams>,
}
