
use crate::{
    kdl::foundations::http::{
        HeaderBody, HttpRequestFile, PathParamBody, QueryParamBody, QueryParamOptions, Url, HeaderOptions, PathParamOptions
    },
    models::{
        operations::collection_operations::{
            CreateRequestInput, CreateRequestProtocolSpecificPayload,
        },
        types::request_types::{
            HttpMethod, HeaderItem, PathParamItem, QueryParamItem
        },
    },
    request_handle::{
        RequestHandle, RequestState
    },
    storage::CollectionRequestSubstore,
};
use anyhow::Result;
use moss_fs::ports::{CreateOptions, FileSystem, RemoveOptions, RenameOptions};
use parking_lot::RwLock;
use patricia_tree::PatriciaMap;
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum RequestOperationError {
    #[error("The name of a request cannot be empty.")]
    EmptyName,
    #[error("`{name}` is an invalid name for a request.")]
    InvalidName { name: String }, // TODO: validate name
    #[error("A request named {name} already exists in {path}.")]
    DuplicateName { path: PathBuf, name: String },
    #[error("The request named {name} does not exist in {path}.")]
    NonexistentRequest { path: PathBuf, name: String },
}

pub(crate) struct CollectionState {
    pub name: String,
    pub order: Option<usize>,
    pub requests: RwLock<PatriciaMap<Arc<RequestHandle>>>,
}

impl CollectionState {
    pub fn new(name: String, order: Option<usize>) -> Self {
        Self {
            name,
            order,
            requests: RwLock::new(PatriciaMap::new()),
        }
    }

    pub fn contains(&self, key: impl AsRef<[u8]>) -> bool {
        let read_guard = self.requests.read();
        read_guard.contains_key(key.as_ref())
    }

    pub fn get_request_handle(&self, key: impl AsRef<[u8]>) -> Option<Arc<RequestHandle>> {
        let key = key.as_ref();
        let read_guard = self.requests.read();
        if let Some(entry) = read_guard.get(key) {
            Some(Arc::clone(&entry))
        } else {
            None
        }
    }

    pub fn get_request_handle_or_init(
        &self,
        key: impl AsRef<[u8]>,
        f: impl FnOnce() -> RequestHandle,
    ) -> Arc<RequestHandle> {
        let key = key.as_ref();
        if let Some(entry) = self.get_request_handle(key) {
            return entry;
        }

        let mut write_guard = self.requests.write();
        if let Some(entry) = write_guard.get(key) {
            return Arc::clone(&entry);
        }

        let entry = Arc::new(f());
        write_guard.insert(key, Arc::clone(&entry));
        entry
    }

    pub fn insert_request_handle(
        &self,
        key: impl AsRef<[u8]>,
        handle: RequestHandle,
    ) -> Result<()> {
        let mut write_guard = self.requests.write();
        write_guard.insert(key.as_ref(), Arc::new(handle));
        Ok(())
    }

    pub fn rename_request_handle(
        &self,
        old_key: impl AsRef<[u8]>,
        new_key: impl AsRef<[u8]>,
        new_name: &str,
    ) -> Result<()> {
        let mut write_guard = self.requests.write();
        let old_handle = write_guard.remove(old_key.as_ref()).unwrap();
        let variants = (*old_handle.state.variants.read()).clone();
        let new_handle = RequestHandle::new(
            old_handle.fs.clone(),
            RequestState {
                name: new_name.to_string(),
                order: old_handle.state.order,
                typ: old_handle.state.typ.clone(),
                variants: RwLock::new(variants),
            },
        );

        write_guard.insert(new_key, Arc::new(new_handle));
        Ok(())
    }

    pub fn remove_request_handle(&self, key: impl AsRef<[u8]>) -> Result<()> {
        let mut write_guard = self.requests.write();
        write_guard.remove(key.as_ref());
        Ok(())
    }
}

pub struct CollectionHandle {
    fs: Arc<dyn FileSystem>,
    // TODO: extract request store
    store: Arc<dyn CollectionRequestSubstore>,
    state: Arc<CollectionState>,
}

fn transform_jsonvalue(value: &JsonValue) -> Option<String> {
    // FIXME: Should we accommodate JSONValue::Array/Object?
    match value {
        JsonValue::Null => Some("".to_string()),
        JsonValue::Bool(value) => Some(value.to_string()),
        JsonValue::Number(value) => Some(value.to_string()),
        JsonValue::String(value) => Some(value.to_string()),
        JsonValue::Array(_) => {
            None
            // TODO: Invalid type, logging
        }
        JsonValue::Object(_) => {
            None
            // TODO: Invalid type, logging
        }
    }
}

