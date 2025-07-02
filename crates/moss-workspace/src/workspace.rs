use anyhow::{Context as _, Result};
use derive_more::{Deref, DerefMut};
use moss_activity_indicator::ActivityIndicator;
use moss_applib::context::Context;
use moss_collection::{
    CollectionBuilder,
    builder::LoadParams,
    collection::Collection,
    services::{storage_service::StorageService, worktree_service::WorktreeService},
};
use moss_environment::environment::{self, Environment};
use moss_file::json::JsonFileHandle;
use moss_fs::FileSystem;
use moss_storage::{
    WorkspaceStorage, primitives::segkey::SegmentExt, storage::operations::ListByPrefix,
    workspace_storage::WorkspaceStorageImpl,
};
use moss_text::sanitized::desanitize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use tauri::Runtime as TauriRuntime;
use tokio::sync::{OnceCell, RwLock};

use crate::{
    defaults, dirs,
    layout::LayoutService,
    manifest::{MANIFEST_FILE_NAME, ManifestModel},
    storage::{entities::collection_store::CollectionCacheEntity, segments::COLLECTION_SEGKEY},
};

#[derive(Deref, DerefMut)]
pub struct CollectionItem {
    pub id: String,
    pub order: Option<usize>,
    #[deref]
    #[deref_mut]
    pub inner: Collection,
}

#[derive(Deref, DerefMut)]
pub struct EnvironmentItem {
    pub id: String,
    pub name: String,
    pub display_name: String,
    #[deref]
    #[deref_mut]
    pub inner: Environment,
}

type CollectionMap = HashMap<String, Arc<RwLock<CollectionItem>>>;
type EnvironmentMap = HashMap<String, Arc<EnvironmentItem>>;

pub struct WorkspaceSummary {
    pub manifest: ManifestModel,
}

pub struct Workspace<R: TauriRuntime> {
    pub(super) abs_path: Arc<Path>,
    pub(super) storage: Arc<dyn WorkspaceStorage>,
    pub(super) collections: OnceCell<CollectionMap>,
    pub(super) environments: OnceCell<EnvironmentMap>,
    #[allow(dead_code)]
    pub(super) activity_indicator: ActivityIndicator<R>,
    #[allow(dead_code)]
    pub(super) next_variable_id: Arc<AtomicUsize>,
    #[allow(dead_code)]
    pub(super) next_environment_id: Arc<AtomicUsize>,

    #[allow(dead_code)]
    pub(super) manifest: JsonFileHandle<ManifestModel>,

    pub layout: LayoutService,
}

pub struct CreateParams {
    pub name: Option<String>,
}

#[derive(Clone)]

pub struct ModifyParams {
    pub name: Option<String>,
}

impl<R: TauriRuntime> Workspace<R> {
    pub async fn load(
        fs: Arc<dyn FileSystem>,
        abs_path: &Path,
        activity_indicator: ActivityIndicator<R>,
    ) -> Result<Self> {
        let storage = {
            let storage = WorkspaceStorageImpl::new(&abs_path)
                .context("Failed to load the workspace state database")?;

            Arc::new(storage)
        };

        let abs_path: Arc<Path> = abs_path.to_owned().into();
        let manifest = JsonFileHandle::load(fs.clone(), &abs_path.join(MANIFEST_FILE_NAME)).await?;

        let layout = LayoutService::new(storage.clone());

        Ok(Self {
            abs_path,
            storage,
            collections: OnceCell::new(),
            environments: OnceCell::new(),
            activity_indicator,
            next_variable_id: Arc::new(AtomicUsize::new(0)),
            next_environment_id: Arc::new(AtomicUsize::new(0)),
            manifest,
            layout,
        })
    }

    pub async fn create(
        fs: Arc<dyn FileSystem>,
        abs_path: &Path,
        activity_indicator: ActivityIndicator<R>,
        params: CreateParams,
    ) -> Result<Self> {
        let storage = {
            let storage = WorkspaceStorageImpl::new(&abs_path)
                .context("Failed to create the workspace state database")?;

            Arc::new(storage)
        };

        let abs_path: Arc<Path> = abs_path.to_owned().into();

        for dir in &[dirs::COLLECTIONS_DIR, dirs::ENVIRONMENTS_DIR] {
            fs.create_dir(&abs_path.join(dir)).await?;
        }

        let manifest = JsonFileHandle::create(
            fs.clone(),
            &abs_path.join(MANIFEST_FILE_NAME),
            ManifestModel {
                name: params
                    .name
                    .unwrap_or(defaults::DEFAULT_WORKSPACE_NAME.to_string()),
            },
        )
        .await?;

        let layout = LayoutService::new(storage.clone());

        Ok(Self {
            abs_path,
            storage,
            collections: OnceCell::new(),
            environments: OnceCell::new(),
            activity_indicator,
            next_variable_id: Arc::new(AtomicUsize::new(0)),
            next_environment_id: Arc::new(AtomicUsize::new(0)),
            manifest,
            layout,
        })
    }

