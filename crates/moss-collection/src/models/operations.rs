use crate::models::{
    primitives::EntryId,
    types::{
        AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription, UpdateDirEntryParams,
        UpdateItemEntryParams,
        configuration::{DirConfigurationModel, ItemConfigurationModel},
    },
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;

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
    #[ts(as = "String")]
    pub id: EntryId,
}

// ########################################################
// ###                   Delete Entry                   ###
// ########################################################

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryInput {
    #[ts(as = "String")]
    pub id: EntryId,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryOutput {
    #[ts(as = "String")]
    pub id: EntryId,
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

#[derive(Clone, Debug, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub enum StreamEntriesInput {
    #[serde(rename = "LOAD_ROOT")]
    LoadRoot,
    #[serde(rename = "RELOAD_PATH")]
    ReloadPath(PathBuf),
}

#[derive(Clone, Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamEntriesOutput {}