fn request_dir(collection_path: &PathBuf, relative_path: Option<PathBuf>, name: &str) -> PathBuf {
    let requests_dir = collection_path.join("requests");
    let path = if let Some(path) = relative_path {
        requests_dir.join(path)
    } else {
        requests_dir
    };
     path.join(format!("{}.request", name))
}
fn create_http_requestfile(
    url: Option<Url>,
    query_params: Vec<QueryParamItem>,
    path_params: Vec<PathParamItem>,
    headers: Vec<HeaderItem>
)
    -> Result<HttpRequestFile> {
    let mut transformed_query_params = HashMap::new();
    for item in &query_params {
        if let Some(value) = transform_jsonvalue(&item.value) {
            transformed_query_params.insert(
                item.key.clone(),
                QueryParamBody {
                    value,
                    desc: item.desc.clone(),
                    order: item.order,
                    disabled: item.disabled,
                    options: QueryParamOptions {
                        propagate: item.options.propagate,
                    },
                },
            );
        }
    }
    let mut transformed_path_params = HashMap::new();
    for item in &path_params {
        if let Some(value) = transform_jsonvalue(&item.value) {
            transformed_path_params.insert(
                item.key.clone(),
                PathParamBody {
                    value,
                    desc: item.desc.clone(),
                    order: item.order,
                    disabled: item.disabled,
                    options: PathParamOptions {
                        propagate: item.options.propagate,
                    },
                },
            );
        }
    }
    let mut transformed_headers = HashMap::new();
    for item in &headers {
        if let Some(value) = transform_jsonvalue(&item.value) {
            transformed_headers.insert(
                item.key.clone(),
                HeaderBody {
                    value,
                    desc: item.desc.clone(),
                    order: item.order,
                    disabled: item.disabled,
                    options: HeaderOptions {
                        propagate: item.options.propagate,
                    },
                },
            );
        }
    }

    Ok(HttpRequestFile {
        url: url.unwrap_or(Url::default()),
        query_params: transformed_query_params,
        path_params: transformed_path_params,
        headers: transformed_headers,
    })
}

