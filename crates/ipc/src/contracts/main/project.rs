use moss_bindingutils::primitives::{ChangePath, ChangeString};
use moss_git::{models::types::BranchInfo, url::GIT_URL_REGEX};
use sapic_base::{
    other::GitProviderKind, project::types::primitives::ProjectId,
    user::types::primitives::AccountId,
};
use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use ts_rs::TS;
use validator::{Validate, ValidationError};
//
// Stream Projects
//

/// @category Event
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "events.ts")]
pub struct StreamProjectsEvent {
    pub id: ProjectId,
    pub name: String,
    pub order: Option<isize>,
    pub expanded: bool,
    pub branch: Option<BranchInfo>,
    pub icon_path: Option<PathBuf>,
    pub archived: bool,
}

/// @category Operation
#[derive(Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct StreamProjectsOutput {
    #[serde(skip)]
    #[ts(skip)]
    pub total_returned: usize,
}

//
// Create Project
//

#[derive(Debug, Serialize, Deserialize, TS, Validate, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct GitHubCreateParams {
    pub account_id: AccountId,

    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// The name of the default branch
    pub branch: String,
}

#[derive(Debug, Serialize, Deserialize, TS, Validate, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct GitLabCreateParams {
    pub account_id: AccountId,

    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// The name of the default branch
    pub branch: String,
}

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub enum CreateProjectGitParams {
    GitHub(GitHubCreateParams),
    GitLab(GitLabCreateParams),
}

impl CreateProjectGitParams {
    pub fn account_id(&self) -> AccountId {
        match self {
            CreateProjectGitParams::GitHub(params) => params.account_id.clone(),
            CreateProjectGitParams::GitLab(params) => params.account_id.clone(),
        }
    }

    pub fn provider_kind(&self) -> GitProviderKind {
        match self {
            CreateProjectGitParams::GitHub(_) => GitProviderKind::GitHub,
            CreateProjectGitParams::GitLab(_) => GitProviderKind::GitLab,
        }
    }

    pub fn repository_url_string(&self) -> String {
        match self {
            CreateProjectGitParams::GitHub(params) => params.repository.clone(),
            CreateProjectGitParams::GitLab(params) => params.repository.clone(),
        }
    }

    pub fn branch_name(&self) -> String {
        match self {
            CreateProjectGitParams::GitHub(params) => params.branch.clone(),
            CreateProjectGitParams::GitLab(params) => params.branch.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, TS, Validate, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct CreateProjectParams {
    #[validate(length(min = 1))]
    pub name: String,

    pub order: isize,
    pub external_path: Option<PathBuf>,

    pub git_params: Option<CreateProjectGitParams>,
    pub icon_path: Option<PathBuf>,
}

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct CreateProjectInput {
    #[serde(flatten)]
    #[validate(nested)]
    pub inner: CreateProjectParams,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct CreateProjectOutput {
    pub id: ProjectId,
    pub name: String,
    pub order: Option<isize>,
    pub expanded: bool,
    pub icon_path: Option<PathBuf>,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: PathBuf,

    #[serde(skip)]
    #[ts(skip)]
    pub external_path: Option<PathBuf>,
}

//
// Delete Project
//

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteProjectInput {
    pub id: ProjectId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct DeleteProjectOutput {
    pub id: ProjectId,

    #[serde(skip)]
    #[ts(skip)]
    pub abs_path: Option<Arc<Path>>,
}

//
// Update Project
//

/// @category Type
#[derive(Clone, Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "types.ts")]
pub struct UpdateProjectParams {
    pub id: ProjectId,

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

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateProjectInput {
    #[serde(flatten)]
    #[validate(nested)]
    pub inner: UpdateProjectParams,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UpdateProjectOutput {
    pub id: ProjectId,
}

//
// Batch Update Project
//

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS, Validate)]
#[serde(rename_all = "camelCase")]
#[ts(optional_fields)]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateProjectInput {
    #[validate(nested)]
    pub items: Vec<UpdateProjectParams>,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct BatchUpdateProjectOutput {
    #[ts(as = "Vec<String>")]
    pub ids: Vec<ProjectId>,
}

//
// Archive Project
//

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ArchiveProjectInput {
    pub id: ProjectId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ArchiveProjectOutput {
    pub id: ProjectId,
}

//
// Unarchive Project
//

/// @category Operation
#[derive(Debug, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UnarchiveProjectInput {
    pub id: ProjectId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct UnarchiveProjectOutput {
    pub id: ProjectId,
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
