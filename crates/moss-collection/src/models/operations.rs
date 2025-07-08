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
    #[validate(custom(function = "validate_create_entry_input_path"))]
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
    #[validate(custom(function = "validate_create_entry_input_path"))]
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
fn validate_create_entry_input_path(path: &Path) -> Result<(), ValidationError> {
    for folder in [
        dirs::REQUESTS_DIR,
        dirs::ENDPOINTS_DIR,
        dirs::COMPONENTS_DIR,
        dirs::SCHEMAS_DIR,
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
    use std::path::PathBuf;
    use validator::Validate;

    use crate::{
        dirs,
        models::{
            operations::{
                CreateDirEntryInput, CreateItemEntryInput, validate_create_entry_input_path,
            },
            primitives::HttpMethod,
            types::configuration::{
                ComponentDirConfigurationModel, ComponentItemConfigurationModel,
                DirConfigurationModel, DirHttpConfigurationModel, EndpointDirConfigurationModel,
                EndpointItemConfigurationModel, HttpEndpointDirConfiguration,
                HttpEndpointItemConfiguration, HttpRequestParts, ItemConfigurationModel,
                ItemHttpRequestConfiguration, ItemRequestConfigurationModel,
                RequestDirConfigurationModel, SchemaDirConfigurationModel,
                SchemaItemConfigurationModel,
            },
        },
    };

    #[test]
    fn test_validate_create_entry_input_path_valid() {
        for path in [
            dirs::REQUESTS_DIR,
            dirs::ENDPOINTS_DIR,
            dirs::COMPONENTS_DIR,
            dirs::SCHEMAS_DIR,
        ] {
            let entry_path = PathBuf::from(&path).join("entry");
            let result = validate_create_entry_input_path(&entry_path);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_validate_create_entry_input_path_invalid() {
        let entry_path = PathBuf::from("Incorrect").join("entry");
        let result = validate_create_entry_input_path(&entry_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_create_item_entry_input_matching() {
        let path = PathBuf::from(dirs::REQUESTS_DIR);
        let input = CreateItemEntryInput {
            path,
            name: "entry".to_string(),
            order: 0,
            configuration: ItemConfigurationModel::Request(ItemRequestConfigurationModel::Http(
                ItemHttpRequestConfiguration {
                    request_parts: HttpRequestParts {
                        method: HttpMethod::Get,
                    },
                },
            )),
        };
        let result = input.validate();
        assert!(result.is_ok());

        let path = PathBuf::from(dirs::ENDPOINTS_DIR);
        let input = CreateItemEntryInput {
            path,
            name: "entry".to_string(),
            order: 0,
            configuration: ItemConfigurationModel::Endpoint(EndpointItemConfigurationModel::Http(
                HttpEndpointItemConfiguration {
                    request_parts: HttpRequestParts {
                        method: HttpMethod::Get,
                    },
                },
            )),
        };
        let result = input.validate();
        assert!(result.is_ok());

        let path = PathBuf::from(dirs::COMPONENTS_DIR);
        let input = CreateItemEntryInput {
            path,
            name: "entry".to_string(),
            order: 0,
            configuration: ItemConfigurationModel::Component(ComponentItemConfigurationModel {}),
        };
        let result = input.validate();
        assert!(result.is_ok());

        let path = PathBuf::from(dirs::SCHEMAS_DIR);
        let input = CreateItemEntryInput {
            path,
            name: "entry".to_string(),
            order: 0,
            configuration: ItemConfigurationModel::Schema(SchemaItemConfigurationModel {}),
        };
        let result = input.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_create_item_entry_input_not_matching() {
        let path = PathBuf::from(dirs::COMPONENTS_DIR);
        let input = CreateItemEntryInput {
            path,
            name: "entry".to_string(),
            order: 0,
            configuration: ItemConfigurationModel::Schema(SchemaItemConfigurationModel {}),
        };
        let result = input.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_create_dir_entry_input_matching() {
        let path = PathBuf::from(dirs::REQUESTS_DIR);
        let input = CreateDirEntryInput {
            path,
            name: "dir".to_string(),
            order: 0,
            configuration: DirConfigurationModel::Request(RequestDirConfigurationModel::Http(
                DirHttpConfigurationModel {},
            )),
        };
        let result = input.validate();
        assert!(result.is_ok());

        let path = PathBuf::from(dirs::ENDPOINTS_DIR);
        let input = CreateDirEntryInput {
            path,
            name: "dir".to_string(),
            order: 0,
            configuration: DirConfigurationModel::Endpoint(EndpointDirConfigurationModel::Http(
                HttpEndpointDirConfiguration {},
            )),
        };
        let result = input.validate();
        assert!(result.is_ok());

        let path = PathBuf::from(dirs::COMPONENTS_DIR);
        let input = CreateDirEntryInput {
            path,
            name: "dir".to_string(),
            order: 0,
            configuration: DirConfigurationModel::Component(ComponentDirConfigurationModel {}),
        };
        let result = input.validate();
        assert!(result.is_ok());

        let path = PathBuf::from(dirs::SCHEMAS_DIR);
        let input = CreateDirEntryInput {
            path,
            name: "dir".to_string(),
            order: 0,
            configuration: DirConfigurationModel::Schema(SchemaDirConfigurationModel {}),
        };
        let result = input.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_create_dir_entry_input_not_matching() {
        let path = PathBuf::from(dirs::REQUESTS_DIR);
        let input = CreateDirEntryInput {
            path,
            name: "dir".to_string(),
            order: 0,
            configuration: DirConfigurationModel::Schema(SchemaDirConfigurationModel {}),
        };
        let result = input.validate();
        assert!(result.is_err());
    }
}
