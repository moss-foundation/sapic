use moss_common::api::{OperationError, OperationResult};
use validator::Validate;

use crate::{
    collection::Collection,
    kdl::http::HttpRequestFile,
    models::{
        operations::{
            CreateRequestEntryInput, CreateRequestEntryOutput, CreateRequestProtocolSpecificPayload,
        },
        types::HttpMethod,
    },
};

impl Collection {
    // TODO: return key or something
    pub async fn create_request_entry(
        &self,
        input: CreateRequestEntryInput,
    ) -> OperationResult<CreateRequestEntryOutput> {
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
            OperationError::InvalidInput(format!(
                "Invalid destination path: {}",
                input.destination.display()
            ))
        })?;

        // Updating the last segment of the path to create a directory with the correct extension.
        // The directory extension is necessary to distinguish regular subdirs from unit dirs.
        encoded_path.set_file_name(format!("{}.request", last_segment.to_string_lossy()));

        let mut changes = vec![];

        let create_dir_changes = worktree.create_entry(&encoded_path, true, None).await?;
        changes.extend(create_dir_changes.into_iter().cloned());

        let spec_file_name = format!("{}.sapic", protocol_as_string);
        let create_file_changes = worktree
            .create_entry(
                encoded_path.join(spec_file_name),
                false,
                Some(content_as_bytes),
            )
            .await?;
        changes.extend(create_file_changes.into_iter().cloned());

        // TODO: update the state database
        // let mut txn = self.collection_storage.begin_write().await?;
        // let request_store = self.collection_storage.request_store().await;
        // request_store.upsert_request_node(
        //     &mut txn,
        //     encoded_path,
        //     RequestNodeEntity::Request(RequestEntity { order: None }), // TODO: add order
        // )?;
        // txn.commit()?;

        Ok(CreateRequestEntryOutput {
            changed_paths: changes.into(),
        })
    }
}
