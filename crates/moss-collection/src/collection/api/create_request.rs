use anyhow::Context as _;
use moss_common::api::{OperationError, OperationResult};
use moss_fs::{
    utils::{encode_name, encode_path},
    CreateOptions,
};
use moss_storage::collection_storage::entities::request_store_entities::{
    RequestEntity, RequestNodeEntity,
};
use std::path::PathBuf;
use validator::Validate;

use crate::{
    collection::{Collection, CollectionRequestData, REQUESTS_DIR},
    collection_registry::RequestNode,
    constants::{
        DELETE_ENTRY_SPEC_FILE, GET_ENTRY_SPEC_FILE, POST_ENTRY_SPEC_FILE, PUT_ENTRY_SPEC_FILE,
    },
    kdl::http::HttpRequestFile,
    models::{
        operations::{
            CreateRequestInput, CreateRequestOutput, CreateRequestProtocolSpecificPayload,
        },
        types::HttpMethod,
    },
};

impl Collection {
    pub async fn create_request(
        &self,
        input: CreateRequestInput,
    ) -> OperationResult<CreateRequestOutput> {
        input.validate()?;

        let request_dir_name = format!("{}.request", encode_name(&input.name));

        let request_dir_relative_path = if let Some(relative_path) = input.relative_path {
            encode_path(&relative_path, None)?
        } else {
            PathBuf::new()
        }
        .join(&request_dir_name);

        let request_dir_abs_path = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&request_dir_relative_path);

        if request_dir_abs_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.name,
                path: request_dir_abs_path,
            });
        }

        let (file_content, spec_file_name) = match input.payload {
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

                let file_name = match method {
                    HttpMethod::Post => POST_ENTRY_SPEC_FILE,
                    HttpMethod::Get => GET_ENTRY_SPEC_FILE,
                    HttpMethod::Put => PUT_ENTRY_SPEC_FILE,
                    HttpMethod::Delete => DELETE_ENTRY_SPEC_FILE,
                };

                (request_file.to_string(), file_name.to_string())
            }

            None => ("".to_string(), GET_ENTRY_SPEC_FILE.to_string()),
        };

        let request_store = self.collection_storage.request_store().await;
        let request_nodes = self.registry().await?.requests_nodes();

        let mut txn = self.collection_storage.begin_write().await?;
        request_store.set_request_node(
            &mut txn,
            request_dir_relative_path.clone(),
            RequestNodeEntity::Request(RequestEntity { order: None }),
        )?;

        self.fs
            .create_dir(&request_dir_abs_path)
            .await
            .context("Failed to create the request directory")?;

        self.fs
            .create_file_with(
                &request_dir_abs_path.join(&spec_file_name),
                file_content,
                CreateOptions::default(),
            )
            .await
            .context("Failed to create the request file")?;

        let request_key = {
            let mut requests_lock = request_nodes.write().await;
            requests_lock.insert(RequestNode::Request(CollectionRequestData {
                name: input.name,
                path: request_dir_relative_path.clone(),
                order: None,
                spec_file_name,
            }))
        };

        txn.commit()?;

        Ok(CreateRequestOutput { key: request_key })
    }
}
