use joinerror::ResultExt;
use moss_activity_indicator::ActivityIndicator;
use moss_applib::{AppRuntime, EventMarker, subscription::EventEmitter};
use moss_environment::builder::{CreateEnvironmentParams, EnvironmentBuilder};
use moss_environment_provider::EnvironmentProvider;
use moss_fs::{CreateOptions, FileSystem, FsResultExt};
use moss_git_hosting_provider::{github::client::GitHubClient, gitlab::client::GitLabClient};
use rustc_hash::FxHashMap;
use std::{cell::LazyCell, path::Path, sync::Arc};
use tokio::sync::{mpsc, watch};

use crate::{
    Workspace, dirs,
    edit::WorkspaceEdit,
    manifest::{MANIFEST_FILE_NAME, ManifestFile},
    models::primitives::CollectionId,
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

#[derive(Clone)]
pub struct OnDidDeleteCollection {
    pub collection_id: CollectionId,
}

#[derive(Clone)]
pub struct OnDidAddCollection {
    pub collection_id: CollectionId,
}

impl EventMarker for OnDidDeleteCollection {}
impl EventMarker for OnDidAddCollection {}

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

        let mut environment_sources = FxHashMap::from_iter([(
            "".to_string().into(),
            params.abs_path.join(dirs::ENVIRONMENTS_DIR),
        )]);

        let on_did_delete_collection_emitter = EventEmitter::<OnDidDeleteCollection>::new();
        let on_did_add_collection_emitter = EventEmitter::<OnDidAddCollection>::new();

        let on_did_delete_collection_event = on_did_delete_collection_emitter.event();
        let on_did_add_collection_event = on_did_add_collection_emitter.event();

        let storage_service: Arc<StorageService<R>> = StorageService::new(&params.abs_path)?.into();
        let layout_service = LayoutService::new(storage_service.clone());

        let collection_service: Arc<CollectionService<R>> = CollectionService::new(
            ctx,
            &params.abs_path,
            self.fs.clone(),
            storage_service.clone(),
            self.github_client.clone(),
            self.gitlab_client.clone(),
            &mut environment_sources,
            on_did_delete_collection_emitter,
            on_did_add_collection_emitter,
        )
        .await?
        .into();

        let environment_service: Arc<EnvironmentService<R>> = EnvironmentService::new(
            &params.abs_path,
            self.fs.clone(),
            storage_service.clone(),
            environment_sources,
        )
        .await?
        .into();

        let edit = WorkspaceEdit::new(self.fs.clone(), params.abs_path.join(MANIFEST_FILE_NAME));

        let on_did_add_collection = Workspace::on_did_add_collection(
            collection_service.clone(),
            environment_service.clone(),
            &on_did_add_collection_event,
        )
        .await;

        let on_did_delete_collection = Workspace::on_did_delete_collection(
            environment_service.clone(),
            &on_did_delete_collection_event,
        )
        .await;

        Ok(Workspace {
            abs_path: params.abs_path,
            activity_indicator,
            edit,
            layout_service,
            collection_service,
            environment_service,
            storage_service,
            github_client: self.github_client,
            gitlab_client: self.gitlab_client,
            _on_did_add_collection: on_did_add_collection,
            _on_did_delete_collection: on_did_delete_collection,
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

        let mut environment_sources = FxHashMap::from_iter([(
            "".to_string().into(),
            params.abs_path.join(dirs::ENVIRONMENTS_DIR),
        )]);

        let on_did_delete_collection_emitter = EventEmitter::<OnDidDeleteCollection>::new();
        let on_did_add_collection_emitter = EventEmitter::<OnDidAddCollection>::new();

        let on_did_delete_collection_event = on_did_delete_collection_emitter.event();
        let on_did_add_collection_event = on_did_add_collection_emitter.event();

        let storage_service: Arc<StorageService<R>> = StorageService::new(&params.abs_path)?.into();
        let layout_service = LayoutService::new(storage_service.clone());
        let collection_service: Arc<CollectionService<R>> = CollectionService::new(
            ctx,
            &params.abs_path,
            self.fs.clone(),
            storage_service.clone(),
            self.github_client.clone(),
            self.gitlab_client.clone(),
            &mut environment_sources,
            on_did_delete_collection_emitter,
            on_did_add_collection_emitter,
        )
        .await?
        .into();

        let environment_service: Arc<EnvironmentService<R>> = EnvironmentService::new(
            &params.abs_path,
            self.fs.clone(),
            storage_service.clone(),
            environment_sources,
        )
        .await?
        .into();

        let edit = WorkspaceEdit::new(self.fs.clone(), params.abs_path.join(MANIFEST_FILE_NAME));

        let on_did_add_collection = Workspace::on_did_add_collection(
            collection_service.clone(),
            environment_service.clone(),
            &on_did_add_collection_event,
        )
        .await;

        let on_did_delete_collection = Workspace::on_did_delete_collection(
            environment_service.clone(),
            &on_did_delete_collection_event,
        )
        .await;

        Ok(Workspace {
            abs_path: params.abs_path,
            activity_indicator,
            edit,
            layout_service,
            collection_service,
            environment_service,
            storage_service,
            github_client: self.github_client,
            gitlab_client: self.gitlab_client,
            _on_did_add_collection: on_did_add_collection,
            _on_did_delete_collection: on_did_delete_collection,
        })
    }
}
