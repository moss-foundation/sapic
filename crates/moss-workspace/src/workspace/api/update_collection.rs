use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context as _;
use moss_collection::collection::Collection;
use moss_common::{
    api::{OperationError, OperationResult, OperationResultExt},
    models::primitives::Identifier,
};
use moss_fs::{RenameOptions, utils::encode_name};
use tauri::Runtime as TauriRuntime;
use validator::Validate;

use crate::{
    models::operations::{UpdateCollectionEntryInput, UpdateCollectionEntryOutput},
    workspace::{COLLECTIONS_DIR, CollectionEntry, Workspace},
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn update_collection_entry(
        &self,
        input: UpdateCollectionEntryInput,
    ) -> OperationResult<UpdateCollectionEntryOutput> {
        input.validate()?;

        let collections = self
            .collections()
            .await
            .context("Failed to get collections")?;

        if let Some(new_name) = input.new_name {
            self.rename_collection(&input.id, new_name).await?;
        }

        let collection_entry = collections
            .read()
            .await
            .get(&input.id)
            .context("Collection not found")
            .map_err_as_not_found()?
            .clone();

        unimplemented!()

        // Ok(UpdateCollectionEntryOutput {
        //     id: input.id,
        //     abs_path: collection_entry.path().clone().into(), // FIXME:
        // })
    }

    async fn rename_collection(&self, id: &Identifier, new_name: String) -> OperationResult<()> {
        let collections = self.collections().await?;
        let collection_entry = collections
            .read()
            .await
            .get(id) // TODO: call remove here
            .context("Collection not found")
            .map_err_as_not_found()?
            .clone();

        if collection_entry.display_name == new_name {
            return Ok(());
        }

        let new_encoded_name = encode_name(&new_name);
        let path = PathBuf::from(&COLLECTIONS_DIR).join(&new_encoded_name);
        let old_abs_path: Arc<Path> = collection_entry.path().clone().into();
        let new_abs_path: Arc<Path> = self.absolutize(path).into();
        if new_abs_path.exists() {
            return Err(OperationError::AlreadyExists {
                name: new_encoded_name,
                path: new_abs_path.to_path_buf(),
            });
        }

        // TODO: To perform a reset, we need a mutable reference.
        // For that, we have to dereference the `Arc`, but to do that, `CollectionEntry` needs to be `Clone`.
        // Right now, that’s not possible because of the `registry` field in the `Collection` type.
        // We’ll only be able to implement this after we remove that field.

        // collection_entry.reset(new_abs_path).await?;

        self.fs
            .rename(
                &old_abs_path,
                &new_abs_path,
                RenameOptions {
                    overwrite: false,
                    ignore_if_exists: false,
                },
            )
            .await?;

        let collection = Collection::new(
            new_abs_path.to_path_buf(), // FIXME: change to Arc<Path> in Collection::new
            self.fs.clone(),
            self.indexer_handle.clone(),
            self.next_collection_entry_id.clone(),
        )?;

        {
            let collection_store = self.workspace_storage.collection_store();
            let mut txn = self.workspace_storage.begin_write().await?;
            collection_store.rekey_collection(
                &mut txn,
                old_abs_path.to_owned().to_path_buf(), // FIXME: change to Arc<Path> in the store
                new_abs_path.to_owned().to_path_buf(), // FIXME: change to Arc<Path> in the store
            )?;
            txn.commit()?;
        }

        {
            let mut collections_lock = collections.write().await;
            collections_lock.insert(
                *id,
                CollectionEntry {
                    id: *id,
                    name: new_encoded_name,
                    display_name: new_name,
                    order: None,
                    inner: collection,
                }
                .into(),
            );
        }

        Ok(())
    }
}
