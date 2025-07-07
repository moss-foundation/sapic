use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use ts_rs::TS;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::{
    dirs,
    models::types::{
        AfterUpdateDirEntryDescription, AfterUpdateItemEntryDescription, UpdateDirEntryParams,
        UpdateItemEntryParams,
        configuration::{DirConfigurationModel, ItemConfigurationModel},
    },
};

// ########################################################
// ###                   Create Entry                   ###
// ########################################################

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[validate(schema(function = "validate_create_item_entry_input"))]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateItemEntryInput {
    #[validate(custom(function = "validate_input_path"))]
    pub path: PathBuf,

    #[validate(length(min = 1))]
    pub name: String,

    pub order: isize,
    pub configuration: ItemConfigurationModel,
}

#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[validate(schema(function = "validate_create_dir_entry_input"))]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateDirEntryInput {
    #[validate(custom(function = "validate_input_path"))]
    pub path: PathBuf,

    #[validate(length(min = 1))]
    pub name: String,

    pub order: isize,
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

// ########################################################
// ###                   Delete Entry                   ###
// ########################################################

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

// Check that input path begins with a valid top folder
// such as requests, endpoints, etc.
fn validate_input_path(path: &Path) -> Result<(), ValidationError> {
    for folder in [
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
        dirs::ENVIRONMENTS_DIR,
        dirs::ASSETS_DIR,
    ] {
        if path.starts_with(folder) {
            return Ok(());
        }
    }
    Err(ValidationError::new(
        "The input path does not start with a valid top folder",
    ))
}

fn validate_create_item_entry_input(input: &CreateItemEntryInput) -> Result<(), ValidationError> {
    let folder = match input.configuration {
        ItemConfigurationModel::Request(_) => dirs::REQUESTS_DIR,
        ItemConfigurationModel::Endpoint(_) => dirs::ENDPOINTS_DIR,
        ItemConfigurationModel::Component(_) => dirs::COMPONENTS_DIR,
        ItemConfigurationModel::Schema(_) => dirs::SCHEMAS_DIR,
    };

    if !input.path.starts_with(folder) {
        Err(ValidationError::new(
            "The input path does not match with the config model",
        ))
    } else {
        Ok(())
    }
}

fn validate_create_dir_entry_input(input: &CreateDirEntryInput) -> Result<(), ValidationError> {
    let folder = match input.configuration {
        DirConfigurationModel::Request(_) => dirs::REQUESTS_DIR,
        DirConfigurationModel::Endpoint(_) => dirs::ENDPOINTS_DIR,
        DirConfigurationModel::Component(_) => dirs::COMPONENTS_DIR,
        DirConfigurationModel::Schema(_) => dirs::SCHEMAS_DIR,
    };

    if !input.path.starts_with(folder) {
        Err(ValidationError::new(
            "The input path does not match with the config model",
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        operations::CreateItemEntryInput,
        types::configuration::{ComponentItemConfigurationModel, ItemConfigurationModel},
    };
    use std::path::PathBuf;
    use validator::Validate;

    #[test]
    fn test_validate_input_path() {
        let path = PathBuf::from("something");
        let input = CreateItemEntryInput {
            path,
            name: "test".to_string(),
            order: 0,
            configuration: ItemConfigurationModel::Component(ComponentItemConfigurationModel {}),
        };

        let result = input.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_create_item_entry_input() {
        let path = PathBuf::from("requests");
        let input = CreateItemEntryInput {
            path,
            name: "component1".to_string(),
            order: 0,
            configuration: ItemConfigurationModel::Component(ComponentItemConfigurationModel {}),
        };
        let result = input.validate();
        assert!(result.is_err());
    }
}
