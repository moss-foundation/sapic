pub mod api;
mod error;

pub use error::*;

use anyhow::{anyhow, Context, Result};
use moss_common::leased_slotmap::{LeasedSlotMap, ResourceKey};
use moss_fs::{FileSystem, RenameOptions};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::{mpsc, OnceCell, RwLock};

use crate::constants::*;
use crate::indexer::{IndexJob, IndexerHandle};
use crate::models::types::{HttpMethod, RequestProtocol};
use crate::storage::{state_db_manager::StateDbManagerImpl, StateDbManager};

#[derive(Clone, Debug)]
pub struct CollectionCache {
    pub name: String,
    pub order: Option<usize>,
}

pub struct CollectionRequestData {
    pub name: String,
    // TODO: More tests on the path
    // FIXME: This field is a bit confusing, since it doesn't match with the input.relative_path
    /// A relative path, like CollectionName/requests
    pub entry_relative_path: PathBuf,
    pub order: Option<usize>,
    pub spec_file_name: String,
}

impl CollectionRequestData {
    pub fn protocol(&self) -> RequestProtocol {
        match self.spec_file_name.as_str() {
            GET_ENTRY_SPEC_FILE => RequestProtocol::Http(HttpMethod::Get),
            POST_ENTRY_SPEC_FILE => RequestProtocol::Http(HttpMethod::Post),
            PUT_ENTRY_SPEC_FILE => RequestProtocol::Http(HttpMethod::Put),
            DELETE_ENTRY_SPEC_FILE => RequestProtocol::Http(HttpMethod::Delete),
            GRAPHQL_ENTRY_SPEC_FILE => RequestProtocol::GraphQL,
            GRPC_ENTRY_SPEC_FILE => RequestProtocol::Grpc,
            _ => RequestProtocol::Http(HttpMethod::Get),
        }
    }
}

type RequestMap = LeasedSlotMap<ResourceKey, CollectionRequestData>;

pub struct Collection {
    fs: Arc<dyn FileSystem>,
    abs_path: PathBuf,
    // We have to use Option so that we can temporarily drop it
    // In the DbManager, we are storing relative paths
    state_db_manager: Option<Arc<dyn StateDbManager>>,
    requests: OnceCell<RwLock<RequestMap>>,
    indexer_handle: IndexerHandle,
}

#[derive(Debug)]
pub struct IndexedRequestDir {
    pub name: String,
    pub request_protocol: Option<RequestProtocol>,
    pub path: Option<PathBuf>,
}

impl Collection {
    pub fn new(
        path: PathBuf,
        fs: Arc<dyn FileSystem>,
        indexer_handle: IndexerHandle,
    ) -> Result<Self> {
        let state_db_manager_impl = StateDbManagerImpl::new(&path).context(format!(
            "Failed to open the collection {} state database",
            path.display()
        ))?;

        Ok(Self {
            fs: Arc::clone(&fs),
            abs_path: path,
            requests: OnceCell::new(),
            state_db_manager: Some(Arc::new(state_db_manager_impl)),
            indexer_handle,
        })
    }

    pub fn state_db_manager(&self) -> Result<Arc<dyn StateDbManager>> {
        self.state_db_manager
            .clone()
            .ok_or(anyhow!("The state_db_manager has been dropped"))
    }

    async fn requests(&self) -> Result<&RwLock<RequestMap>> {
        let result = self
            .requests
            .get_or_try_init(|| async move {
                let requests_dir_path = self.abs_path.join(REQUESTS_DIR);
                if !requests_dir_path.exists() {
                    return Ok(RwLock::new(LeasedSlotMap::new()));
                }

                let (result_tx, mut result_rx) = mpsc::unbounded_channel();
                self.indexer_handle.emit_job(IndexJob {
                    collection_key: ResourceKey::from(457895),
                    collection_abs_path: self.abs_path.clone(),
                    result_tx,
                })?;

                let mut requests = LeasedSlotMap::new();
                let restored_requests = self.state_db_manager()?.request_store().scan()?;

                while let Some(indexed_item) = result_rx.recv().await {
                    match indexed_item {
                        crate::indexer::IndexedEntry::Request(indexed_request_entry) => {
                            let request_dir_relative_path = indexed_request_entry
                                .folder_path
                                .strip_prefix(&self.abs_path)
                                .unwrap()
                                .to_path_buf();

                            let order = restored_requests
                                .get(&request_dir_relative_path)
                                .and_then(|e| e.order);

                            let spec_file_name = indexed_request_entry
                                .spec_file_path
                                .file_name()
                                .and_then(|name| name.to_str())
                                .unwrap_or(GET_ENTRY_SPEC_FILE)
                                .to_string();

                            requests.insert(CollectionRequestData {
                                name: indexed_request_entry.folder_name,
                                entry_relative_path: request_dir_relative_path,
                                order,
                                spec_file_name,
                            });
                        }
                        crate::indexer::IndexedEntry::RequestGroup(
                            _indexed_request_group_entry,
                        ) => {}
                    }
                }

                Ok::<_, anyhow::Error>(RwLock::new(requests))
            })
            .await?;

        Ok(result)
    }

    pub fn path(&self) -> &PathBuf {
        &self.abs_path
    }

    // Temporarily drop the db for fs renaming, and reloading it from the new path
    pub async fn reset(&mut self, new_path: PathBuf) -> Result<()> {
        let _ = self.state_db_manager.take();

        let old_path = std::mem::replace(&mut self.abs_path, new_path.clone());
        self.fs
            .rename(&old_path, &new_path, RenameOptions::default())
            .await?;

        let state_db_manager_impl = StateDbManagerImpl::new(new_path).context(format!(
            "Failed to open the collection {} state database",
            self.abs_path.display()
        ))?;
        self.state_db_manager = Some(Arc::new(state_db_manager_impl));

        Ok(())
    }
}
