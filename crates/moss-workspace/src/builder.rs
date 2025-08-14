use joinerror::ResultExt;
use moss_activity_indicator::ActivityIndicator;
use moss_applib::AppRuntime;
use moss_environment::builder::{CreateEnvironmentParams, EnvironmentBuilder};
use moss_fs::{CreateOptions, FileSystem, FsResultExt};
use moss_git_hosting_provider::{github::client::GitHubClient, gitlab::client::GitLabClient};
use std::{cell::LazyCell, path::Path, sync::Arc};

use crate::{
    Workspace, dirs,
    edit::WorkspaceEdit,
    manifest::{MANIFEST_FILE_NAME, ManifestFile},
    services::{
        collection_service::CollectionService, environment_service::EnvironmentService,
        layout_service::LayoutService, storage_service::StorageService,
    },
};

struct PredefinedEnvironment {
    name: String,
    order: isize,
    color: Option<String>,
}

const PREDEFINED_ENVIRONMENTS: LazyCell<Vec<PredefinedEnvironment>> = LazyCell::new(|| {
    vec![PredefinedEnvironment {
        name: "Globals".to_string(),
        order: 0,
        color: Some("#3574F0".to_string()),
    }]
});

pub struct LoadWorkspaceParams {
    pub abs_path: Arc<Path>,
}

#[derive(Clone)]
pub struct CreateWorkspaceParams {
    pub name: String,
    pub abs_path: Arc<Path>,
}

pub struct WorkspaceBuilder {
    fs: Arc<dyn FileSystem>,
    github_client: Arc<GitHubClient>,
    gitlab_client: Arc<GitLabClient>,
}

impl WorkspaceBuilder {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        github_client: Arc<GitHubClient>,
        gitlab_client: Arc<GitLabClient>,
    ) -> Self {
        Self {
            fs,
            github_client,
            gitlab_client,
        }
    }

    pub async fn initialize(
        fs: Arc<dyn FileSystem>,
        params: CreateWorkspaceParams,
    ) -> joinerror::Result<()> {
        debug_assert!(params.abs_path.is_absolute());

        for dir in &[dirs::COLLECTIONS_DIR, dirs::ENVIRONMENTS_DIR] {
            fs.create_dir(&params.abs_path.join(dir)).await?;
        }

        fs.create_file_with(
            &params.abs_path.join(MANIFEST_FILE_NAME),
            serde_json::to_string(&ManifestFile { name: params.name })?.as_bytes(),
            CreateOptions::default(),
        )
        .await
        .join_err::<()>(format!("failed to create manifest file"))?;

        for env in PREDEFINED_ENVIRONMENTS.iter() {
            EnvironmentBuilder::new(fs.clone())
                .initialize(CreateEnvironmentParams {
                    name: env.name.clone(),
                    abs_path: &params.abs_path.join(dirs::ENVIRONMENTS_DIR),
                    color: env.color.clone(),
                    order: env.order,
                    variables: vec![],
                })
                .await
                .join_err_with::<()>(|| format!("failed to initialize environment {}", env.name))?;
        }

        Ok(())
    }

    pub async fn load<R: AppRuntime>(
        self,
        ctx: &R::AsyncContext,
        activity_indicator: ActivityIndicator<R::EventLoop>, // FIXME: will be passed as a service in the future
        params: LoadWorkspaceParams,
    ) -> joinerror::Result<Workspace<R>> {
        debug_assert!(params.abs_path.is_absolute());

        let storage_service: Arc<StorageService<R>> = StorageService::new(&params.abs_path)?.into();
        let layout_service = LayoutService::new(storage_service.clone());
        let collection_service = CollectionService::new(
            ctx,
            &params.abs_path,
            self.fs.clone(),
            storage_service.clone(),
            self.github_client.clone(),
            self.gitlab_client.clone(),
        )
        .await?;

        let environment_service = EnvironmentService::new(
            ctx,
            &params.abs_path,
            self.fs.clone(),
            storage_service.clone(),
        )
        .await?;

        let edit = WorkspaceEdit::new(self.fs.clone(), params.abs_path.join(MANIFEST_FILE_NAME));

        Ok(Workspace {
            abs_path: params.abs_path,
            activity_indicator,
            edit,
            layout_service,
            collection_service,
            environment_service,
            storage_service,
            _github_client: self.github_client,
            _gitlab_client: self.gitlab_client,
        })
    }

    pub async fn create<R: AppRuntime>(
        self,
        ctx: &R::AsyncContext,
        activity_indicator: ActivityIndicator<R::EventLoop>, // FIXME: will be passed as a service in the future
        params: CreateWorkspaceParams,
    ) -> joinerror::Result<Workspace<R>> {
        debug_assert!(params.abs_path.is_absolute());

        WorkspaceBuilder::initialize(self.fs.clone(), params.clone())
            .await
            .join_err::<()>("failed to initialize workspace")?;

        let storage_service: Arc<StorageService<R>> = StorageService::new(&params.abs_path)?.into();
        let layout_service = LayoutService::new(storage_service.clone());
        let collection_service = CollectionService::new(
            ctx,
            &params.abs_path,
            self.fs.clone(),
            storage_service.clone(),
            self.github_client.clone(),
            self.gitlab_client.clone(),
        )
        .await?;
        let environment_service = EnvironmentService::new(
            ctx,
            &params.abs_path,
            self.fs.clone(),
            storage_service.clone(),
        )
        .await?;

        let edit = WorkspaceEdit::new(self.fs.clone(), params.abs_path.join(MANIFEST_FILE_NAME));

        Ok(Workspace {
            abs_path: params.abs_path,
            activity_indicator,
            edit,
            layout_service,
            collection_service,
            environment_service,
            storage_service,
            _github_client: self.github_client,
            _gitlab_client: self.gitlab_client,
        })
    }
}
