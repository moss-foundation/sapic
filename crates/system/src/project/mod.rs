pub mod project_edit_service;
pub mod project_service;

use async_trait::async_trait;
use moss_bindingutils::primitives::ChangeString;
use moss_git::{repository::Repository, url::GitUrl};
use sapic_base::{
    other::GitProviderKind,
    project::{config::ProjectConfig, manifest::ProjectManifest, types::primitives::ProjectId},
    user::types::primitives::AccountId,
};
use sapic_core::context::AnyAsyncContext;
use std::path::{Path, PathBuf};

use crate::user::account::Account;

pub struct CreateConfigParams {
    pub internal_abs_path: PathBuf,
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
    pub internal_abs_path: PathBuf,
    pub external_abs_path: Option<PathBuf>,
    pub git_params: Option<CreateProjectGitParams>,
    pub icon_path: Option<PathBuf>,
}

pub struct ImportArchivedProjectParams {
    pub internal_abs_path: PathBuf,
    pub archive_path: PathBuf,
}

pub struct ImportExternalProjectParams {
    pub internal_abs_path: PathBuf,
    pub external_abs_path: PathBuf,
}

pub struct CloneProjectGitParams {
    pub provider_kind: GitProviderKind,
    pub repository_url: GitUrl,
    pub branch_name: Option<String>,
}
pub struct CloneProjectParams {
    pub internal_abs_path: PathBuf,
    pub git_params: CloneProjectGitParams,
}

pub struct ExportArchiveParams {
    pub project_path: PathBuf,
    pub archive_path: PathBuf,
}

#[async_trait]
pub trait ProjectBackend: Send + Sync {
    async fn read_project_config(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
    ) -> joinerror::Result<ProjectConfig>;

    async fn read_project_manifest(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
    ) -> joinerror::Result<ProjectManifest>;

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

    async fn clone_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        account: &Account,
        params: CloneProjectParams,
    ) -> joinerror::Result<Repository>;

    async fn import_archived_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ImportArchivedProjectParams,
    ) -> joinerror::Result<()>;

    async fn import_external_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ImportExternalProjectParams,
    ) -> joinerror::Result<()>;

    async fn export_archive(
        &self,
        ctx: &dyn AnyAsyncContext,
        params: ExportArchiveParams,
    ) -> joinerror::Result<()>;

    async fn delete_project(
        &self,
        ctx: &dyn AnyAsyncContext,
        abs_path: &Path,
    ) -> joinerror::Result<Option<PathBuf>>;
}

pub struct ProjectEditParams {
    pub name: Option<String>,
    pub repository: Option<ChangeString>,
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
