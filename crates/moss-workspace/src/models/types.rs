mod editor;
pub use editor::*;

use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_environment::models::{
    primitives::{EnvironmentId, VariableId},
    types::{AddVariableParams, UpdateVariableParams, VariableInfo},
};
use moss_git::url::GIT_URL_REGEX;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;
use validator::{Validate, ValidationError};

use crate::models::primitives::{ChangeCollectionId, CollectionId};

use super::primitives::{ActivitybarPosition, SidebarPosition};

pub type EnvironmentName = String;

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateCollectionParams {
    pub id: CollectionId,

    #[validate(length(min = 1))]
    pub name: Option<String>,

    #[validate(custom(function = "validate_change_repository"))]
    #[ts(optional, type = "ChangeString")]
    pub repository: Option<ChangeString>,

    // TODO: add validation
    #[ts(optional, type = "ChangePath")]
    pub icon_path: Option<ChangePath>,
    pub order: Option<isize>,
    pub expanded: Option<bool>,
}

/// @category Type
#[derive(Debug, Deserialize, Validate, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct UpdateEnvironmentParams {
    pub id: EnvironmentId,

    /// When updating an environment, we can move it to another collection
    /// or remove its link to a specific collection to make it global.
    pub collection_id: Option<ChangeCollectionId>,
    pub name: Option<String>,
    pub order: Option<isize>,
    #[ts(optional, type = "ChangeString")]
    pub color: Option<ChangeString>,
    pub expanded: Option<bool>,
    pub vars_to_add: Vec<AddVariableParams>,
    pub vars_to_update: Vec<UpdateVariableParams>,
    pub vars_to_delete: Vec<VariableId>,
}

fn validate_change_repository(repo: &ChangeString) -> Result<(), ValidationError> {
    match repo {
        ChangeString::Update(repo) => GIT_URL_REGEX
            .is_match(repo)
            .then_some(())
            .ok_or(ValidationError::new("Invalid Git URL format")),
        ChangeString::Remove => Ok(()),
    }
}

/// @category Type
#[derive(Debug, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct CollectionInfo {
    pub id: String,
    pub display_name: String,
    pub order: Option<isize>,
}

/// @category Type
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EnvironmentInfo {
    pub id: String,
    pub collection_id: Option<String>,
    pub name: String,
    pub display_name: String,
    pub order: isize,
    pub color: Option<String>,
    pub variables: Vec<VariableInfo>,
}

// ------------------------------------------------------------
// Activitybar Part State
// ------------------------------------------------------------

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct ActivitybarItemStateInfo {
    pub id: String,
    pub order: isize,
    pub visible: bool,
}

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct ActivitybarPartStateInfo {
    pub last_active_container_id: Option<String>,
    pub position: ActivitybarPosition,
    pub items: Vec<ActivitybarItemStateInfo>,
}

// ------------------------------------------------------------
// Sidebar Part State
// ------------------------------------------------------------

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct SidebarPartStateInfo {
    pub position: SidebarPosition,
    pub size: usize,
    pub visible: bool,
}

// ------------------------------------------------------------
// Panel Part State
// ------------------------------------------------------------

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct PanelPartStateInfo {
    pub size: usize,
    pub visible: bool,
}

// ------------------------------------------------------------
// Editor Part State
// ------------------------------------------------------------

/// @category Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct EditorPartStateInfo {
    pub grid: EditorGridState,
    #[ts(type = "Record<string, EditorPanelState>")]
    pub panels: HashMap<String, EditorPanelState>,
    pub active_group: Option<String>,
}