    pub async fn modify(&self, params: ModifyParams) -> Result<()> {
        if params.name.is_some() {
            self.manifest
                .edit(
                    |model| {
                        model.name = params.name.unwrap();
                        Ok(())
                    },
                    |model| {
                        serde_json::to_string(model).map_err(|err| {
                            anyhow::anyhow!("Failed to serialize JSON file: {}", err)
                        })
                    },
                )
                .await?;
        }
        Ok(())
    }

    pub async fn summary(fs: Arc<dyn FileSystem>, abs_path: &Path) -> Result<WorkspaceSummary> {
        let manifest = JsonFileHandle::load(fs, &abs_path.join(MANIFEST_FILE_NAME)).await?;
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

    pub async fn environments<C: Context<R>>(&self, ctx: &C) -> Result<&EnvironmentMap> {
        let fs = <dyn FileSystem>::global::<R, C>(ctx);
        let result = self
            .environments
            .get_or_try_init(|| async move {
                let mut environments = HashMap::new();

                let abs_path = self.abs_path.join(dirs::ENVIRONMENTS_DIR);
                if !abs_path.exists() {
                    return Ok(environments);
                }

                // TODO: restore environments cache from the database
                let mut read_dir = fs.read_dir(&abs_path).await?;
                while let Some(entry) = read_dir.next_entry().await? {
                    if entry.file_type().await?.is_dir() {
                        continue;
                    }

                    let entry_abs_path = entry.path();
                    let name = entry_abs_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .to_string();
                    let decoded_name = desanitize(&name);

                    let environment = Environment::load(
                        &entry_abs_path,
                        fs.clone(),
                        self.storage.variable_store().clone(),
                        self.next_variable_id.clone(),
                        environment::LoadParams {
                            create_if_not_exists: false,
                        },
                    )
                    .await?;

                    let id = environment.id().await;
                    let entry = EnvironmentItem {
                        id: id.clone(),
                        name,
                        display_name: decoded_name,
                        inner: environment,
                    };

                    environments.insert(id.clone(), Arc::new(entry));
                }

                Ok::<_, anyhow::Error>(environments)
            })
            .await?;

        Ok(result)
    }

    pub async fn collections_mut<C: Context<R>>(&mut self, ctx: &C) -> Result<&mut CollectionMap> {
        if !self.collections.initialized() {
            self.collections(ctx).await?;
        }

        Ok(self.collections.get_mut().unwrap())
    }

    pub async fn collections<C: Context<R>>(&self, ctx: &C) -> Result<&CollectionMap> {
        let fs = <dyn FileSystem>::global::<R, C>(ctx);
        let result = self
            .collections
            .get_or_try_init(|| async move {
                let dir_abs_path = self.abs_path.join(dirs::COLLECTIONS_DIR);
                let mut collections = HashMap::new();

                if !dir_abs_path.exists() {
                    return Ok(collections);
                }

                let restored_items = ListByPrefix::list_by_prefix(
                    self.storage.item_store().as_ref(),
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

                let mut read_dir = fs.read_dir(&dir_abs_path).await?;
                while let Some(entry) = read_dir.next_entry().await? {
                    if !entry.file_type().await?.is_dir() {
                        continue;
                    }

                    let id = entry.file_name().to_string_lossy().to_string();

                    let cache = match restored_entities.remove(&id).map_or(Ok(None), |v| {
                        v.deserialize::<CollectionCacheEntity>().map(Some)
                    }) {
                        Ok(value) => value,
                        Err(_err) => {
                            // TODO: logging
                            println!("failed to get the collection {:?} info", id);
                            continue;
                        }
                    };

                    let collection = {
                        let collection_abs_path: Arc<Path> = entry.path().to_owned().into();
                        let storage = Arc::new(StorageService::new(&collection_abs_path)?);
                        let worktree = WorktreeService::new(
                            collection_abs_path.clone(),
                            fs.clone(),
                            storage.clone(),
                        );
                        CollectionBuilder::new(fs.clone())
                            .with_service::<StorageService>(storage)
                            .with_service(worktree)
                            .load(LoadParams {
                                internal_abs_path: collection_abs_path,
                            })
                            .await?
                    };

                    collections.insert(
                        id.clone(),
                        Arc::new(RwLock::new(CollectionItem {
                            id: id.clone(),
                            order: cache.map(|v| v.order).flatten(),
                            inner: collection,
                        })),
                    );
                }

                Ok::<_, anyhow::Error>(collections)
            })
            .await?;

        Ok(result)
    }

    // Test only utility, not feature-flagged for easier CI setup
    pub fn __storage(&self) -> Arc<dyn WorkspaceStorage> {
        self.storage.clone()
    }
}
