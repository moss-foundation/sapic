pub mod api;
mod error;

mod primitives;
mod utils;
mod visit;

pub use error::*;

use anyhow::{anyhow, Context as _, Result};
use moss_common::leased_slotmap::{LeasedSlotMap, ResourceKey};
use moss_fs::utils::decode_directory_name;
use moss_fs::{FileSystem, RenameOptions};
use primitives::CollectionEntryFilename;
use std::{collections::HashMap, ffi::OsString, path::PathBuf, sync::Arc};
use tokio::sync::{mpsc, OnceCell, RwLock};

use crate::indexer::IndexJob;
use crate::models::types::{HttpMethod, RequestProtocol};
use crate::storage::{state_db_manager::StateDbManagerImpl, StateDbManager};

const REQUESTS_DIR: &'static str = "requests";
const REQUEST_DIR_EXT: &'static str = "request";
const SAPIC_FILE_EXT: &'static str = "sapic";

#[derive(Clone, Debug)]
pub struct CollectionCache {
    pub name: String,
    pub order: Option<usize>,
}

pub struct CollectionRequestData {
    pub name: String,
    // TODO: More tests on the path
    // FIXME: This field is a bit confusing, since it doesn't match with the input.relative_path
    pub request_dir_relative_path: PathBuf, // Relative path from collection/requests
    pub order: Option<usize>,
    // FIXME: Should we create separate backend/frontend types for RequestType?
    pub protocol: RequestProtocol,
}

impl CollectionRequestData {
    fn request_file_path(&self) -> PathBuf {
        let file_ext = CollectionEntryFilename::from(&self.protocol);
        let file_name = utils::request_file_name(&self.name, &file_ext);

        self.request_dir_relative_path.join(file_name)
    }

    fn request_file_name(&self) -> String {
        let file_ext = CollectionEntryFilename::from(&self.protocol);

        utils::request_file_name(&self.name, &file_ext)
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
    tx: mpsc::UnboundedSender<IndexJob>,
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
        tx: mpsc::UnboundedSender<IndexJob>,
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
            tx,
        })
    }

    pub fn state_db_manager(&self) -> Result<Arc<dyn StateDbManager>> {
        self.state_db_manager
            .clone()
            .ok_or(anyhow!("The state_db_manager has been dropped"))
    }

    // async fn index_requests(&self, root: &PathBuf) -> Result<HashMap<PathBuf, IndexedRequestDir>> {
    //     let mut result = HashMap::new();
    //     let mut stack: Vec<PathBuf> = vec![root.clone()];

    //     while let Some(current_dir) = stack.pop() {
    //         let mut dir = self.fs.read_dir(&current_dir).await.context(format!(
    //             "Failed to read the directory: {}",
    //             current_dir.display()
    //         ))?;

    //         while let Some(entry) = dir.next_entry().await? {
    //             let file_type = entry.file_type().await?;
    //             if !file_type.is_dir() {
    //                 continue;
    //             }

    //             let path = entry.path();

    //             if path
    //                 .extension()
    //                 .map(|ext| ext == REQUEST_DIR_EXT)
    //                 .unwrap_or(false)
    //             {
    //                 if let Ok(relative_path) = path.strip_prefix(&root) {
    //                     let request_entry = self.index_request_dir(&path).await?;
    //                     result.insert(relative_path.to_path_buf(), request_entry);
    //                 } else {
    //                     // TODO: log error
    //                     continue;
    //                 }
    //             } else {
    //                 stack.push(path);
    //             }
    //         }
    //     }

    //     Ok(result)
    // }

    // async fn index_request_dir(&self, path: &PathBuf) -> Result<IndexedRequestDir> {
    //     let folder_name = path
    //         .file_name()
    //         .and_then(|s| s.to_str())
    //         .context("Failed to read the request folder name")?;

    //     let mut request_entry = IndexedRequestDir {
    //         name: get_request_name(folder_name)?,
    //         request_protocol: None,
    //         path: None,
    //     };

    //     let mut inner_dir = self.fs.read_dir(&path).await?;

    //     while let Some(inner_entry) = inner_dir.next_entry().await? {
    //         let file_path = inner_entry.path();
    //         let file_metadata = inner_entry.metadata().await?;

    //         if !file_metadata.is_file() || !is_sapic_file(&file_path) {
    //             continue;
    //         }

    //         let file_name = if let Some(name) = file_path.file_name() {
    //             name.to_owned()
    //         } else {
    //             // TODO: logging?
    //             println!("Failed to read the request file name");
    //             continue;
    //         };

    //         let parse_output = parse_request_folder_name(file_name)?;

    //         request_entry.path = Some(file_path);
    //         request_entry.request_protocol = Some(parse_output.file_ext.into());
    //     }

    //     Ok(request_entry)
    // }

    async fn requests(&self) -> Result<&RwLock<RequestMap>> {
        let result = self
            .requests
            .get_or_try_init(|| async move {
                let requests_dir_path = self.abs_path.join(REQUESTS_DIR);
                if !requests_dir_path.exists() {
                    return Ok(RwLock::new(LeasedSlotMap::new()));
                }

                let (result_tx, mut result_rx) = mpsc::unbounded_channel();
                self.tx.send(IndexJob {
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

                            let protocol = indexed_request_entry
                                .spec_file_path
                                .file_name()
                                .and_then(|name| name.to_str())
                                .and_then(|name| CollectionEntryFilename::try_from(name).ok());

                            let protocol = if let Some(protocol) = protocol {
                                RequestProtocol::from(protocol)
                            } else {
                                RequestProtocol::Http(HttpMethod::Get)
                            };

                            requests.insert(CollectionRequestData {
                                name: indexed_request_entry.folder_name,
                                request_dir_relative_path,
                                order,
                                protocol,
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

fn is_sapic_file(file_path: &PathBuf) -> bool {
    file_path
        .extension()
        .map(|ext| ext == SAPIC_FILE_EXT)
        .unwrap_or(false)
}

struct RequestFolderParseOutput {
    name: String,
    file_ext: CollectionEntryFilename,
}

fn parse_request_folder_name(file_name: OsString) -> Result<RequestFolderParseOutput> {
    let file_name_str = file_name
        .to_str()
        .ok_or_else(|| anyhow!("failed to retrieve the request filename"))?;

    let mut segments = file_name_str.split('.');

    let name = decode_directory_name(
        segments
            .next()
            .ok_or_else(|| anyhow!("failed to retrieve the request name"))?,
    )?;

    let file_type_str = segments
        .next()
        .ok_or_else(|| anyhow!("failed to retrieve the request type"))?;

    Ok(RequestFolderParseOutput {
        name,
        file_ext: CollectionEntryFilename::try_from(file_type_str)?,
    })
}

fn get_request_name(folder_name: &str) -> Result<String> {
    if let Some(prefix) = folder_name.strip_suffix(".request") {
        Ok(decode_directory_name(prefix)?)
    } else {
        Err(anyhow!(
            "failed to extract the request name from the directory name"
        ))
    }
}
