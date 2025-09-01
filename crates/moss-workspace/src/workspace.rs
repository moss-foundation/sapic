use anyhow::Result;
use joinerror::ResultExt;
use json_patch::{PatchOperation, ReplaceOperation};
use jsonptr::PointerBuf;
use moss_activity_broadcaster::ActivityBroadcaster;
use moss_applib::{
    AppRuntime,
    subscription::{Event, Subscription},
};
use moss_collection::Collection;
use moss_edit::json::EditOptions;
use moss_environment::{AnyEnvironment, Environment, models::primitives::EnvironmentId};
use moss_fs::{FileSystem, FsResultExt};
use moss_user::profile::ActiveProfile;
use serde_json::Value as JsonValue;
use std::{path::Path, sync::Arc};

use crate::{
    builder::{OnDidAddCollection, OnDidDeleteCollection},
    edit::WorkspaceEdit,
    manifest::{MANIFEST_FILE_NAME, ManifestFile},
    models::primitives::CollectionId,
    services::{
        collection_service::CollectionService, environment_service::EnvironmentService,
        layout_service::LayoutService, storage_service::StorageService,
    },
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

pub trait AnyWorkspace<R: AppRuntime> {
    type Collection;
    type Environment: AnyEnvironment<R>;
}

pub struct Workspace<R: AppRuntime> {
    pub(super) abs_path: Arc<Path>,

    #[allow(dead_code)]
    pub(super) broadcaster: ActivityBroadcaster<R::EventLoop>,
    pub(super) edit: WorkspaceEdit,
    pub(super) active_profile: Arc<ActiveProfile>,
    pub(super) layout_service: LayoutService<R>,
    pub(super) collection_service: Arc<CollectionService<R>>,
    pub(super) environment_service: Arc<EnvironmentService<R>>,
    pub(super) storage_service: Arc<StorageService<R>>,

    pub(super) _on_did_delete_collection: Subscription<OnDidDeleteCollection>,
    pub(super) _on_did_add_collection: Subscription<OnDidAddCollection>,
}

impl<R: AppRuntime> AnyWorkspace<R> for Workspace<R> {
    type Collection = Collection<R>;
    type Environment = Environment<R>;
}

impl<R: AppRuntime> Workspace<R> {
    pub(super) async fn on_did_add_collection(
        collection_service: Arc<CollectionService<R>>,
        environment_service: Arc<EnvironmentService<R>>,
        on_did_add_collection_event: &Event<OnDidAddCollection>,
    ) -> Subscription<OnDidAddCollection> {
        on_did_add_collection_event
            .subscribe(move |event| {
                let collection_service_clone = collection_service.clone();
                let environment_service_clone = environment_service.clone();

                async move {
                    let collection = collection_service_clone
                        .collection(&event.collection_id)
                        .await;

                    if let Some(collection) = collection {
                        environment_service_clone
                            .add_source(event.collection_id.inner(), collection.environments_path())
                            .await;
                    } else {
                        unreachable!()
                    }
                }
            })
            .await
    }

    pub(super) async fn on_did_delete_collection(
        environment_service: Arc<EnvironmentService<R>>,
        on_did_delete_collection_event: &Event<OnDidDeleteCollection>,
    ) -> Subscription<OnDidDeleteCollection> {
        on_did_delete_collection_event
            .subscribe(move |event| {
                let environment_service_clone = environment_service.clone();

                async move {
                    environment_service_clone
                        .remove_source(&event.collection_id.inner())
                        .await;
                }
            })
            .await
    }
}

impl<R: AppRuntime> Workspace<R> {
    pub fn abs_path(&self) -> &Path {
        &self.abs_path
    }

    pub async fn collection(&self, id: &CollectionId) -> Option<Arc<Collection<R>>> {
        self.collection_service.collection(id).await
    }

    pub async fn environment(&self, id: &EnvironmentId) -> Option<Arc<Environment<R>>> {
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
            self._on_did_add_collection.unsubscribe().await;
            self._on_did_delete_collection.unsubscribe().await;
        }
    }
}

#[cfg(any(test, feature = "integration-tests"))]
impl<R: AppRuntime> Workspace<R> {
    pub fn db(&self) -> &Arc<dyn moss_storage::WorkspaceStorage<R::AsyncContext>> {
        self.storage_service.storage()
    }
}
