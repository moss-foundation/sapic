use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;

use crate::models::types::{
    AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription, UpdateDirEntryParams,
    UpdateItemEntryParams,
    configuration::{DirConfigurationModel, ItemConfigurationModel},
};

// ########################################################
// ###                   Create Entry                   ###
// ########################################################

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateItemEntryInput {
    // TODO: Add validation for path
    pub path: PathBuf,

    #[validate(length(min = 1))]
    pub name: String,

    pub order: usize,
    pub configuration: ItemConfigurationModel,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateDirEntryInput {
    // TODO: Add validation for path
    pub path: PathBuf,

    #[validate(length(min = 1))]
    pub name: String,

    pub order: usize,
    pub configuration: DirConfigurationModel,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum CreateEntryInput {
    Item(CreateItemEntryInput),
    Dir(CreateDirEntryInput),
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateEntryOutput {
    pub id: String,
}

// ########################################################
// ###                   Delete Entry                   ###
// ########################################################

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryInput {
    pub id: String,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryOutput {
    pub id: String,
}

// ########################################################
// ###                   Update Entry                   ###
// ########################################################

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum UpdateEntryInput {
    Item(UpdateItemEntryParams),
    Dir(UpdateDirEntryParams),
}

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

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum BatchUpdateEntryKind {
    Item(UpdateItemEntryParams),
    Dir(UpdateDirEntryParams),
}

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateEntryInput {
    pub entries: Vec<BatchUpdateEntryKind>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum BatchUpdateEntryOutputKind {
    Item(AfterUpdateItemEntryDescription),
    Dir(AfterUpdateDirEntryDescription),
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateEntryOutput {}

// ########################################################
// ###                  Stream Entries                  ###
// ########################################################

#[derive(Clone, Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamEntriesOutput {}
