use anyhow::Context as _;
use moss_fs::{utils::encode_directory_name, CreateOptions};
use validator::Validate;

use crate::{
    collection::{
        primitives::FileExt, Collection, CollectionRequestData, OperationError, REQUESTS_DIR,
    },
    kdl::http::HttpRequestFile,
    models::{
        operations::{
            CreateRequestInput, CreateRequestOutput, CreateRequestProtocolSpecificPayload,
        },
        storage::RequestEntity,
        types::{HttpMethod, RequestProtocol},
    },
};

impl Collection {
    pub async fn create_request(
        &self,
        input: CreateRequestInput,
    ) -> Result<CreateRequestOutput, OperationError> {
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

        let (file_content, file_ext, protocol) = match input.payload {
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

                let protocol = RequestProtocol::Http(method);

                (request_file.to_string(), FileExt::from(&protocol), protocol)
            }

            // FIXME:
            None => {
                let protocol = RequestProtocol::Http(HttpMethod::Get);

                ("".to_string(), FileExt::from(&protocol), protocol)
            }
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
        let request_file_name = format!(
            "{}.{}.sapic",
            encode_directory_name(&input.name),
            file_ext.to_string()
        );
        self.fs
            .create_dir(&request_dir_full_path)
            .await
            .context("Failed to create the collection directory")?;
        self.fs
            .create_file_with(
                &request_dir_full_path.join(request_file_name),
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
                request_dir_relative_path: request_dir_relative_path.clone(),
                order: None,
                protocol,
            })
        };

        Ok(CreateRequestOutput { key: request_key })
    }
}