impl CollectionHandle {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        store: Arc<dyn CollectionRequestSubstore>,
        name: String,
        order: Option<usize>,
    )
        -> Self {
        Self {
            fs,
            store,
            state: Arc::new(CollectionState::new(name, order)),
        }
    }

    pub fn with_state(
        fs: Arc<dyn FileSystem>,
        store: Arc<dyn CollectionRequestSubstore>,
        state: CollectionState
    ) -> Self {
        Self {
            fs,
            store,
            state: Arc::new(state),
        }
    }

    pub(crate) fn state(&self) -> Arc<CollectionState> {
        Arc::clone(&self.state)
    }

    pub async fn create_request(
        &self,
        collection_path: &PathBuf,
        relative_path: Option<PathBuf>,
        input: CreateRequestInput,
    )
        -> Result<()> {
        let name = input.name;
        if name.trim().is_empty() {
            return Err(RequestOperationError::EmptyName.into());
        }
        let request_dir = request_dir(collection_path, relative_path, &name);
        let key = request_dir.to_string_lossy().to_string();
        if self.state.contains(&key) {
            return Err(RequestOperationError::DuplicateName { path, name }.into());
        }

        self.fs.create_dir(&request_dir).await?;

        let (request_file_content, request_type) = match input.payload {
            Some(CreateRequestProtocolSpecificPayload::Http {
                method,
                query_params,
                path_params,
                headers,
            }) => {
                let request_file = create_http_requestfile(input.url, query_params, path_params, headers)?;
                self.state.insert_request_handle(
                    key,
                    RequestHandle::new(
                        self.fs.clone(),
                        RequestState {
                            name: name.clone(),
                            order: None,
                            typ: Some(method.clone().into()),
                            // TODO: handling variants
                            variants: Default::default(),
                        },
                    ),
                )?;
                (
                    request_file.to_string(),
                    method_to_request_type_str(&method),
                )
            }

            None => {
                self.state().insert_request_handle(
                    key,
                    RequestHandle::new(
                        self.fs.clone(),
                        RequestState {
                            name: name.clone(),
                            order: None,
                            typ: None,
                            variants: Default::default(),
                        },
                    ),
                )?;
                (String::new(), "get".to_string())
            }
        };

        self.fs
            .create_file_with(
                &request_dir.join(format!("{}.{}.sapic", name, request_type)),
                request_file_content,
                CreateOptions::default(),
            )
            .await
    }


    pub async fn rename_request(
        &self,
        collection_path: &PathBuf,
        relative_path: Option<PathBuf>,
        old_name: &str,
        new_name: &str,
    )
        -> Result<()> {
        if new_name.trim().is_empty() {
            return Err(RequestOperationError::EmptyName.into());
        }
        let requests_dir = collection_path.join("requests");
        let path = if let Some(path) = relative_path {
            requests_dir.join(path)
        } else {
            requests_dir
        };
        let old_request_dir = path.join(format!("{}.request", old_name));
        let old_key = old_request_dir.to_string_lossy().to_string();
        if !self.state.contains(&old_key) {
            return Err(RequestOperationError::NonexistentRequest {
                path,
                name: old_name.to_string(),
            }
            .into());
        }

        // FIXME: Theoretically, a batch rename operation might fail in the middle
        // Resulting in some files successfully renamed and others not
        // Should we consider this scenario? How should we handle it?

        let new_request_dir = path.join(format!("{}.request", new_name));
        let new_key = new_request_dir.to_string_lossy().to_string();
        self.fs
            .rename(&old_request_dir, &new_request_dir, RenameOptions::default())
            .await?;

        let mut dir = self.fs.read_dir(&new_request_dir).await?;

        while let Some(entry) = dir.next_entry().await? {
            let file_type = entry.file_type().await?;
            if file_type.is_dir() {
                continue;
            }
            let old_path = entry.path();
            let old_filename = old_path.file_name().unwrap().to_string_lossy().to_string();
            let new_filename = format!(
                "{}.{}",
                new_name,
                old_filename
                    .strip_prefix(&format!("{}.", old_name))
                    .unwrap()
            );
            let new_path = old_path.parent().unwrap().join(new_filename);
            self.fs
                .rename(&old_path, &new_path, RenameOptions::default())
                .await?;
        }

        self.state.rename_request_handle(old_key, new_key, new_name)
    }

    pub async fn delete_request(
        &self,
        collection_path: &PathBuf,
        relative_path: Option<PathBuf>,
        name: &str,
    )
        -> Result<()> {
        if name.trim().is_empty() {
            return Err(RequestOperationError::EmptyName.into());
        }
        let request_dir = request_dir(collection_path, relative_path, &name);
        let key = request_dir.to_string_lossy().to_string();
        if !self.state.contains(&key) {
            return Err(RequestOperationError::NonexistentRequest {
                path,
                name: name.to_string(),
            }
            .into());
        }

        self.fs
            .remove_dir(
                &request_dir,
                RemoveOptions {
                    recursive: true,
                    ignore_if_not_exists: true,
                },
            )
            .await?;

        // TODO: update store after implementing db
        self.state.remove_request_handle(key)
    }

    pub async fn update_request(
        &self,
        collection_path: &PathBuf,
        relative_path: Option<PathBuf>,
        name: &str,
        input: UpdateRequestInput
    )
        -> Result<()>
    {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::types::request_types::{HeaderItem, PathParamItem};
    use crate::{
        models::{
            operations::collection_operations::CreateRequestProtocolSpecificPayload,
            types::request_types::{
                HeaderOptions, HttpMethod, PathParamOptions, QueryParamItem, QueryParamOptions,
            },
        },
        storage::MockCollectionRequestSubstore,
    };
    use moss_fs::adapters::disk::DiskFileSystem;
    use moss_fs::ports::RemoveOptions;

    const TEST_COLLECTION_PATH: &'static str = "TestCollection";

    fn collection_handle() -> CollectionHandle {
        let fs = Arc::new(DiskFileSystem::new());
        let collection_request_substore = MockCollectionRequestSubstore::new();

        CollectionHandle::new(
            fs,
            Arc::new(collection_request_substore),
            "TestCollection".to_string(),
            None,
        )
    }

    #[test]
    fn test_create_request() {
        let handle = collection_handle();
        let name = "create".to_string();

        let input = CreateRequestInput {
            name: name.clone(),
            url: Some(Url::new(
                "https://spacex-production.up.railway.app".to_string(),
            )),
            payload: Some(CreateRequestProtocolSpecificPayload::Http {
                method: HttpMethod::Get,
                query_params: vec![QueryParamItem {
                    key: "pageToken".to_string(),
                    value: JsonValue::Null,
                    order: Some(1),
                    desc: None,
                    disabled: false,
                    options: QueryParamOptions { propagate: true },
                }],
                path_params: vec![PathParamItem {
                    key: "docId".to_string(),
                    value: JsonValue::Null,
                    order: Some(1),
                    desc: None,
                    disabled: false,
                    options: PathParamOptions { propagate: true },
                }],
                headers: vec![HeaderItem {
                    key: "user_agent".to_string(),
                    value: JsonValue::Null,
                    order: Some(1),
                    desc: None,
                    disabled: false,
                    options: HeaderOptions { propagate: true },
                }],
            }),
        };

        let fut = async {
            handle
                .fs
                .remove_dir(
                    &PathBuf::from(TEST_COLLECTION_PATH).join(format!("{}.request", name)),
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .unwrap();
            handle
                .create_request(&PathBuf::from(TEST_COLLECTION_PATH), None, input)
                .await
                .unwrap();
        };

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(fut);

        assert!(handle.state.contains(
            PathBuf::from(TEST_COLLECTION_PATH)
                .join("requests")
                .join("create.request")
                .to_string_lossy()
                .to_string()
        ))
    }

    #[test]
    fn test_create_request_with_duplicate_name() {
        let handle = collection_handle();
        let name = "duplicate".to_string();
        let input = CreateRequestInput {
            name: name.clone(),
            url: None,
            payload: None,
        };

        let fut = async {
            handle
                .fs
                .remove_dir(
                    &PathBuf::from(TEST_COLLECTION_PATH).join(format!("{}.request", name)),
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .unwrap();
            handle
                .create_request(&PathBuf::from(TEST_COLLECTION_PATH), None, input.clone())
                .await
                .unwrap();

            handle
                .create_request(&PathBuf::from(TEST_COLLECTION_PATH), None, input)
                .await
        };

        let result = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(fut);

        assert!(result.is_err());
    }

    #[test]
    fn test_rename_request() {
        let handle = collection_handle();
        let input = CreateRequestInput {
            name: "pre-rename".to_string(),
            url: None,
            payload: None,
        };

        let fut = async {
            handle
                .fs
                .remove_dir(
                    &PathBuf::from(TEST_COLLECTION_PATH).join("pre-rename.request"),
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .unwrap();
            handle
                .fs
                .remove_dir(
                    &PathBuf::from(TEST_COLLECTION_PATH).join("post-rename.request"),
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .unwrap();

            handle
                .create_request(&PathBuf::from(TEST_COLLECTION_PATH), None, input.clone())
                .await
                .unwrap();

            handle
                .rename_request(
                    &PathBuf::from(TEST_COLLECTION_PATH),
                    None,
                    "pre-rename",
                    "post-rename",
                )
                .await
        };

        let result = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(fut);
        dbg!(&result);
        assert!(result.is_ok());
        assert_eq!(
            handle
                .state
                .get_request_handle(
                    PathBuf::from(TEST_COLLECTION_PATH)
                        .join("requests")
                        .join("post-rename.request")
                        .to_string_lossy()
                        .to_string()
                )
                .unwrap()
                .state
                .name,
            "post-rename"
        );
        assert!(!handle.state.contains(
            PathBuf::from(TEST_COLLECTION_PATH)
                .join("requests")
                .join("pre-rename.request")
                .to_string_lossy()
                .to_string()
        ));
    }

    #[test]
    fn test_delete_request() {
        let handle = collection_handle();
        let name = "delete".to_string();
        let input = CreateRequestInput {
            name: name.clone(),
            url: None,
            payload: None,
        };

        let fut = async {
            handle
                .fs
                .remove_dir(
                    &PathBuf::from(TEST_COLLECTION_PATH).join(format!("{}.request", name)),
                    RemoveOptions {
                        recursive: true,
                        ignore_if_not_exists: true,
                    },
                )
                .await
                .unwrap();

            handle
                .create_request(&PathBuf::from(TEST_COLLECTION_PATH), None, input)
                .await
                .unwrap();

            handle
                .delete_request(&PathBuf::from(TEST_COLLECTION_PATH), None, &name)
                .await
        };

        let result = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(fut);

        assert!(result.is_ok());
        assert!(!handle.state.contains(
            PathBuf::from(TEST_COLLECTION_PATH)
                .join("requests")
                .join(format!("{}.request", name))
                .to_string_lossy()
                .to_string()
        ))
    }
}

fn method_to_request_type_str(method: &HttpMethod) -> String {
    match method {
        HttpMethod::Post => "post".to_string(),
        HttpMethod::Get => "get".to_string(),
        HttpMethod::Put => "put".to_string(),
        HttpMethod::Delete => "del".to_string(),
    }
}
