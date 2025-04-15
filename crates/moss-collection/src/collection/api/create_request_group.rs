use anyhow::Context;
use validator::Validate;
use moss_fs::CreateOptions;
use moss_fs::utils::encode_path;
use crate::collection::{Collection, OperationError};
use crate::constants::{FOLDER_ENTRY_SPEC_FILE, REQUESTS_DIR};
use crate::models::operations::{CreateRequestGroupInput, CreateRequestGroupOutput};

impl Collection {
    pub async fn create_request_group(
        &self,
        input: CreateRequestGroupInput
    ) -> Result<CreateRequestGroupOutput, OperationError> {
        input.validate()?;

        let encoded_path = encode_path(&input.path, None)?;
        let request_group_full_path = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&encoded_path);

        if request_group_full_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: request_group_full_path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                path: input.path
            });
        }

        // TODO: Update state_db_manager and request_map?
        self.fs
            .create_dir(&request_group_full_path)
            .await
            .context("Failed to create the request group directory")?;

        self.fs
            .create_file(
                &request_group_full_path.join(FOLDER_ENTRY_SPEC_FILE),
                CreateOptions::default(),
            )
            .await
            .context("Failed to create the request group spec file")?;

        Ok(CreateRequestGroupOutput { })
    }
}