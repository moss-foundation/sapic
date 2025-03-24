use anyhow::{anyhow, Context as _, Result};
use dashmap::DashSet;
use moss_fs::{CreateOptions, FileSystem, RemoveOptions, RenameOptions};
use slotmap::KeyData;
use std::{collections::HashMap, ffi::OsString, path::PathBuf, sync::Arc};
use thiserror::Error;
use tokio::sync::{OnceCell, RwLock};
use validator::{Validate, ValidationErrors};
use crate::models::operations::collection_operations::DeleteRequestInput;
use crate::{
    kdl::http::HttpRequestFile,
    leased_slotmap::LeasedSlotMap,
    models::{
        collection::{HttpRequestType, RequestType},
        indexing::RequestEntry,
        operations::collection_operations::{
            CreateRequestInput, CreateRequestOutput, CreateRequestProtocolSpecificPayload,
            RenameRequestInput,
        },
        storage::RequestEntity,
        types::request_types::HttpMethod,
    },
    storage::{state_db_manager::StateDbManagerImpl, StateDbManager},
};
use crate::sanitizer::{decode_directory_name, encode_directory_name};

const REQUESTS_DIR: &'static str = "requests";
const REQUEST_DIR_EXT: &'static str = "request";
const REQUEST_FILE_EXT: &'static str = "sapic";

slotmap::new_key_type! {
    pub struct RequestKey;
}

impl From<u64> for RequestKey {
    fn from(value: u64) -> Self {
        Self(KeyData::from_ffi(value))
    }
}

impl RequestKey {
    pub fn as_u64(self) -> u64 {
        self.0.as_ffi()
    }
}

impl std::fmt::Display for RequestKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u64())
    }
}

#[derive(Error, Debug)]
pub enum OperationError {
    #[error("validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("request {name} not found at {path}")]
    NotFound { name: String, path: PathBuf },

    #[error("request {name} already exists at {path}")]
    AlreadyExists { name: String, path: PathBuf },

    #[error("unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

#[derive(Clone, Debug)]
pub struct CollectionMetadata {
    pub name: String,
    pub order: Option<usize>,
}

pub struct CollectionRequestData {
    pub name: String,
    // TODO: More tests on the path
    pub request_dir_relative_path: PathBuf, // Relative path from collection/requests
    pub order: Option<usize>,
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

type RequestMap = LeasedSlotMap<RequestKey, CollectionRequestData>;



pub struct Collection {
    fs: Arc<dyn FileSystem>,
    // FIXME: Should we store the full path instead of relative path here?
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

    pub async fn create_request(&self, input: CreateRequestInput) -> Result<CreateRequestOutput, OperationError> {
        input.validate()?;

        let request_dir_name = format!("{}.request", encode_directory_name(&input.name));

        let request_dir_relative_path = input
            .relative_path
            .unwrap_or_default()
            .join(&request_dir_name);

        let request_dir_full_path = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&request_dir_relative_path);

