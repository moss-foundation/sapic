use serde::Serialize;
use std::path::PathBuf;
use ts_rs::TS;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::models::{
    primitives::EntryProtocol,
    types::configuration::{DirConfigurationModel, ItemConfigurationModel},
};

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateItemEntryInput {
    pub path: PathBuf,
    pub name: String,

    #[ts(optional)]
    pub order: Option<usize>,

    #[serde(flatten)]
    pub configuration: ItemConfigurationModel,
}

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateDirEntryInput {
    pub path: PathBuf,
    pub name: String,

    #[ts(optional)]
    pub order: Option<usize>,

    #[serde(flatten)]
    pub configuration: DirConfigurationModel,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum CreateEntryInput {
    Item(CreateItemEntryInput),
    Dir(CreateDirEntryInput),
}

impl CreateEntryInput {
    pub fn path(&self) -> &PathBuf {
        match self {
            CreateEntryInput::Item(item) => &item.path,
            CreateEntryInput::Dir(dir) => &dir.path,
        }
    }
}
impl Validate for CreateEntryInput {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            CreateEntryInput::Item(item) => item.validate(),
            CreateEntryInput::Dir(dir) => dir.validate(),
        }
    }
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateEntryOutput {
    pub id: Uuid,
}

#[derive(Clone, Debug, Serialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryInput {
    pub id: Uuid,
    pub path: PathBuf,
}

#[derive(Clone, Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteEntryOutput {}

// Update Entry

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateItemEntryInput {
    pub id: Uuid,

    #[ts(optional)]
    pub name: Option<String>,

    #[ts(optional)]
    pub protocol: Option<EntryProtocol>,

    #[ts(optional)]
    pub order: Option<usize>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateDirEntryInput {
    pub id: Uuid,

    #[ts(optional)]
    pub name: Option<String>,

    #[ts(optional)]
    pub order: Option<usize>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub enum UpdateEntryInput {
    Item(UpdateItemEntryInput),
    Dir(UpdateDirEntryInput),
}

#[derive(Clone, Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateEntryOutput {}

// Stream Entries

#[derive(Clone, Debug, Serialize, TS)]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamEntriesOutput {}
