use std::path::PathBuf;
use anyhow::Context as _;
use moss_fs::{
    utils::{
        encode_directory_name, encode_path
    },
    CreateOptions
};
use validator::Validate;
use crate::{
    collection::{Collection, CollectionRequestData, OperationError, REQUESTS_DIR},
    constants::{
        DELETE_ENTRY_SPEC_FILE, GET_ENTRY_SPEC_FILE, POST_ENTRY_SPEC_FILE, PUT_ENTRY_SPEC_FILE,
    },
    kdl::http::HttpRequestFile,
    models::{
        operations::{
            CreateRequestInput, CreateRequestOutput, CreateRequestProtocolSpecificPayload,
        },
        storage::RequestEntity,
        types::HttpMethod,
    },
};

impl Collection {
    pub async fn create_request(
        &self,
        input: CreateRequestInput,
    ) -> Result<CreateRequestOutput, OperationError> {
        input.validate()?;

        let request_dir_name = format!("{}.request", encode_directory_name(&input.name));

        let request_dir_relative_path = if let Some(relative_path) = input.relative_path {
            encode_path(None, &relative_path)?
        } else {
            PathBuf::new()
        }.join(&request_dir_name);

        let request_dir_full_path = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&request_dir_relative_path);

        if request_dir_full_path.exists() {
            return Err(OperationError::RequestAlreadyExists {
                name: input.name,
                path: request_dir_full_path,
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

        let request_store = self.state_db_manager.request_store().await;
        let requests = self.requests().await?;

        let (mut txn, table) = request_store.begin_write()?;
        table.insert(
            &mut txn,
            request_dir_relative_path.to_string_lossy().to_string(),
            &RequestEntity { order: None },
        )?;

        self.fs
            .create_dir(&request_dir_full_path)
            .await
            .context("Failed to create the request directory")?;
        self.fs
            .create_file_with(
                &request_dir_full_path.join(&spec_file_name),
                file_content,
                CreateOptions::default(),
            )
            .await
            .context("Failed to create the request file")?;

        txn.commit()?;

        let request_key = {
            let mut requests_lock = requests.write().await;
            requests_lock.insert(CollectionRequestData {
                name: input.name,
                entry_relative_path: request_dir_relative_path.clone(),
                order: None,
                spec_file_name,
            })
        };

        Ok(CreateRequestOutput { key: request_key })
    }
}
