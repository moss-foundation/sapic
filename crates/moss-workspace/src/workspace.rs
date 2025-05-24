use anyhow::{Context, Result};
use moss_activity_indicator::ActivityIndicator;
use moss_collection::collection::Collection;
use moss_common::models::primitives::Identifier;
use moss_environment::environment::Environment;
use moss_file::toml::EditableToml;
use moss_fs::FileSystem;
use moss_storage::{
    WorkspaceStorage,
    primitives::segkey::SegmentExt,
    storage::operations::ListByPrefix,
    workspace_storage::{
        WorkspaceStorageImpl, entities::collection_store_entities::CollectionCacheEntity,
    },
};
use moss_text::sanitized::desanitize;
use std::{
    collections::HashMap,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::{OnceCell, RwLock};
use uuid::Uuid;

use crate::{
    defaults, dirs,
    manifest::{MANIFEST_FILE_NAME, ManifestModel, ManifestModelDiff},
    storage::segments::COLLECTION_SEGKEY,
};

pub struct CollectionItem {
    pub id: Uuid,
    pub name: String,
    pub order: Option<usize>,
    pub inner: Collection,
}

impl Deref for CollectionItem {
    type Target = Collection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct EnvironmentItem {
    pub id: Identifier,
    pub name: String,
    pub display_name: String,
    pub order: Option<usize>,
    pub inner: Environment,
}

impl Deref for EnvironmentItem {
    type Target = Environment;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

type CollectionMap = HashMap<Uuid, Arc<RwLock<CollectionItem>>>;
type EnvironmentMap = HashMap<Identifier, Arc<EnvironmentItem>>;

pub struct WorkspaceSummary {
    pub manifest: ManifestModel,
}

pub struct Workspace<R: TauriRuntime> {
    #[allow(dead_code)]
    pub(super) app_handle: AppHandle<R>,
    pub(super) abs_path: Arc<Path>,
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) workspace_storage: Arc<dyn WorkspaceStorage>,
    pub(super) collections: OnceCell<RwLock<CollectionMap>>,
    #[allow(dead_code)]
    pub(super) environments: OnceCell<RwLock<EnvironmentMap>>,
    #[allow(dead_code)]
    pub(super) activity_indicator: ActivityIndicator<R>,
    pub(super) next_collection_entry_id: Arc<AtomicUsize>,
    #[allow(dead_code)]
    pub(super) next_variable_id: Arc<AtomicUsize>,
    #[allow(dead_code)]
    pub(super) next_environment_id: Arc<AtomicUsize>,

    #[allow(dead_code)]
    pub(super) manifest: EditableToml<ManifestModel>,
}

pub struct CreateParams {
    pub name: Option<String>,
}

pub struct ModifyParams {
    pub name: Option<String>,
}

impl<R: TauriRuntime> Workspace<R> {
    pub async fn load(
        app_handle: AppHandle<R>,
        abs_path: &Path,
        fs: Arc<dyn FileSystem>,
        activity_indicator: ActivityIndicator<R>,
    ) -> Result<Self> {
        let state_db_manager = WorkspaceStorageImpl::new(&abs_path)
            .context("Failed to load the workspace state database")?;

        let abs_path: Arc<Path> = abs_path.to_owned().into();
        let manifest = EditableToml::load(fs.clone(), abs_path.join(MANIFEST_FILE_NAME)).await?;

        Ok(Self {
            app_handle,
            abs_path,
            fs,
            workspace_storage: Arc::new(state_db_manager),
            collections: OnceCell::new(),
            environments: OnceCell::new(),
            activity_indicator,
            next_collection_entry_id: Arc::new(AtomicUsize::new(0)),
            next_variable_id: Arc::new(AtomicUsize::new(0)),
            next_environment_id: Arc::new(AtomicUsize::new(0)),
            manifest,
        })
    }

    pub async fn create(
        app_handle: AppHandle<R>,
        abs_path: &Path,
        fs: Arc<dyn FileSystem>,
        activity_indicator: ActivityIndicator<R>,
        params: CreateParams,
    ) -> Result<Self> {
        let state_db_manager = WorkspaceStorageImpl::new(&abs_path)
            .context("Failed to open the workspace state database")?;

        let abs_path: Arc<Path> = abs_path.to_owned().into();
        let manifest = EditableToml::new(
            fs.clone(),
            abs_path.join(MANIFEST_FILE_NAME),
            ManifestModel {
                name: params
                    .name
                    .unwrap_or(defaults::DEFAULT_WORKSPACE_NAME.to_string()),
            },
        )
        .await?;

        Ok(Self {
            app_handle,
            abs_path,
            fs,
            workspace_storage: Arc::new(state_db_manager),
            collections: OnceCell::new(),
            environments: OnceCell::new(),
            activity_indicator,
            next_collection_entry_id: Arc::new(AtomicUsize::new(0)),
            next_variable_id: Arc::new(AtomicUsize::new(0)),
            next_environment_id: Arc::new(AtomicUsize::new(0)),
            manifest,
        })
    }

    pub async fn modify(&self, params: ModifyParams) -> Result<()> {
        if params.name.is_some() {
            self.manifest
                .edit(ManifestModelDiff {
                    name: params.name.to_owned(),
                })
                .await?;
        }

        Ok(())
    }

    pub async fn summary(fs: &Arc<dyn FileSystem>, abs_path: &Path) -> Result<WorkspaceSummary> {
        let manifest = EditableToml::load(fs.clone(), abs_path.join(MANIFEST_FILE_NAME)).await?;
        Ok(WorkspaceSummary {
            manifest: manifest.model().await,
        })
    }

    pub async fn manifest(&self) -> ManifestModel {
        self.manifest.model().await
    }

    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    pub(super) fn absolutize<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.abs_path.join(path)
    }

    #[allow(dead_code)]
    async fn environments(&self) -> Result<&RwLock<EnvironmentMap>> {
        let result = self
            .environments
            .get_or_try_init(|| async move {
                let mut environments = HashMap::new();

                let abs_path = self.abs_path.join(dirs::ENVIRONMENTS_DIR);
                if !abs_path.exists() {
                    return Ok(RwLock::new(environments));
                }

                // TODO: restore environments cache from the database
                let mut read_dir = self.fs.read_dir(&abs_path).await?;
                while let Some(entry) = read_dir.next_entry().await? {
                    if entry.file_type().await?.is_dir() {
                        continue;
                    }

                    let entry_abs_path: Arc<Path> = entry.path().into();
                    let name = entry_abs_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string();
                    let decoded_name = desanitize(&name);

                    let environment = Environment::new(
                        entry_abs_path,
                        self.fs.clone(),
                        self.workspace_storage.variable_store().clone(),
                        self.next_variable_id.clone(),
                    )
                    .await?;

                    let id = Identifier::new(&self.next_environment_id);
                    let entry = EnvironmentItem {
                        id,
                        name,
                        display_name: decoded_name,
                        order: None,
                        inner: environment,
                    };

                    environments.insert(id, Arc::new(entry));
                }

                Ok::<_, anyhow::Error>(RwLock::new(environments))
            })
            .await?;

        Ok(result)
    }

    pub async fn collections(&self) -> Result<&RwLock<CollectionMap>> {
        let result = self
            .collections
            .get_or_try_init(|| async move {
                let dir_abs_path = self.abs_path.join(dirs::COLLECTIONS_DIR);
                let mut collections = HashMap::new();

                if !dir_abs_path.exists() {
                    return Ok(RwLock::new(collections));
                }

                let restored_items = ListByPrefix::list_by_prefix(
                    self.workspace_storage.item_store().as_ref(),
                    COLLECTION_SEGKEY.as_str().expect("invalid utf-8"),
                )?;

                let filtered_restored_items = restored_items.iter().filter_map(|(k, v)| {
                    let path = k.after(&COLLECTION_SEGKEY);
                    if let Some(path) = path {
                        Some((path, v))
                    } else {
                        None
                    }
                });

                let mut restored_entities = HashMap::with_capacity(restored_items.len());
                for (segkey, value) in filtered_restored_items {
                    let id_str = match String::from_utf8(segkey.as_bytes().to_owned()) {
                        Ok(id) => id,
                        Err(_) => {
                            // TODO: logging
                            println!("failed to get the workspace {:?} name", segkey);
                            continue;
                        }
                    };

                    restored_entities.insert(id_str, value);
                }

                let mut read_dir = self.fs.read_dir(&dir_abs_path).await?;
                while let Some(entry) = read_dir.next_entry().await? {
                    if !entry.file_type().await?.is_dir() {
                        continue;
                    }

                    let id_str = entry.file_name().to_string_lossy().to_string();
                    let id = match Uuid::parse_str(&id_str) {
                        Ok(id) => id,
                        Err(_) => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", id_str);
                            continue;
                        }
                    };

                    let cache = match restored_entities.remove(&id_str).map_or(Ok(None), |v| {
                        v.deserialize::<CollectionCacheEntity>().map(Some)
                    }) {
                        Ok(value) => value,
                        Err(_err) => {
                            // TODO: logging
                            println!("failed to get the collection {:?} info", id_str);
                            continue;
                        }
                    };

                    let collection = Collection::load(
                        &entry.path(),
                        self.fs.clone(),
                        self.next_collection_entry_id.clone(),
                    )
                    .await?;
                    let manifest = collection.manifest().await;

                    collections.insert(
                        id,
                        Arc::new(RwLock::new(CollectionItem {
                            id,
                            name: manifest.name,
                            order: cache.map(|v| v.order).flatten(),
                            inner: collection,
                        })),
                    );
                }

                Ok::<_, anyhow::Error>(RwLock::new(collections))
            })
            .await?;

        Ok(result)
    }

    // TODO: Feature flag for test utils
    pub fn _storage(&self) -> Arc<dyn WorkspaceStorage> {
        self.workspace_storage.clone()
    }
}
