use moss_git::{models::types::BranchInfo, url::GIT_URL_REGEX};
use sapic_base::{
    other::GitProviderKind, project::types::primitives::ProjectId,
    user::types::primitives::AccountId,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use ts_rs::TS;
use validator::Validate;

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
pub struct GitHubCreateParams {
    pub account_id: AccountId,

    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// The name of the default branch
    pub branch: String,
}

#[derive(Debug, Serialize, Deserialize, TS, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GitLabCreateParams {
    pub account_id: AccountId,

    #[validate(regex(path = "*GIT_URL_REGEX"))]
    pub repository: String,
    /// The name of the default branch
    pub branch: String,
}

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
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
