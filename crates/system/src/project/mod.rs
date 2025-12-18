pub mod project_service;

use async_trait::async_trait;
use moss_git::url::GitUrl;
use sapic_base::{
    other::GitProviderKind,
    project::{config::ProjectConfig, manifest::ProjectManifest},
};
use sapic_core::context::AnyAsyncContext;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct CreateProjectGitParams {
    pub provider_kind: GitProviderKind,
    pub repository_url: GitUrl,
    pub branch_name: String,
}

pub struct CreateProjectParams {
    pub name: Option<String>,
    pub internal_abs_path: PathBuf,
    pub external_abs_path: Option<PathBuf>,
    pub git_params: Option<CreateProjectGitParams>,
    pub icon_path: Option<PathBuf>,
}

#[async_trait]

pub trait ProjectBackend: Send + Sync {
    async fn read_project_config(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
    ) -> joinerror::Result<ProjectConfig>;

    async fn create_project_manifest(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
    ) -> joinerror::Result<ProjectManifest>;

    async fn create_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: CreateProjectParams,
    ) -> joinerror::Result<()>;
}
