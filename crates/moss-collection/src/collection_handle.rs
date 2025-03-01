use anyhow::Result;
use kdl::KdlValue;
use moss_fs::ports::{CreateOptions, FileSystem};
use parking_lot::RwLock;
use patricia_tree::PatriciaMap;
use serde_json::Value as JsonValue;
use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::{
    kdl::foundations::http::{HttpRequestFile, QueryParamBody, QueryParamOptions, Url},
    models::{
        operations::collection_operations::{
            CreateRequestInput, CreateRequestProtocolSpecificPayload,
        },
        types::request_types::HttpMethod,
    },
    request_handle::RequestHandle,
    storage::CollectionRequestSubstore,
};

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

    pub fn get_request_handle_or_init(
        &self,
        key: &[u8],
        f: impl FnOnce() -> RequestHandle,
    ) -> Arc<RequestHandle> {
        {
            let read_guard = self.requests.read();
            if let Some(entry) = read_guard.get(key) {
                return Arc::clone(&entry);
            }
        }

        let mut write_guard = self.requests.write();
        if let Some(entry) = write_guard.get(key) {
            return Arc::clone(&entry);
        }

        let entry = Arc::new(f());
        write_guard.insert(key, Arc::clone(&entry));
        entry
    }

    pub fn insert_request_handle(&self, key: &[u8], handle: RequestHandle) -> Result<()> {
        let mut write_guard = self.requests.write();
        write_guard.insert(key, Arc::new(handle));
        Ok(())
    }

}

pub struct CollectionHandle {
    fs: Arc<dyn FileSystem>,
    store: Arc<dyn CollectionRequestSubstore>,
    state: Arc<CollectionState>,
}

impl CollectionHandle {
    pub fn new(
        fs: Arc<dyn FileSystem>,
        store: Arc<dyn CollectionRequestSubstore>,
        name: String,
        order: Option<usize>,
    ) -> Self {
        Self {
            fs,
            store,
            state: Arc::new(CollectionState::new(name, order)),
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
    ) -> Result<()> {
        // TODO: update `store` and `state`
        let requests_dir = collection_path.join("requests");
        let path = if let Some(path) = relative_path {
            requests_dir.join(path)
        } else {
            requests_dir
        };

        let request_dir = path.join(format!("{}.request", input.name));
        self.fs.create_dir(&request_dir).await?;

        let (request_file_content, request_type) = match input.payload {
            Some(CreateRequestProtocolSpecificPayload::Http {
                method,
                query_params,
            }) => {
                let mut transformed_query_params = HashMap::new();
                for item in &query_params {
                    let value = match &item.value {
                        JsonValue::Null => "".to_string(),
                        JsonValue::Bool(value) => value.to_string(),
                        JsonValue::Number(value) => value.to_string(),
                        JsonValue::String(value) => value.to_string(),
                        JsonValue::Array(_) => {
                            // TODO: Invalid type, logging
                            continue;
                        }
                        JsonValue::Object(_) => {
                            // TODO: Invalid type, logging
                            continue;
                        }
                    };

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

                (
                    HttpRequestFile {
                        url: input.url.unwrap_or(Url::default()),
                        query_params: transformed_query_params,
                        path_params: Default::default(),
                        headers: Default::default(),
                    }
                    .to_string(),
                    method_to_request_type_str(&method),
                )
            }

            None => (String::new(), "get".to_string()),
        };

        self.fs
            .create_file_with(
                &request_dir.join(format!("{}.{}.sapic", input.name, request_type)),
                request_file_content,
                CreateOptions::default(),
            )
            .await?;

        Ok(())
    }

    // FIXME
    // pub async fn rename_request(&self, )


}

#[cfg(test)]
mod tests {
    use moss_fs::adapters::disk::DiskFileSystem;

    use crate::{
        models::{
            operations::collection_operations::CreateRequestProtocolSpecificPayload,
            types::request_types::{HttpMethod, QueryParamItem, QueryParamOptions},
        },
        storage::MockCollectionRequestSubstore,
    };

    use super::*;

    const TEST_COLLECTION_PATH: &'static str =
        "TestCollection";

    #[test]
    fn create_request() {
        let fs = Arc::new(DiskFileSystem::new());
        let collection_request_substore = MockCollectionRequestSubstore::new();

        let handle = CollectionHandle::new(
            fs,
            Arc::new(collection_request_substore),
            "TestCollection".to_string(),
            None,
        );

        let fut = async {
            handle
                .create_request(
                    &PathBuf::from(TEST_COLLECTION_PATH),
                    None,
                    CreateRequestInput {
                        name: "Test42".to_string(),
                        url: Some(
                            Url::new("https://spacex-production.up.railway.app".to_string())),
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
                        }),
                    },
                )
                .await
                .unwrap();
        };

        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(fut);
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
