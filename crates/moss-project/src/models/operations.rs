use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;

use crate::models::{
    primitives::{EntryClass, EntryId, EntryKind, EntryProtocol},
    types::{
        AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription, CreateDirEntryParams,
        CreateItemEntryParams, HeaderInfo, PathParamInfo, QueryParamInfo, UpdateDirEntryParams,
        UpdateItemEntryParams, VcsOperation,
    },
};
// ########################################################
// ###                   Create Entry                   ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum CreateEntryInput {
    Item(CreateItemEntryParams),
    Dir(CreateDirEntryParams),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateEntryOutput {
    pub id: EntryId,
}

// ########################################################
// ###                Batch Create Entry                ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum BatchCreateEntryKind {
    Item(CreateItemEntryParams),
    Dir(CreateDirEntryParams),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchCreateEntryInput {
    pub entries: Vec<BatchCreateEntryKind>,
}

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchCreateEntryOutput {
    #[ts(as = "Vec<String>")]
    pub ids: Vec<EntryId>,
}

// ########################################################
// ###                   Delete Entry                   ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryInput {
    #[ts(as = "String")]
    pub id: EntryId,
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryOutput {
    #[ts(as = "String")]
    pub id: EntryId,
}

// ########################################################
// ###                   Update Entry                   ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum UpdateEntryInput {
    Item(UpdateItemEntryParams),
    Dir(UpdateDirEntryParams),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum UpdateEntryOutput {
    Item(AfterUpdateItemEntryDescription),
    Dir(AfterUpdateDirEntryDescription),
}

// ########################################################
// ###                  Batch Update Entry              ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum BatchUpdateEntryKind {
    Item(UpdateItemEntryParams),
    Dir(UpdateDirEntryParams),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateEntryInput {
    pub entries: Vec<BatchUpdateEntryKind>,
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum BatchUpdateEntryOutputKind {
    Item(AfterUpdateItemEntryDescription),
    Dir(AfterUpdateDirEntryDescription),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateEntryOutput {}

// ########################################################
// ###                  Stream Entries                  ###
// ########################################################

/// @category Operation
#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub enum StreamEntriesInput {
    #[serde(rename = "LOAD_ROOT")]
    LoadRoot,
    #[serde(rename = "RELOAD_PATH")]
    ReloadPath(PathBuf),
}

/// @category Operation
#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct StreamEntriesOutput {
    // TODO: count total?
}

// ########################################################
// ###                  Describe Entry                  ###
// ########################################################
/// @category Operation
#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEntryInput {
    #[ts(as = "String")]
    pub id: EntryId,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct DescribeEntryOutput {
    // TODO: Separate dir/item entry?
    pub name: String,
    pub class: EntryClass,
    pub kind: EntryKind,
    pub protocol: Option<EntryProtocol>,
    pub url: Option<String>,
    pub headers: Vec<HeaderInfo>,
    pub path_params: Vec<PathParamInfo>,
    pub query_params: Vec<QueryParamInfo>,
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
