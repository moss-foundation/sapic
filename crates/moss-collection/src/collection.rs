pub mod api;
mod error;
mod primitives;
mod utils;

pub use error::*;

use anyhow::{anyhow, Context as _, Result};
use moss_common::leased_slotmap::{LeasedSlotMap, ResourceKey};
use moss_fs::utils::decode_directory_name;
use moss_fs::{FileSystem, RenameOptions};
use primitives::EndpointFileExt;
use std::{collections::HashMap, ffi::OsString, path::PathBuf, sync::Arc};
use std::path::Path;
use tokio::sync::{OnceCell, RwLock};

use crate::models::types::RequestProtocol;
use crate::storage::{state_db_manager::StateDbManagerImpl, StateDbManager};

const REQUESTS_DIR: &'static str = "requests";
const REQUEST_DIR_EXT: &'static str = "request";
const REQUEST_FILE_EXT: &'static str = "sapic";

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
        let file_ext = EndpointFileExt::from(&self.protocol);
        let file_name = utils::request_file_name(&self.name, &file_ext);

        self.request_dir_relative_path.join(file_name)
    }

    fn request_file_name(&self) -> String {
        let file_ext = EndpointFileExt::from(&self.protocol);

        utils::request_file_name(&self.name, &file_ext)
    }
}

type RequestMap = LeasedSlotMap<ResourceKey, CollectionRequestData>;

pub struct Collection {
    fs: Arc<dyn FileSystem>,
    abs_path: PathBuf,
    // We have to use Option so that we can temporarily drop it
    // In the DbManager, we are storing relative paths
    state_db_manager: Arc<dyn StateDbManager>,
    requests: OnceCell<RwLock<RequestMap>>,
}

#[derive(Debug)]
pub struct IndexedEndpointDir {
    pub name: String,
    pub request_protocol: Option<RequestProtocol>,
    pub path: Option<PathBuf>,
}

impl Collection {
    pub fn new(path: PathBuf, fs: Arc<dyn FileSystem>) -> Result<Self> {
        let state_db_manager_impl = StateDbManagerImpl::new(&path).context(format!(
            "Failed to open the collection {} state database",
            path.display()
        ))?;

        Ok(Self {
            fs: Arc::clone(&fs),
            abs_path: path,
            requests: OnceCell::new(),
            state_db_manager: Arc::new(state_db_manager_impl),
        })
    }

    pub fn state_db_manager(&self) -> Arc<dyn StateDbManager> {
        self.state_db_manager.clone()
    }

    async fn index_requests(&self, root: &PathBuf) -> Result<HashMap<PathBuf, IndexedEndpointDir>> {
        let mut result = HashMap::new();
        let mut stack: Vec<PathBuf> = vec![root.clone()];

        while let Some(current_dir) = stack.pop() {
            let mut dir = self.fs.read_dir(&current_dir).await.context(format!(
                "Failed to read the directory: {}",
                current_dir.display()
            ))?;

            while let Some(entry) = dir.next_entry().await? {
                let file_type = entry.file_type().await?;
                if !file_type.is_dir() {
                    continue;
                }

                let path = entry.path();

                if path
                    .extension()
                    .map(|ext| ext == REQUEST_DIR_EXT)
                    .unwrap_or(false)
                {
                    if let Ok(relative_path) = path.strip_prefix(&root) {
                        let request_entry = self.index_request_dir(&path).await?;
                        result.insert(relative_path.to_path_buf(), request_entry);
                    } else {
                        // TODO: log error
                        continue;
                    }
                } else {
                    stack.push(path);
                }
            }
        }

        Ok(result)
    }

    async fn index_request_dir(&self, path: &PathBuf) -> Result<IndexedEndpointDir> {
        let folder_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .context("Failed to read the request folder name")?;

        let mut request_entry = IndexedEndpointDir {
            name: get_request_name(folder_name)?,
            request_protocol: None,
            path: None,
        };

        let mut inner_dir = self.fs.read_dir(&path).await?;

        while let Some(inner_entry) = inner_dir.next_entry().await? {
            let file_path = inner_entry.path();
            let file_metadata = inner_entry.metadata().await?;

            if !file_metadata.is_file() || !is_sapic_file(&file_path) {
                continue;
            }

            let file_name = if let Some(name) = file_path.file_name() {
                name.to_owned()
            } else {
                // TODO: logging?
                println!("Failed to read the request file name");
                continue;
            };

            let parse_output = parse_request_folder_name(file_name)?;

            request_entry.path = Some(file_path);
            request_entry.request_protocol = Some(parse_output.file_ext.into());
        }

        Ok(request_entry)
    }

    async fn requests(&self) -> Result<&RwLock<RequestMap>> {
        let result = self
            .requests
            .get_or_try_init(|| async move {
                let requests_dir_path = self.abs_path.join(REQUESTS_DIR);
                if !requests_dir_path.exists() {
                    return Ok(RwLock::new(LeasedSlotMap::new()));
                }

                let indexed_requests = self.index_requests(&requests_dir_path).await?;
                let restored_requests = self.state_db_manager().request_store().scan()?;

                let mut requests = LeasedSlotMap::new();
                for (request_dir_relative_path, indexed_request_entry) in indexed_requests {
                    let entity = restored_requests.get(&request_dir_relative_path);

                    requests.insert(CollectionRequestData {
                        name: indexed_request_entry.name,
                        request_dir_relative_path,
                        order: entity.and_then(|e| e.order),
                        protocol: indexed_request_entry.request_protocol.unwrap(), // FIXME: get rid of Option type for typ
                    });
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
    pub async fn reset(&mut self, new_path: &Path) -> Result<()> {
        if let Some(manager) = Arc::get_mut(&mut self.state_db_manager) {
            self.abs_path = new_path.to_path_buf();
            manager.reset(self.fs.clone(), new_path).await
        } else {
            Err(anyhow!("The Collection StateDbManager is being used"))
        }
    }
}

fn is_sapic_file(file_path: &PathBuf) -> bool {
    file_path
        .extension()
        .map(|ext| ext == REQUEST_FILE_EXT)
        .unwrap_or(false)
}

struct RequestFolderParseOutput {
    name: String,
    file_ext: EndpointFileExt,
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
        file_ext: EndpointFileExt::try_from(file_type_str)?,
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
