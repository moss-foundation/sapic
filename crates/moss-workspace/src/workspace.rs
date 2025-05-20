use anyhow::{Context, Result};
use moss_activity_indicator::ActivityIndicator;
use moss_collection::collection::Collection;
use moss_common::models::primitives::Identifier;
use moss_environment::environment::Environment;
use moss_fs::{CreateOptions, FileSystem, utils::decode_name};
use moss_storage::{
    WorkspaceStorage,
    primitives::segkey::SegmentExt,
    storage::operations::ListByPrefix,
    workspace_storage::{
        WorkspaceStorageImpl, entities::collection_store_entities::CollectionEntity,
    },
};
use std::{
    collections::HashMap,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::{OnceCell, RwLock};

use crate::{
    manifest::{MANIFEST_FILE_NAME, Manifest},
    storage::segments::COLLECTION_SEGKEY,
};

pub const COLLECTIONS_DIR: &str = "collections";
pub const ENVIRONMENTS_DIR: &str = "environments";

pub struct CollectionEntry {
    pub id: Identifier,
    pub name: String,
    pub display_name: String,
    pub order: Option<usize>,
    pub inner: Collection,
}

impl Deref for CollectionEntry {
    type Target = Collection;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct EnvironmentEntry {
    pub id: Identifier,
    pub name: String,
    pub display_name: String,
    pub order: Option<usize>,
    pub inner: Environment,
}

impl Deref for EnvironmentEntry {
    type Target = Environment;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

type CollectionMap = HashMap<Identifier, Arc<CollectionEntry>>;
type EnvironmentMap = HashMap<Identifier, Arc<EnvironmentEntry>>;

pub struct Workspace<R: TauriRuntime> {
    #[allow(dead_code)]
    pub(super) app_handle: AppHandle<R>,
    pub(super) abs_path: Arc<Path>,
    pub(super) fs: Arc<dyn FileSystem>,
    pub(super) workspace_storage: Arc<dyn WorkspaceStorage>,
    pub(super) collections: OnceCell<RwLock<CollectionMap>>,
    pub(super) environments: OnceCell<RwLock<EnvironmentMap>>,
    #[allow(dead_code)]
    pub(super) activity_indicator: ActivityIndicator<R>,
    pub(super) next_collection_entry_id: Arc<AtomicUsize>,
    pub(super) next_collection_id: Arc<AtomicUsize>,
    pub(super) next_variable_id: Arc<AtomicUsize>,
    pub(super) next_environment_id: Arc<AtomicUsize>,

    #[allow(dead_code)]
    pub(super) manifest: Manifest,
}

impl<R: TauriRuntime> Workspace<R> {
    pub async fn open(
        app_handle: AppHandle<R>,
        abs_path: Arc<Path>,
        fs: Arc<dyn FileSystem>,
        activity_indicator: ActivityIndicator<R>,
    ) -> Result<Self> {
        let state_db_manager = WorkspaceStorageImpl::new(&abs_path)
            .context("Failed to open the workspace state database")?;

        let manifest = read_manifest(&fs, &abs_path).await?;

        Ok(Self {
            app_handle,
            abs_path,
            fs,
            workspace_storage: Arc::new(state_db_manager),
            collections: OnceCell::new(),
            environments: OnceCell::new(),
            activity_indicator,
            next_collection_entry_id: Arc::new(AtomicUsize::new(0)),
            next_collection_id: Arc::new(AtomicUsize::new(0)),
            next_variable_id: Arc::new(AtomicUsize::new(0)),
            next_environment_id: Arc::new(AtomicUsize::new(0)),
            manifest,
        })
    }

    pub async fn create(
        name: String,
        app_handle: AppHandle<R>,
        abs_path: Arc<Path>,
        fs: Arc<dyn FileSystem>,
        activity_indicator: ActivityIndicator<R>,
    ) -> Result<Self> {
        let state_db_manager = WorkspaceStorageImpl::new(&abs_path)
            .context("Failed to open the workspace state database")?;

        let manifest_path = abs_path.join(MANIFEST_FILE_NAME);
        let manifest = Manifest::new(name);
        let content = toml::to_string_pretty(&manifest)?;
        fs.create_file_with(
            &manifest_path,
            content.as_bytes(),
            CreateOptions {
                overwrite: true,
                ignore_if_exists: false,
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
            next_collection_id: Arc::new(AtomicUsize::new(0)),
            next_variable_id: Arc::new(AtomicUsize::new(0)),
            next_environment_id: Arc::new(AtomicUsize::new(0)),
            manifest,
        })
    }

    pub async fn open_manifest(fs: &Arc<dyn FileSystem>, abs_path: &Arc<Path>) -> Result<Manifest> {
        Ok(read_manifest(fs, abs_path).await?)
    }

    pub fn abs_path(&self) -> &Arc<Path> {
        &self.abs_path
    }

    pub(super) fn absolutize<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.abs_path.join(path)
    }

    async fn environments(&self) -> Result<&RwLock<EnvironmentMap>> {
        let result = self
            .environments
            .get_or_try_init(|| async move {
                let mut environments = HashMap::new();

                let abs_path = self.abs_path.join(ENVIRONMENTS_DIR);
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
                    let decoded_name = decode_name(&name)?;

                    let environment = Environment::new(
                        entry_abs_path,
                        self.fs.clone(),
                        self.workspace_storage.variable_store().clone(),
                        self.next_variable_id.clone(),
                    )
                    .await?;

                    let id = Identifier::new(&self.next_environment_id);
                    let entry = EnvironmentEntry {
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
                let mut collections = HashMap::new();

                if !self.abs_path.join(COLLECTIONS_DIR).exists() {
                    return Ok(RwLock::new(collections));
                }

                // TODO: Support external collections with absolute path

                let collection_items = ListByPrefix::list_by_prefix(
                    self.workspace_storage.item_store().as_ref(),
                    COLLECTIONS_DIR,
                )?
                .into_iter()
                .filter_map(|(k, v)| {
                    let path = k.after(&COLLECTION_SEGKEY);
                    if let Some(path) = path {
                        Some((path, v))
                    } else {
                        None
                    }
                });
                for (segkey, any_value) in collection_items.into_iter() {
                    let value: CollectionEntity = any_value.deserialize()?;
                    let encoded_name = match String::from_utf8(segkey.as_bytes().to_owned()) {
                        Ok(name) => name,
                        Err(_) => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", segkey);
                            continue;
                        }
                    };

                    // TODO: A self-healing mechanism needs to be implemented here.
                    // Collections that are found in the database but do not actually exist
                    // in the file system should be collected and deleted from the database in
                    // a parallel thread.

                    let abs_path: Arc<Path> =
                        if let Some(external_abs_path) = value.external_abs_path {
                            external_abs_path.into()
                        } else {
                            self.abs_path
                                .join(COLLECTIONS_DIR)
                                .join(encoded_name)
                                .into()
                        };
                    if !abs_path.exists() {
                        // TODO: logging
                        continue;
                    }

                    let (display_name, encoded_name) = match abs_path.file_name() {
                        Some(name) => {
                            let name = name.to_string_lossy().to_string();

                            (decode_name(&name)?, name)
                        }
                        None => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", segkey);
                            continue;
                        }
                    };

                    let id = Identifier::new(&self.next_collection_id);
                    let collection = Collection::new(
                        abs_path.to_path_buf(), // FIXME: change to Arc<Path> in Collection::new
                        self.fs.clone(),
                        self.next_collection_entry_id.clone(),
                    )?;
                    collections.insert(
                        id,
                        CollectionEntry {
                            id,
                            name: encoded_name,
                            display_name,
                            order: value.order,
                            inner: collection,
                        }
                        .into(),
                    );
                }

                Ok::<_, anyhow::Error>(RwLock::new(collections))
            })
            .await?;

        Ok(result)
    }
}

impl<R: TauriRuntime> Workspace<R> {
    #[cfg(test)]
    pub fn truncate(&self) -> Result<()> {
        // let collection_store = self.workspace_storage.collection_store();

        // let (mut txn, table) = collection_store.begin_write()?;
        // table.truncate(&mut txn)?;
        // Ok(txn.commit()?)
        todo!()
    }
}

async fn read_manifest(fs: &Arc<dyn FileSystem>, abs_path: &Arc<Path>) -> Result<Manifest> {
    let manifest_path = abs_path.join(MANIFEST_FILE_NAME);
    let mut reader = fs
        .open_file(&manifest_path)
        .await
        .context("Failed to open existing workspace manifest file")?;

    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .context("Failed to read workspace manifest file")?;

    Ok(toml::from_str(&buf)?)
}
