use anyhow::Result;
use moss_common::leased_slotmap::ResourceKey;
use moss_fs::utils::{encode_name, encode_path};
use moss_fs::RenameOptions;
use std::path::Path;
use tokio::io::AsyncWriteExt;
use validator::Validate;

use crate::collection::{Collection, OperationError};
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
    ) -> Result<(), OperationError> {
        input.validate()?;

        // FIXME: we won't need this once we implement `ResourceKey`
        let group_relative_path_old = encode_path(&input.path, None)?;
        let new_encoded_name = encode_name(&input.new_name);
        let group_relative_path_new = group_relative_path_old
            .parent()
            .unwrap()
            .join(&new_encoded_name);

        if group_relative_path_old == group_relative_path_new {
            return Ok(());
        }

        let group_full_path_old = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&group_relative_path_old);

        if !group_full_path_old.exists() {
            return Err(OperationError::NotFound {
                name: group_relative_path_old
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
                path: group_full_path_old,
            });
        }

        let group_full_path_new = self
            .abs_path
            .join(REQUESTS_DIR)
            .join(&group_relative_path_new);

        if group_full_path_new.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.new_name,
                path: group_full_path_new,
            });
        }

        self.fs
            .rename(
                &group_full_path_old,
                &group_full_path_new,
                RenameOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        let requests = self.registry().await?.requests_nodes();
        let requests_lock = requests.read().await;

        let keys_to_rename = requests_lock
            .iter()
            .filter_map(|(key, iter_slot)| {
                if iter_slot
                    .value()
                    .path()
                    .starts_with(&group_relative_path_old)
                {
                    Some(key)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        std::mem::drop(requests_lock);

        for key in keys_to_rename {
            self.update_request_relative_path(
                key,
                &group_relative_path_old,
                &group_relative_path_new,
            )
            .await?;
        }

        Ok(())
    }
}
