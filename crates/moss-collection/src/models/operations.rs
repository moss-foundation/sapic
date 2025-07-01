use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use uuid::Uuid;
use validator::Validate;

use crate::models::{
    primitives::{EntryPath, EntryProtocol},
    types::configuration::{
        CompositeDirConfigurationModel, CompositeItemConfigurationModel, DirConfigurationModel,
        ItemConfigurationModel,
    },
};

// Create Entry

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
    pub id: Uuid,
}

// Delete Entry

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryInput {
    pub id: Uuid,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryOutput {
    pub id: Uuid,
}

// Update Entry

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateItemEntryInput {
    pub id: Uuid,
    // TODO: Add validation for path
    pub path: PathBuf,

    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub protocol: Option<EntryProtocol>,
    pub order: Option<usize>,
    pub expanded: Option<bool>,
}

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateDirEntryInput {
    pub id: Uuid,
    pub path: PathBuf,

    #[validate(length(min = 1))]
    pub name: Option<String>,
    pub order: Option<usize>,
    pub expanded: Option<bool>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum UpdateEntryInput {
    Item(UpdateItemEntryInput),
    Dir(UpdateDirEntryInput),
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateItemEntryOutput {
    pub id: Uuid,
    pub path: EntryPath,
    pub configuration: CompositeItemConfigurationModel,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateDirEntryOutput {
    pub id: Uuid,
    pub path: EntryPath,
    pub configuration: CompositeDirConfigurationModel,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "operations.ts")]
pub enum UpdateEntryOutput {
    Item(UpdateItemEntryOutput),
    Dir(UpdateDirEntryOutput),
}

// Stream Entries

#[derive(Clone, Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamEntriesOutput {}
