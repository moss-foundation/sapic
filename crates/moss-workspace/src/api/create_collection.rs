use anyhow::Context as _;
use moss_collection::collection::{self, Collection};
use moss_common::api::{OperationError, OperationResult};
use moss_db::primitives::AnyValue;
use moss_fs::FileSystem;
use moss_storage::storage::operations::PutItem;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tauri::Runtime as TauriRuntime;
use tokio::sync::RwLock;
use uuid::Uuid;
use validator::Validate;

use crate::{
    context::{AnyWorkspaceContext, Subscribe},
    dirs,
    models::operations::{CreateCollectionInput, CreateCollectionOutput},
    storage::{entities::collection_store::CollectionCacheEntity, segments::COLLECTION_SEGKEY},
    workspace::{CollectionItem, Workspace},
};

impl<R: TauriRuntime> Workspace<R> {
    pub async fn create_collection<C: AnyWorkspaceContext<R>>(
        &mut self,
        ctx: &C,
        input: &CreateCollectionInput,
    ) -> OperationResult<CreateCollectionOutput> {
        input.validate()?;

        let id = Uuid::new_v4();
        let id_str = id.to_string();

        let path = PathBuf::from(dirs::COLLECTIONS_DIR).join(&id_str);
        let abs_path: Arc<Path> = self.absolutize(path).into();

        if abs_path.exists() {
            return Err(OperationError::AlreadyExists(
                abs_path.to_string_lossy().to_string(),
            ));
        }

        let fs = <dyn FileSystem>::global::<R, C>(ctx);
        let collections = self
            .collections_mut(ctx)
            .await
            .context("Failed to get collections")?;

        fs.create_dir(&abs_path)
            .await
            .context("Failed to create the collection directory")?;

        let order = input.order.to_owned();
        let collection = Collection::create(
            fs.clone(),
            collection::CreateParams {
                name: Some(input.name.to_owned()),
                internal_abs_path: &abs_path,
                external_abs_path: input.external_path.as_deref(),
                repository: input.repo.to_owned(),
                icon_path: input.icon_path.to_owned(),
            },
        )
        .await
        .map_err(|e| OperationError::Internal(e.to_string()))?;

        let on_did_change = collection.on_did_change().subscribe(|_event| async move {

            // TODO: Save in the database whether the collection was collapsed/expanded
        });
        ctx.subscribe(Subscribe::OnCollectionDidChange(id, on_did_change))
            .await;

        collections.insert(
            id,
            Arc::new(RwLock::new(CollectionItem {
                id,
                order: order.clone(),
                inner: collection,
            })),
        );

        {
            let key = COLLECTION_SEGKEY.join(&id_str);
            let value = AnyValue::serialize(&CollectionCacheEntity {
                order: order.clone(),
                external_abs_path: None,
            })?;
            PutItem::put(self.storage.item_store().as_ref(), key, value)?;
        }

        Ok(CreateCollectionOutput { id, abs_path })
    }
}
