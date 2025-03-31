pub mod api;

mod error;
pub use error::*;

use anyhow::{anyhow, Context as _, Result};
use moss_common::leased_slotmap::{LeasedSlotMap, ResourceKey};
use moss_fs::utils::{decode_directory_name, encode_directory_name};
use moss_fs::{FileSystem, RenameOptions};
use std::{collections::HashMap, ffi::OsString, path::PathBuf, sync::Arc};
use tokio::sync::{OnceCell, RwLock};

use crate::{
    models::{collection::RequestType, indexing::RequestEntry},
    storage::{state_db_manager::StateDbManagerImpl, StateDbManager},
};

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
    pub typ: RequestType,
}

fn request_file_name(name: &str, typ: &RequestType) -> String {
    format!("{}.{}.sapic", encode_directory_name(name), typ.to_string())
}

impl CollectionRequestData {
    fn request_file_path(&self) -> PathBuf {
        self.request_dir_relative_path
            .join(request_file_name(&self.name, &self.typ))
    }

    fn request_file_name(&self) -> String {
        request_file_name(&self.name, &self.typ)
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
            state_db_manager: Some(Arc::new(state_db_manager_impl)),
        })
    }

    pub fn state_db_manager(&self) -> Result<Arc<dyn StateDbManager>> {
        self.state_db_manager
            .clone()
            .ok_or(anyhow!("The state_db_manager has been dropped"))
    }

    async fn index_requests(&self, root: &PathBuf) -> Result<HashMap<PathBuf, RequestEntry>> {
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

    async fn index_request_dir(&self, path: &PathBuf) -> Result<RequestEntry> {
        let folder_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .context("Failed to read the request folder name")?;

        let mut request_entry = RequestEntry {
            name: get_request_name(folder_name)?,
            typ: None,
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
            request_entry.typ = Some(parse_output.file_type);
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
                let restored_requests = self.state_db_manager()?.request_store().scan()?;

                let mut requests = LeasedSlotMap::new();
                for (request_dir_relative_path, indexed_request_entry) in indexed_requests {
                    let entity = restored_requests.get(&request_dir_relative_path);

                    requests.insert(CollectionRequestData {
                        name: indexed_request_entry.name,
                        request_dir_relative_path,
                        order: entity.and_then(|e| e.order),
                        typ: indexed_request_entry.typ.unwrap(), // FIXME: get rid of Option type for typ
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
        .map(|ext| ext == REQUEST_FILE_EXT)
        .unwrap_or(false)
}

struct RequestFolderParseOutput {
    name: String,
    file_type: RequestType,
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
        file_type: RequestType::try_from(file_type_str)?,
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