        if request_dir_full_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.name,
                path: request_dir_full_path,
            });
        }

        let (request_file_content, request_file_extension) = match input.payload {
            Some(CreateRequestProtocolSpecificPayload::Http {
                method,
                query_params,
                path_params,
                headers,
                body,
            }) => {
                let request_file = HttpRequestFile::new(
                    input.url.as_deref(),
                    query_params,
                    path_params,
                    headers,
                    body,
                )
                .to_string();

                // FIXME:
                let request_file_extension = match method {
                    HttpMethod::Post => "post".to_string(),
                    HttpMethod::Get => "get".to_string(),
                    HttpMethod::Put => "put".to_string(),
                    HttpMethod::Delete => "del".to_string(),
                };

                (request_file.to_string(), request_file_extension)
            }

            // FIXME:
            None => ("".to_string(), "get".to_string()),
        };

        let request_store = self.state_db_manager()?.request_store();
        let requests = self.requests().await?;

        let (mut txn, table) = request_store.begin_write()?;
        table.insert(
            &mut txn,
            request_dir_relative_path.to_string_lossy().to_string(),
            &RequestEntity { order: None },
        )?;

        // For consistency we are encoding both the directory and the request file
        let request_file_name = format!("{}.{}.sapic", encode_directory_name(&input.name), request_file_extension);
        self.fs
            .create_dir(&request_dir_full_path)
            .await
            .context("Failed to create the collection directory")?;
        self.fs
            .create_file_with(
                &request_dir_full_path.join(request_file_name),
                request_file_content,
                CreateOptions::default(),
            )
            .await
            .context("Failed to create the request file")?;

        txn.commit()?;

        let request_key = {
            let mut requests_lock = requests.write().await;
            requests_lock.insert(CollectionRequestData {
                name: input.name,
                request_dir_relative_path: request_dir_relative_path.clone(),
                order: None,
                typ: RequestType::Http(HttpRequestType::Get), // FIXME:
            })
        };

        Ok(CreateRequestOutput {
            key: request_key.as_u64(),
        })
    }

    pub async fn rename_request(&self, input: RenameRequestInput) -> Result<(), OperationError> {
        input.validate()?;

        let requests = self.requests().await?;
        let mut requests_lock = requests.write().await;

        let request_key = RequestKey::from(input.key);
        let mut lease_request_data = requests_lock.lease(request_key)?;

        let request_dir_relative_path_old = lease_request_data.request_dir_relative_path.to_owned();
        let request_dir_path_old = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&request_dir_relative_path_old);
        if !request_dir_path_old.exists() {
            return Err(OperationError::NotFound {
                name: lease_request_data.name.clone(),
                path: request_dir_path_old
            })
        }

        let request_dir_relative_path_new = lease_request_data
            .request_dir_relative_path
            .parent()
            .context("Failed to get the parent directory")?
            .join(format!("{}.request", encode_directory_name(&input.new_name)));

        // Rename the request directory
        let request_dir_path_new = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&request_dir_relative_path_new);
        if request_dir_path_new.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.new_name,
                path: request_dir_path_new
            })
        }

        self.fs
            .rename(
                &request_dir_path_old,
                &request_dir_path_new,
                RenameOptions::default(),
            )
            .await
            .context("Failed to rename the request directory")?;

        // Rename the request file
        let request_file_path_old =
            request_dir_path_new.join(&lease_request_data.request_file_name());
        let request_file_name_new = request_file_name(&input.new_name, &lease_request_data.typ);
        let request_file_path_new = request_dir_path_new.join(&request_file_name_new);
        self.fs
            .rename(
                &request_file_path_old,
                &request_file_path_new,
                RenameOptions::default(),
            )
            .await
            .context("Failed to rename the request file")?;

        let request_store = self.state_db_manager()?.request_store();
        let (mut txn, table) = request_store.begin_write()?;
        table.remove(
            &mut txn,
            request_dir_relative_path_old.to_string_lossy().to_string(),
        )?;

        table.insert(
            &mut txn,
            request_dir_relative_path_new.to_string_lossy().to_string(),
            &RequestEntity {
                order: lease_request_data.order,
            },
        )?;

        lease_request_data.name = input.new_name;
        lease_request_data.request_dir_relative_path = request_dir_relative_path_new.clone();
        Ok(txn.commit()?)
    }

    pub async fn delete_request(&self, input: DeleteRequestInput) -> Result<()> {
        let requests = self.requests().await?;
        let mut requests_lock = requests.write().await;

        let request_key = RequestKey::from(input.key);
        let mut request_data = requests_lock.remove(request_key)?;
        std::mem::drop(requests_lock);

        let request_dir_relative_path = request_data.request_dir_relative_path.clone();
        let request_dir_path = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&request_dir_relative_path);

        // TODO: Add logging when the request was already deleted from the fs?
        // TODO: Self-healing process
        self.fs
            .remove_dir(
                &request_dir_path,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await
            .context("Failed to remove the request directory")?;

        let request_store = self.state_db_manager()?.request_store();
        let (mut txn, table) = request_store.begin_write()?;
        table.remove(
            &mut txn,
            request_dir_relative_path.to_string_lossy().to_string(),
        )?;

        txn.commit()?;

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

    let name = decode_directory_name(segments
        .next()
        .ok_or_else(|| anyhow!("failed to retrieve the request name"))?
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

#[cfg(test)]
mod tests {
    use moss_fs::RealFileSystem;

    use super::*;

    fn random_string(length: usize) -> String {
        use rand::{distr::Alphanumeric, Rng};

        rand::rng()
            .sample_iter(Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    pub fn random_request_name() -> String {
        format!("Test_{}_Request", random_string(10))
    }

    pub fn random_collection_name() -> String {
        format!("Test_{}_Collection", random_string(10))
    }

    #[tokio::test]
    async fn create_request() {
        let workspace_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
        let collection_path = workspace_path.join(random_collection_name());
        tokio::fs::create_dir_all(&collection_path).await.unwrap();

        let collection =
            Collection::new(collection_path.clone(), Arc::new(RealFileSystem::new())).unwrap();

        let request_name = random_request_name();
        let create_request_result = collection
            .create_request(CreateRequestInput {
                name: request_name.clone(),
                relative_path: None,
                payload: None,
                url: None,
            })
            .await;

        assert!(create_request_result.is_ok());

        let create_request_output = create_request_result.unwrap();

        let requests = collection.requests().await.unwrap();
        let requests_lock = requests.read().await;
        let request_key = RequestKey::from(create_request_output.key);
        let request = requests_lock.read(request_key).unwrap();

        assert_eq!(request.name, request_name);

        // Clean up
        {
            tokio::fs::remove_dir_all(collection_path).await.unwrap();
        }
    }

    #[tokio::test]
    async fn create_request_invalid_name() {
        let workspace_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
        let collection_path = workspace_path.join(random_collection_name());
        tokio::fs::create_dir_all(&collection_path).await.unwrap();

        let collection =
            Collection::new(collection_path.clone(), Arc::new(DiskFileSystem::new())).unwrap();

        let request_name = "".to_string();
        let create_request_result = collection
            .create_request(CreateRequestInput {
                name: request_name.clone(),
                relative_path: None,
                payload: None,
                url: None,
            })
            .await;

        assert!(create_request_result.is_err());
    }

    #[tokio::test]
    async fn create_request_forbidden_characters() {
        let workspace_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
        let collection_path = workspace_path.join(random_collection_name());
        tokio::fs::create_dir_all(&collection_path).await.unwrap();

        let collection =
            Collection::new(collection_path.clone(), Arc::new(DiskFileSystem::new())).unwrap();

        let request_name = format!("{}/name",random_request_name());
        let create_request_result = collection
            .create_request(CreateRequestInput {
                name: request_name.clone(),
                relative_path: None,
                payload: None,
                url: None,
            })
            .await;

        assert!(create_request_result.is_ok());

        let create_request_output = create_request_result.unwrap();

        let requests = collection.requests().await.unwrap();
        let requests_lock = requests.read().await;
        let request_key = RequestKey::from(create_request_output.key);
        let request = requests_lock.read(request_key).unwrap();

        assert_eq!(request.name, request_name);

        // Clean up
        {
            tokio::fs::remove_dir_all(collection_path).await.unwrap();
        }
    }

    #[tokio::test]
    async fn create_request_already_exists() {
        let workspace_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
        let collection_path = workspace_path.join(random_collection_name());
        tokio::fs::create_dir_all(&collection_path).await.unwrap();

        let collection =
            Collection::new(collection_path.clone(), Arc::new(DiskFileSystem::new())).unwrap();

        let request_name = random_request_name();
        collection
            .create_request(CreateRequestInput {
                name: request_name.clone(),
                relative_path: None,
                payload: None,
                url: None,
            })
            .await.unwrap();

        let create_request_result = collection
            .create_request(CreateRequestInput {
                name: request_name.clone(),
                relative_path: None,
                payload: None,
                url: None,
            })
            .await;

        assert!(create_request_result.is_err());

        // Clean up
        {
            tokio::fs::remove_dir_all(collection_path).await.unwrap();
        }
    }

    #[tokio::test]
    async fn rename_request() {
        let workspace_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
        let collection_path = workspace_path.join(random_collection_name());
        tokio::fs::create_dir_all(&collection_path).await.unwrap();

        let collection =
            Collection::new(collection_path.clone(), Arc::new(RealFileSystem::new())).unwrap();

        let request_name = random_request_name();
        let create_request_output = collection
            .create_request(CreateRequestInput {
                name: request_name.clone(),
                relative_path: None,
                payload: None,
                url: None,
            })
            .await
            .unwrap();

        let old_request_path = PathBuf::from(format!("{}.request", request_name));
        let new_request_name = random_request_name();
        let rename_collection_result = collection
            .rename_request(RenameRequestInput {
                key: create_request_output.key,
                new_name: new_request_name.clone(),
            })
            .await;
        let new_request_path = PathBuf::from(format!("{}.request", new_request_name));
        rename_collection_result.unwrap();
        // assert!(rename_collection_result.is_ok());

        let requests = collection.requests().await.unwrap();
        let requests_lock = requests.read().await;
        let request_key = RequestKey::from(create_request_output.key);

        let request = requests_lock.read(request_key).unwrap();
        assert_eq!(request.name, new_request_name);

        // Clean up
        {
            tokio::fs::remove_dir_all(collection_path).await.unwrap();
        }
    }

    #[tokio::test]
    async fn rename_request_invalid_name() {
        let workspace_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
        let collection_path = workspace_path.join(random_collection_name());
        tokio::fs::create_dir_all(&collection_path).await.unwrap();

        let collection =
            Collection::new(collection_path.clone(), Arc::new(DiskFileSystem::new())).unwrap();

        let request_name = random_request_name();
        let create_request_output = collection
            .create_request(CreateRequestInput {
                name: request_name.clone(),
                relative_path: None,
                payload: None,
                url: None,
            })
            .await
            .unwrap();

        let new_request_name = "".to_string();
        let rename_collection_result = collection
            .rename_request(RenameRequestInput {
                key: create_request_output.key,
                new_name: new_request_name.clone(),
            })
            .await;
        assert!(rename_collection_result.is_err());
    }

    #[tokio::test]
    async fn rename_request_forbidden_characters() {
        let workspace_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
        let collection_path = workspace_path.join(random_collection_name());
        tokio::fs::create_dir_all(&collection_path).await.unwrap();

        let collection =
            Collection::new(collection_path.clone(), Arc::new(DiskFileSystem::new())).unwrap();

        let request_name = random_request_name();
        let create_request_output = collection
            .create_request(CreateRequestInput {
                name: request_name.clone(),
                relative_path: None,
                payload: None,
                url: None,
            })
            .await
            .unwrap();

        let new_request_name = format!("{}/name", random_request_name());
        let rename_collection_result = collection
            .rename_request(RenameRequestInput {
                key: create_request_output.key,
                new_name: new_request_name.clone(),
            })
            .await;

        rename_collection_result.unwrap();
        // assert!(rename_collection_result.is_ok());

        let requests = collection.requests().await.unwrap();
        let requests_lock = requests.read().await;
        let request_key = RequestKey::from(create_request_output.key);

        let request = requests_lock.read(request_key).unwrap();
        assert_eq!(request.name, new_request_name);

        // Clean up
        {
            tokio::fs::remove_dir_all(collection_path).await.unwrap();
        }
    }

    #[tokio::test]
    async fn rename_request_already_exists() {
        let workspace_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
        let collection_path = workspace_path.join(random_collection_name());
        tokio::fs::create_dir_all(&collection_path).await.unwrap();

        let collection =
            Collection::new(collection_path.clone(), Arc::new(DiskFileSystem::new())).unwrap();

        let request_name = random_request_name();
        let create_request_output = collection
            .create_request(CreateRequestInput {
                name: request_name.clone(),
                relative_path: None,
                payload: None,
                url: None,
            })
            .await
            .unwrap();

        let old_request_path = PathBuf::from(format!("{}.request", request_name));
        let new_request_name = request_name.clone();
        let rename_collection_result = collection
            .rename_request(RenameRequestInput {
                key: create_request_output.key,
                new_name: new_request_name.clone(),
            })
            .await;

        assert!(rename_collection_result.is_err());

        // Clean up
        {
            tokio::fs::remove_dir_all(collection_path).await.unwrap();
        }
    }

    #[tokio::test]
    async fn delete_request() {
        let workspace_path: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
        let collection_path = workspace_path.join(random_collection_name());
        tokio::fs::create_dir_all(&collection_path).await.unwrap();

        let collection =
            Collection::new(collection_path.clone(), Arc::new(RealFileSystem::new())).unwrap();

        let request_name = random_request_name();
        let request_path = PathBuf::from(format!("{}.request", request_name));
        let create_request_output = collection
            .create_request(CreateRequestInput {
                name: request_name.clone(),
                relative_path: None,
                payload: None,
                url: None,
            })
            .await
            .unwrap();

        let request_key = RequestKey::from(create_request_output.key);

        {
            let requests = collection.requests().await.unwrap();
            let requests_lock = requests.read().await;
            assert!(requests_lock.read(request_key).is_ok());
        }

        let delete_request_result = collection
            .delete_request(DeleteRequestInput {
                key: create_request_output.key,
            })
            .await;

        assert!(delete_request_result.is_ok());

        let requests = collection.requests().await.unwrap();
        let requests_lock = requests.read().await;

        assert!(requests_lock.read(request_key).is_err());

        // Clean up
        {
            tokio::fs::remove_dir_all(collection_path).await.unwrap();
        }
    }
}
