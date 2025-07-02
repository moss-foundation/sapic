use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::models::{
    primitives::EntryProtocol,
    types::configuration::{DirConfigurationModel, ItemConfigurationModel},
};

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateItemEntryInput {
    pub path: PathBuf,
    pub name: String,
    pub order: Option<usize>,
    pub configuration: ItemConfigurationModel,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateDirEntryInput {
    pub path: PathBuf,
    pub name: String,
    pub order: Option<usize>,
    pub configuration: DirConfigurationModel,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS)]
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

    pub fn name(&self) -> &str {
        match self {
            CreateEntryInput::Item(item) => &item.name,
            CreateEntryInput::Dir(dir) => &dir.name,
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

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
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
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateItemEntryInput {
    pub id: Uuid,
    pub name: Option<String>,
    pub protocol: Option<EntryProtocol>,
    pub order: Option<usize>,
}

#[derive(Clone, Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateDirEntryInput {
    pub id: Uuid,
    pub name: Option<String>,
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
