pub mod api;

use anyhow::{Context, Result};
use moss_activity_indicator::ActivityIndicator;
use moss_collection::collection::Collection;
use moss_common::{leased_slotmap::LeasedSlotMap, models::primitives::Identifier};
use moss_environment::environment::Environment;
use moss_fs::{FileSystem, utils::decode_name};
use moss_storage::{WorkspaceStorage, workspace_storage::WorkspaceStorageImpl};
use std::{
    collections::HashMap,
    ops::Deref,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicUsize},
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::{OnceCell, RwLock, mpsc};

pub const COLLECTIONS_DIR: &str = "collections";
pub const ENVIRONMENTS_DIR: &str = "environments";

// type EnvironmentSlot = (Environment, EnvironmentCache);
// type EnvironmentMap = LeasedSlotMap<ResourceKey, EnvironmentSlot>;

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
    app_handle: AppHandle<R>,
    abs_path: Arc<Path>,
    fs: Arc<dyn FileSystem>,
    workspace_storage: Arc<dyn WorkspaceStorage>,
    collections: OnceCell<RwLock<CollectionMap>>,
    environments: OnceCell<RwLock<EnvironmentMap>>,
    #[allow(dead_code)]
    activity_indicator: ActivityIndicator<R>,
    next_collection_entry_id: Arc<AtomicUsize>,
    next_collection_id: Arc<AtomicUsize>,
    next_variable_id: Arc<AtomicUsize>,
    next_environment_id: Arc<AtomicUsize>,
}

impl<R: TauriRuntime> Workspace<R> {
    pub fn new(
        app_handle: AppHandle<R>,
        path: Arc<Path>,
        fs: Arc<dyn FileSystem>,
        activity_indicator: ActivityIndicator<R>,
    ) -> Result<Self> {
        let state_db_manager = WorkspaceStorageImpl::new(&path)
            .context("Failed to open the workspace state database")?;

        Ok(Self {
            app_handle,
            abs_path: path,
            fs,
            workspace_storage: Arc::new(state_db_manager),
            collections: OnceCell::new(),
            environments: OnceCell::new(),
            activity_indicator,
            next_collection_entry_id: Arc::new(AtomicUsize::new(0)),
            next_collection_id: Arc::new(AtomicUsize::new(0)),
            next_variable_id: Arc::new(AtomicUsize::new(0)),
            next_environment_id: Arc::new(AtomicUsize::new(0)),
        })
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

                for entry in self.fs.read_dir(&abs_path).await?.next_entry().await? {
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
                for (path, collection_data) in self
                    .workspace_storage
                    .collection_store()
                    .list_collection()?
                {
                    debug_assert!(path.is_relative());

                    let (display_name, encoded_name) = match path.file_name() {
                        Some(name) => {
                            let name = name.to_string_lossy().to_string();

                            (decode_name(&name)?, name)
                        }
                        None => {
                            // TODO: logging
                            println!("failed to get the collection {:?} name", path);
                            continue;
                        }
                    };

                    // TODO: A self-healing mechanism needs to be implemented here.
                    // Collections that are found in the database but do not actually exist
                    // in the file system should be collected and deleted from the database in
                    // a parallel thread.

                    let id = Identifier::new(&self.next_collection_id);
                    let abs_path: Arc<Path> = self.abs_path.join(path).into();
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
                            order: collection_data.order,
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
