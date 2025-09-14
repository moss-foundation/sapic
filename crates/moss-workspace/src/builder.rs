use joinerror::ResultExt;
use moss_app_delegate::AppDelegate;
use moss_applib::{AppRuntime, EventMarker, subscription::EventEmitter};
use moss_environment::builder::{CreateEnvironmentParams, EnvironmentBuilder};
use moss_fs::{CreateOptions, FileSystem, FsResultExt};
use moss_user::profile::Profile;
use rustc_hash::FxHashMap;
use std::{cell::LazyCell, path::Path, sync::Arc};

use crate::{
    Workspace, dirs,
    edit::WorkspaceEdit,
    manifest::{MANIFEST_FILE_NAME, ManifestFile},
    models::primitives::ProjectId,
    services::{
        environment_service::EnvironmentService, layout_service::LayoutService,
        project_service::ProjectService, storage_service::StorageService,
    },
};

struct PredefinedEnvironment {
    name: String,
    color: Option<String>,
}

const PREDEFINED_ENVIRONMENTS: LazyCell<Vec<PredefinedEnvironment>> = LazyCell::new(|| {
    vec![PredefinedEnvironment {
        name: "Globals".to_string(),
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

pub struct WorkspaceBuilder<R: AppRuntime> {
    fs: Arc<dyn FileSystem>,
    active_profile: Arc<Profile<R>>,
}

#[derive(Clone)]
pub struct OnDidDeleteProject {
    pub project_id: ProjectId,
}

#[derive(Clone)]
pub struct OnDidAddProject {
    pub project_id: ProjectId,
}

impl EventMarker for OnDidDeleteProject {}
impl EventMarker for OnDidAddProject {}

impl<R: AppRuntime> WorkspaceBuilder<R> {
    pub fn new(fs: Arc<dyn FileSystem>, active_profile: Arc<Profile<R>>) -> Self {
        Self { fs, active_profile }
    }

    pub async fn initialize(
        fs: Arc<dyn FileSystem>,
        params: CreateWorkspaceParams,
    ) -> joinerror::Result<()> {
        debug_assert!(params.abs_path.is_absolute());

        for dir in &[dirs::PROJECTS_DIR, dirs::ENVIRONMENTS_DIR] {
            fs.create_dir(&params.abs_path.join(dir)).await?;
        }

        fs.create_file_with(
            &params.abs_path.join(MANIFEST_FILE_NAME),
            serde_json::to_string(&ManifestFile { name: params.name })?.as_bytes(),
            CreateOptions::default(),
        )
        .await
        .join_err::<()>("failed to create manifest file")?;

        for env in PREDEFINED_ENVIRONMENTS.iter() {
            EnvironmentBuilder::new(fs.clone())
                .initialize(CreateEnvironmentParams {
                    name: env.name.clone(),
                    abs_path: &params.abs_path.join(dirs::ENVIRONMENTS_DIR),
                    color: env.color.clone(),
                    variables: vec![],
                })
                .await
                .join_err_with::<()>(|| {
                    format!("failed to initialize environment `{}`", env.name)
                })?;
        }

        Ok(())
    }

    pub async fn load(
        self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        params: LoadWorkspaceParams,
    ) -> joinerror::Result<Workspace<R>> {
        debug_assert!(params.abs_path.is_absolute());

        let mut environment_sources = FxHashMap::from_iter([(
            "".to_string().into(),
            params.abs_path.join(dirs::ENVIRONMENTS_DIR),
        )]);

        let on_did_delete_collection_emitter = EventEmitter::<OnDidDeleteProject>::new();
        let on_did_add_collection_emitter = EventEmitter::<OnDidAddProject>::new();

        let on_did_delete_collection_event = on_did_delete_collection_emitter.event();
        let on_did_add_collection_event = on_did_add_collection_emitter.event();

        let storage_service: Arc<StorageService<R>> = StorageService::new(&params.abs_path)
            .join_err::<()>("failed to create storage service")?
            .into();
        let layout_service = LayoutService::new(storage_service.clone());

        let collection_service: Arc<ProjectService<R>> = ProjectService::new(
            ctx,
            app_delegate,
            &params.abs_path,
            self.fs.clone(),
            storage_service.clone(),
            &mut environment_sources,
            &self.active_profile,
            on_did_delete_collection_emitter,
            on_did_add_collection_emitter,
        )
        .await
        .join_err::<()>("failed to create collection service")?
        .into();

        let environment_service: Arc<EnvironmentService<R>> = EnvironmentService::new(
            &params.abs_path,
            self.fs.clone(),
            storage_service.clone(),
            environment_sources,
        )
        .await
        .join_err::<()>("failed to create environment service")?
        .into();

        let edit = WorkspaceEdit::new(self.fs.clone(), params.abs_path.join(MANIFEST_FILE_NAME));

        let on_did_add_collection = Workspace::on_did_add_project(
            collection_service.clone(),
            environment_service.clone(),
            &on_did_add_collection_event,
        )
        .await;

        let on_did_delete_collection = Workspace::on_did_delete_project(
            environment_service.clone(),
            &on_did_delete_collection_event,
        )
        .await;

        Ok(Workspace {
            abs_path: params.abs_path,
            edit,
            layout_service,
            project_service: collection_service,
            environment_service,
            storage_service,
            active_profile: self.active_profile,
            _on_did_add_project: on_did_add_collection,
            _on_did_delete_project: on_did_delete_collection,
        })
    }

    pub async fn create(
        self,
        ctx: &R::AsyncContext,
        app_delegate: &AppDelegate<R>,
        params: CreateWorkspaceParams,
    ) -> joinerror::Result<Workspace<R>> {
        debug_assert!(params.abs_path.is_absolute());

        WorkspaceBuilder::<R>::initialize(self.fs.clone(), params.clone())
            .await
            .join_err::<()>("failed to initialize workspace")?;

        let mut environment_sources = FxHashMap::from_iter([(
            "".to_string().into(),
            params.abs_path.join(dirs::ENVIRONMENTS_DIR),
        )]);

        let on_did_delete_collection_emitter = EventEmitter::<OnDidDeleteProject>::new();
        let on_did_add_collection_emitter = EventEmitter::<OnDidAddProject>::new();

        let on_did_delete_collection_event = on_did_delete_collection_emitter.event();
        let on_did_add_collection_event = on_did_add_collection_emitter.event();

        let storage_service: Arc<StorageService<R>> = StorageService::new(&params.abs_path)?.into();
        let layout_service = LayoutService::new(storage_service.clone());
        let collection_service: Arc<ProjectService<R>> = ProjectService::new(
            ctx,
            app_delegate,
            &params.abs_path,
            self.fs.clone(),
            storage_service.clone(),
            &mut environment_sources,
            &self.active_profile,
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

        let on_did_add_collection = Workspace::on_did_add_project(
            collection_service.clone(),
            environment_service.clone(),
            &on_did_add_collection_event,
        )
        .await;

        let on_did_delete_collection = Workspace::on_did_delete_project(
            environment_service.clone(),
            &on_did_delete_collection_event,
        )
        .await;

        Ok(Workspace {
            abs_path: params.abs_path,
            edit,
            layout_service,
            project_service: collection_service,
            environment_service,
            storage_service,
            active_profile: self.active_profile,
            _on_did_add_project: on_did_add_collection,
            _on_did_delete_project: on_did_delete_collection,
        })
    }
}
