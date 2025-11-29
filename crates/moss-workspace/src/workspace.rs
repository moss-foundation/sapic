use anyhow::Result;
use joinerror::ResultExt;
use json_patch::{PatchOperation, ReplaceOperation};
use jsonptr::PointerBuf;
use moss_edit::json::EditOptions;
use moss_environment::{AnyEnvironment, Environment};
use moss_fs::{FileSystem, FsResultExt};
use moss_project::{Project, models::primitives::ProjectId};
use sapic_base::{
    environment::types::primitives::EnvironmentId,
    workspace::{manifest::ManifestFile, types::primitives::WorkspaceId},
};
use sapic_core::subscription::{Event, Subscription};
use sapic_system::user::profile::Profile;
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};

use crate::{
    builder::{OnDidAddProject, OnDidDeleteProject},
    edit::WorkspaceEdit,
    environment::EnvironmentService,
    manifest::MANIFEST_FILE_NAME,
    project::ProjectService,
};

pub struct WorkspaceSummary {
    pub name: String,
}

impl WorkspaceSummary {
    pub async fn new(fs: &Arc<dyn FileSystem>, abs_path: &Path) -> joinerror::Result<Self> {
        debug_assert!(abs_path.is_absolute());

        let manifest_path = abs_path.join(MANIFEST_FILE_NAME);

        let rdr = fs.open_file(&manifest_path).await.join_err_with::<()>(|| {
            format!("failed to open manifest file: {}", manifest_path.display())
        })?;

        let manifest: ManifestFile = serde_json::from_reader(rdr).join_err_with::<()>(|| {
            format!("failed to parse manifest file: {}", manifest_path.display())
        })?;

        Ok(WorkspaceSummary {
            name: manifest.name,
        })
    }
}

#[derive(Clone)]
pub struct WorkspaceModifyParams {
    pub name: Option<String>,
}

pub trait AnyWorkspace {
    type Project;
    type Environment: AnyEnvironment;
}

// DEPRECATED
pub struct Workspace {
    pub(super) id: WorkspaceId,
    pub(super) abs_path: Arc<Path>,
    pub(super) edit: WorkspaceEdit,
    pub(super) active_profile: Arc<Profile>,
    pub(super) project_service: Arc<ProjectService>,
    pub(super) environment_service: Arc<EnvironmentService>,
    pub(super) _on_did_delete_project: Subscription<OnDidDeleteProject>,
    pub(super) _on_did_add_project: Subscription<OnDidAddProject>,
}

impl AnyWorkspace for Workspace {
    type Project = Project;
    type Environment = Environment;
}

impl Workspace {
    pub(super) async fn on_did_add_project(
        project_service: Arc<ProjectService>,
        environment_service: Arc<EnvironmentService>,
        on_did_add_project_event: &Event<OnDidAddProject>,
    ) -> Subscription<OnDidAddProject> {
        on_did_add_project_event
            .subscribe(move |event| {
                let project_service_clone = project_service.clone();
                let environment_service_clone = environment_service.clone();
                async move {
                    let project = project_service_clone.project(&event.project_id).await;

                    if let Some(project) = project {
                        environment_service_clone
                            .add_source(event.project_id.inner(), project.environments_path())
                            .await;
                    } else {
                        unreachable!()
                    }
                }
            })
            .await
    }

    pub(super) async fn on_did_delete_project(
        environment_service: Arc<EnvironmentService>,
        on_did_delete_project_event: &Event<OnDidDeleteProject>,
    ) -> Subscription<OnDidDeleteProject> {
        on_did_delete_project_event
            .subscribe(move |event| {
                let environment_service_clone = environment_service.clone();
                async move {
                    environment_service_clone
                        .remove_source(&event.project_id.inner())
                        .await;
                }
            })
            .await
    }
}

impl Workspace {
    pub fn id(&self) -> WorkspaceId {
        self.id.clone()
    }
    pub fn abs_path(&self) -> &Path {
        &self.abs_path
    }

    pub async fn project(&self, id: &ProjectId) -> Option<Arc<Project>> {
        self.project_service.project(id).await
    }

    pub async fn environment(&self, id: &EnvironmentId) -> Option<Arc<Environment>> {
        self.environment_service.environment(id).await
    }

    pub async fn modify(&self, params: WorkspaceModifyParams) -> Result<()> {
        let mut patches = Vec::new();

        if let Some(new_name) = params.name {
            patches.push((
                PatchOperation::Replace(ReplaceOperation {
                    path: unsafe { PointerBuf::new_unchecked("/name") },
                    value: JsonValue::String(new_name),
                }),
                EditOptions {
                    ignore_if_not_exists: false,
                    create_missing_segments: false,
                },
            ));
        }

        self.edit
            .edit(&patches)
            .await
            .join_err::<()>("failed to edit workspace")?;
        Ok(())
    }

    pub async fn dispose(&self) {
        // We need to unsubscribe from the events to avoid circular references
        {
            self._on_did_add_project.unsubscribe().await;
            self._on_did_delete_project.unsubscribe().await;
        }
    }
}
