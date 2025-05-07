use moss_common::api::{OperationError, OperationResult};
use moss_storage::collection_storage::entities::request_store_entities::{
    RequestEntity, RequestNodeEntity,
};
use validator::Validate;

use crate::{
    collection::Collection,
    constants::{
        DELETE_ENTRY_SPEC_FILE, GET_ENTRY_SPEC_FILE, POST_ENTRY_SPEC_FILE, PUT_ENTRY_SPEC_FILE,
    },
    kdl::http::HttpRequestFile,
    models::{
        operations::{CreateRequestEntryInput, CreateRequestProtocolSpecificPayload},
        types::HttpMethod,
    },
};

impl Collection {
    // TODO: return key or something
    pub async fn create_request(&self, input: CreateRequestEntryInput) -> OperationResult<()> {
        input.validate()?;

        let worktree = self.worktree().await?;

        let (content_as_bytes, protocol_as_string) = match input.payload {
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
                    HttpMethod::Post => "post",
                    HttpMethod::Get => "get",
                    HttpMethod::Put => "put",
                    HttpMethod::Delete => "del",
                };

                (request_file.to_string().into_bytes(), file_name.to_string())
            }

            None => (vec![], "get".to_string()),
        };

        let mut encoded_path = moss_fs::utils::encode_path(&input.destination, None)?;
        let last_segment = encoded_path.file_name().ok_or_else(|| {
            OperationError::Validation(format!(
                "Invalid destination path: {}",
                input.destination.display()
            ))
        })?;

        // Updating the last segment of the path to create a directory with the correct extension.
        // The directory extension is necessary to distinguish regular subdirs from unit dirs.
        encoded_path.set_file_name(format!("{}.request", last_segment.to_string_lossy()));

        let _dir_entry = worktree.create_entry(&encoded_path, true, None).await?;

        let spec_file_name = format!("{}.sapic", protocol_as_string);
        let _file_entry = worktree
            .create_entry(
                encoded_path.join(spec_file_name),
                false,
                Some(content_as_bytes),
            )
            .await?;

        let mut txn = self.collection_storage.begin_write().await?;
        let request_store = self.collection_storage.request_store().await;
        request_store.upsert_request_node(
            &mut txn,
            encoded_path,
            RequestNodeEntity::Request(RequestEntity { order: None }), // TODO: add order
        )?;
        txn.commit()?;

        Ok(())
    }
}
