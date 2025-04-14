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

        // If the input path begins with or ends with a slash/backslash
        // The slash will be normalized away
        // For example: if input.path == "folder/", the encoded_path will be "folder"
        // In RustRover, the (back)slash will be ignore both at the beginning and at the end
        // In VS Code, a leading (back)slash will prompt an error, and a trailing one will be ignored
        // Thus I think the current behavior makes sense

        let encoded_path = encode_path(None, &input.path)?;
        let request_group_full_path = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&encoded_path);

        if request_group_full_path.exists() {
            return Err(OperationError::RequestGroupAlreadyExists {
                path: input.path
            });
        }

        // TODO: Update state_db_manager and request_map?
        self.fs
            .create_dir(&request_group_full_path)
            .await
            .context("Failed to create the request group directory")?;

        // Create an empty folder.sapic spec file
        // Otherwise the indexer will not recognize the request group
        // FIXME: Should we create a spec file for all the folders created in the process
        // Or only the innermost one like we are doing now?
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