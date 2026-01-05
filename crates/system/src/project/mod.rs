pub mod project_edit_service;
pub mod project_service;

use async_trait::async_trait;
use moss_git::{repository::Repository, url::GitUrl};
use sapic_base::{
    other::GitProviderKind,
    project::{config::ProjectConfig, manifest::ProjectManifest, types::primitives::ProjectId},
    user::types::primitives::AccountId,
};
use sapic_core::context::AnyAsyncContext;
use std::path::PathBuf;

use crate::user::account::Account;

pub struct CreateConfigParams {
    pub external_abs_path: Option<PathBuf>,
    pub account_id: Option<AccountId>,
}

#[derive(Clone)]
pub struct CreateProjectGitParams {
    pub provider_kind: GitProviderKind,
    pub repository_url: GitUrl,
    pub branch_name: String,
}

pub struct CreateProjectParams {
    pub name: Option<String>,
    pub external_abs_path: Option<PathBuf>,
    pub git_params: Option<CreateProjectGitParams>,
    pub icon_path: Option<PathBuf>,
}

pub struct ImportArchivedProjectParams {
    pub archive_path: PathBuf,
}

pub struct ImportExternalProjectParams {
    pub external_abs_path: PathBuf,
}

pub struct CloneProjectGitParams {
    pub provider_kind: GitProviderKind,
    pub repository_url: GitUrl,
    pub branch_name: Option<String>,
}
pub struct CloneProjectParams {
    pub git_params: CloneProjectGitParams,
}

pub struct ExportArchiveParams {
    pub archive_path: PathBuf,
}

pub struct LookedUpProject {
    pub id: ProjectId,
    pub abs_path: PathBuf,
    pub manifest: ProjectManifest,
    pub config: ProjectConfig,
}

#[async_trait]
pub trait ProjectServiceFs: Send + Sync {
    async fn lookup_projects(
        &self,
        ctx: &dyn AnyAsyncContext,
    ) -> joinerror::Result<Vec<LookedUpProject>>;

    // FIXME: I think this is probably incorrect but I want to prioritize migration
    // Reading and Creating file-based config and manifest should be platform dependent
    // So they probably should not be in the system-level interface
    async fn read_project_config(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<ProjectConfig>;

    async fn read_project_manifest(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<ProjectManifest>;

    async fn create_project_manifest(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<ProjectManifest>;

    async fn create_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: CreateProjectParams,
    ) -> joinerror::Result<PathBuf>;

    async fn clone_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        account: &Account,
        params: CloneProjectParams,
    ) -> joinerror::Result<(Repository, PathBuf)>;

    async fn import_archived_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ImportArchivedProjectParams,
    ) -> joinerror::Result<PathBuf>;

    async fn import_external_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ImportExternalProjectParams,
    ) -> joinerror::Result<PathBuf>;

    async fn export_archive(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ExportArchiveParams,
    ) -> joinerror::Result<()>;

    async fn delete_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
    ) -> joinerror::Result<Option<PathBuf>>;
}

pub struct ProjectEditParams {
    pub name: Option<String>,
}

pub struct ProjectConfigEditParams {
    pub archived: Option<bool>,
}

#[async_trait]
pub trait ProjectEditBackend: Send + Sync {
    async fn edit(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ProjectEditParams,
    ) -> joinerror::Result<()>;

    async fn edit_config(
        &self,
        ctx: &dyn AnyAsyncContext,
        id: &ProjectId,
        params: ProjectConfigEditParams,
    ) -> joinerror::Result<()>;
}
