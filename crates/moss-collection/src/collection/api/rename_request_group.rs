use anyhow::Result;
use moss_common::api::{OperationError, OperationResult};
use moss_common::leased_slotmap::ResourceKey;
use moss_fs::utils::encode_name;
use moss_fs::RenameOptions;
use std::path::Path;
use validator::Validate;

use crate::collection::Collection;
use crate::constants::REQUESTS_DIR;
use crate::models::operations::RenameRequestGroupInput;

impl Collection {
    async fn update_request_relative_path(
        &self,
        key: ResourceKey,
        old_prefix: &Path,
        new_prefix: &Path,
    ) -> Result<()> {
        let request_store = self.state_db_manager.request_store().await;
        let requests = self.registry().await?.requests_nodes();

        let mut requests_lock = requests.write().await;
        let (mut txn, table) = request_store.begin_write()?;

        // Update the request map entry
        let entry_relative_path_old = requests_lock.get(key)?.path().clone();
        let entry_relative_path_new =
            new_prefix.join((&entry_relative_path_old).strip_prefix(&old_prefix)?);
        requests_lock
            .get_mut(key)?
            .set_path(entry_relative_path_new.clone());

        // Update the state db
        let entity = table.remove(
            &mut txn,
            entry_relative_path_old.to_string_lossy().to_string(),
        )?;

        table.insert(
            &mut txn,
            entry_relative_path_new.to_string_lossy().to_string(),
            &entity,
        )?;

        Ok(txn.commit()?)
    }

    pub async fn rename_request_group(
        &self,
        input: RenameRequestGroupInput,
    ) -> OperationResult<()> {
        input
            .validate()
            .map_err(|error| OperationError::Validation(error.to_string()))?;

        let request_nodes = self.registry().await?.requests_nodes();

        let group_dir_relative_path_old = {
            let requests_lock = request_nodes.read().await;

            let group_data = requests_lock.get(input.key)?;

            if !group_data.is_request_group() {
                return Err(OperationError::Validation(format!(
                    "Resource {} is not a request group",
                    input.key
                )));
            }

            group_data.path().to_owned()
        };

        let new_encoded_name = encode_name(&input.new_name);
        let group_dir_relative_path_new = group_dir_relative_path_old
            .parent()
            .expect("Relative path should not be empty or end in root/prefix")
            .join(&new_encoded_name);
        if group_dir_relative_path_old == group_dir_relative_path_new {
            return Ok(());
        }

        let group_dir_abs_path_old = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&group_dir_relative_path_old);

        if !group_dir_abs_path_old.exists() {
            return Err(OperationError::NotFound {
                name: group_dir_relative_path_old
                    .file_name()
                    .expect("Relative path should not terminate in ..")
                    .to_string_lossy()
                    .to_string(),
                path: group_dir_abs_path_old,
            });
        }

        let group_abs_path_new = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&group_dir_relative_path_new);

        if group_abs_path_new.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.new_name,
                path: group_abs_path_new,
            });
        }

        self.fs
            .rename(
                &group_dir_abs_path_old,
                &group_abs_path_new,
                RenameOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        // Find all entities that fall inside the request group
        let keys_to_update = {
            let requests_lock = request_nodes.read().await;
            requests_lock
                .iter()
                .filter_map(|(key, iter_slot)| {
                    if iter_slot
                        .value()
                        .path()
                        .starts_with(&group_dir_relative_path_old)
                        && iter_slot.value().path() != &group_dir_relative_path_old
                    {
                        Some(key)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        };

        for key in keys_to_update {
            self.update_request_relative_path(
                key,
                &group_dir_relative_path_old,
                &group_dir_relative_path_new,
            )
            .await?;
        }

        let request_store = self.state_db_manager.request_store().await;
        let (mut txn, table) = request_store.begin_write()?;
        let store_entity = table.remove(
            &mut txn,
            group_dir_relative_path_old.to_string_lossy().to_string(),
        )?;
        table.insert(
            &mut txn,
            group_dir_relative_path_new.to_string_lossy().to_string(),
            &store_entity,
        )?;

        let mut requests_lock = request_nodes.write().await;
        let mut lease_group_data = requests_lock.lease(input.key)?;

        lease_group_data.set_name(input.new_name);
        lease_group_data.set_path(group_dir_relative_path_new);

        Ok(txn.commit()?)
    }
}
