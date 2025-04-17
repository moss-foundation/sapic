use anyhow::Context as _;
use moss_fs::utils::encode_name;
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::{
        entities::CollectionEntity,
        operations::{RenameCollectionInput, RenameCollectionOutput},
    },
    workspace::{OperationError, Workspace},
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn rename_collection(
        &self,
        input: RenameCollectionInput,
    ) -> Result<RenameCollectionOutput, OperationError> {
        input.validate()?;

        let collections = self
            .collections()
            .await
            .context("Failed to get collections")?;

        let mut collections_lock = collections.write().await;
        let mut lease_guard = collections_lock
            .lease(input.key)
            .context("Failed to lease the collection")?;

        let (collection, metadata) = &mut *lease_guard;

        if metadata.name == input.new_name {
            return Ok(RenameCollectionOutput {
                path: collection.path().to_owned(),
            });
        }

        let old_full_path = collection.path().to_owned();
        if !old_full_path.exists() {
            return Err(OperationError::NotFound {
                name: metadata.name.clone(),
                path: old_full_path,
            });
        }

        // requests/request_name
        let old_relative_path = old_full_path.strip_prefix(&self.path).unwrap();
        let new_relative_path = old_relative_path
            .parent()
            .context("Parent directory not found")?
            .join(encode_name(&input.new_name));
        let new_full_path = self.path.join(&new_relative_path);

        if new_full_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: input.new_name,
                path: new_full_path,
            });
        }

        let collection_store = self.state_db_manager.collection_store();
        let (mut txn, table) = collection_store.begin_write()?;

        let old_table_key = old_relative_path.to_string_lossy().to_string();
        let new_table_key = new_relative_path.to_string_lossy().to_string();

        table.remove(&mut txn, old_table_key)?;
        table.insert(
            &mut txn,
            new_table_key,
            &CollectionEntity {
                order: metadata.order,
            },
        )?;

        // The state_db_manager will hold the `state.db` file open, preventing renaming on Windows
        // We need to temporarily drop it, and reload the database after that
        collection
            .reset(new_full_path.clone())
            .await
            .context("Failed to reset the collection")?;

        metadata.name = input.new_name.clone();

        txn.commit()?;

        Ok(RenameCollectionOutput {
            path: new_full_path,
        })
    }
}
