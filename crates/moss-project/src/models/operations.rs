use sapic_base::resource::types::primitives::ResourceId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;

use crate::models::{
    primitives::{ResourceClass, ResourceKind, ResourceProtocol},
    types::{
        AfterCreateResourceDescription, AfterUpdateDirResourceDescription,
        AfterUpdateItemResourceDescription, BodyInfo, CreateDirResourceParams,
        CreateItemResourceParams, HeaderInfo, PathParamInfo, QueryParamInfo,
        UpdateDirResourceParams, UpdateItemResourceParams, VcsOperation,
    },
};
// ########################################################
// ###                Create Resource                   ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum CreateResourceInput {
    Item(CreateItemResourceParams),
    Dir(CreateDirResourceParams),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateResourceOutput {
    pub id: ResourceId,
}

// ########################################################
// ###             Batch Create Resource                ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum BatchCreateResourceKind {
    Item(CreateItemResourceParams),
    Dir(CreateDirResourceParams),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchCreateResourceInput {
    pub resources: Vec<BatchCreateResourceKind>,
}

/// @category Operation
#[derive(Clone, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchCreateResourceOutput {
    pub resources: Vec<AfterCreateResourceDescription>,
}

// ########################################################
// ###                Delete Resource                   ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteResourceInput {
    pub id: ResourceId,
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteResourceOutput {
    pub id: ResourceId,
}

// ########################################################
// ###                Update Resource                   ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum UpdateResourceInput {
    Item(UpdateItemResourceParams),
    Dir(UpdateDirResourceParams),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum UpdateResourceOutput {
    Item(AfterUpdateItemResourceDescription),
    Dir(AfterUpdateDirResourceDescription),
}

// ########################################################
// ###                Batch Update Resource             ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum BatchUpdateResourceKind {
    Item(UpdateItemResourceParams),
    Dir(UpdateDirResourceParams),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateResourceInput {
    pub resources: Vec<BatchUpdateResourceKind>,
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum BatchUpdateResourceOutputKind {
    Item(AfterUpdateItemResourceDescription),
    Dir(AfterUpdateDirResourceDescription),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateResourceOutput {}

// ########################################################
// ###                Stream Resources                  ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub enum StreamResourcesInput {
    #[serde(rename = "LOAD_ROOT")]
    LoadRoot,
    #[serde(rename = "RELOAD_PATH")]
    ReloadPath(PathBuf),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct StreamResourcesOutput {
    // TODO: count total?
}

// ########################################################
// ###               Describe Resource                  ###
// ########################################################

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeResourceOutput {
    pub name: String,
    pub class: ResourceClass,
    pub kind: ResourceKind,
    pub protocol: Option<ResourceProtocol>,
    pub url: Option<String>,
    pub headers: Vec<HeaderInfo>,
    pub path_params: Vec<PathParamInfo>,
    pub query_params: Vec<QueryParamInfo>,
    pub body: Option<BodyInfo>,
}

/// @category Operation
#[derive(Clone, Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ExecuteVcsOperationInput {
    pub operation: VcsOperation,
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ExecuteVcsOperationOutput {}
