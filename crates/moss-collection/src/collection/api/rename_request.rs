use anyhow::{Context as _, Result};
use moss_fs::utils::encode_directory_name;
use moss_fs::RenameOptions;
use validator::Validate;

use crate::collection::{Collection, OperationError, REQUESTS_DIR};
use crate::models::{operations::RenameRequestInput, storage::RequestEntity};

impl Collection {
    pub async fn rename_request(&self, input: RenameRequestInput) -> Result<(), OperationError> {
        input.validate()?;

        let requests = self.requests().await?;
        let mut requests_lock = requests.write().await;

        let mut lease_request_data = requests_lock.lease(input.key)?;

        if lease_request_data.name == input.new_name {
            return Ok(());
        }

        let request_dir_relative_path_old = lease_request_data.entry_relative_path.to_owned();
        let request_dir_path_old = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&request_dir_relative_path_old);
        if !request_dir_path_old.exists() {
            return Err(OperationError::NotFound {
                name: lease_request_data.name.clone(),
                path: request_dir_path_old,
            });
        }

        let request_dir_relative_path_new = lease_request_data
            .entry_relative_path
            .parent()
            .context("Failed to get the parent directory")?
            .join(format!(
                "{}.request",
                encode_directory_name(&input.new_name)
            ));

        let request_dir_path_new = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&request_dir_relative_path_new);
        if request_dir_path_new.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.new_name,
                path: request_dir_path_new,
            });
        }

        self.fs
            .rename(
                &request_dir_path_old,
                &request_dir_path_new,
                RenameOptions::default(),
            )
            .await
            .context("Failed to rename the request directory")?;

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
        lease_request_data.entry_relative_path = request_dir_relative_path_new.clone();
        Ok(txn.commit()?)
    }
}
