use anyhow::{Context as _, Result};
use moss_applib::{AppRuntime, ServiceMarker};
use moss_db::{Transaction, primitives::AnyValue};
use moss_storage::{
    WorkspaceStorage,
    primitives::segkey::SegKeyBuf,
    storage::operations::{
        GetItem, ListByPrefix, TransactionalPutItem, TransactionalRemoveByPrefix,
    },
    workspace_storage::WorkspaceStorageImpl,
};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::Arc,
};

use crate::{
    models::primitives::{ActivitybarPosition, CollectionId, SidebarPosition},
    storage::{
        entities::state_store::{EditorGridStateEntity, EditorPanelStateEntity},
        segments::{self, SEGKEY_COLLECTION},
    },
};

pub struct StorageService<R: AppRuntime> {
    pub(super) storage: Arc<dyn WorkspaceStorage<R::AsyncContext>>,
}

impl<R: AppRuntime> ServiceMarker for StorageService<R> {}

impl<R: AppRuntime> StorageService<R> {
    pub(crate) fn new(abs_path: &Path) -> Result<Self> {
        let storage = WorkspaceStorageImpl::new(&abs_path)
            .context("Failed to load the workspace state database")?;

        Ok(Self {
            storage: Arc::new(storage),
        })
    }

    pub async fn begin_write(&self, ctx: &R::AsyncContext) -> joinerror::Result<Transaction> {
        Ok(self.storage.begin_write_with_context(ctx).await?)
    }

    // Items operations

    pub(super) async fn put_item_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        id: &str,
        order: isize,
    ) -> joinerror::Result<()> {
        let store = self.storage.item_store();

        let segkey = SEGKEY_COLLECTION.join(id.to_string()).join("order");
        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            txn,
            segkey,
            AnyValue::serialize(&order)?,
        )
        .await?;

        Ok(())
    }

    pub(super) async fn put_expanded_items_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        expanded_entries: &HashSet<CollectionId>,
    ) -> joinerror::Result<()> {
        let store = self.storage.item_store();
        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            txn,
            segments::SEGKEY_EXPANDED_ITEMS.to_segkey_buf(),
            AnyValue::serialize(&expanded_entries)?,
        )
        .await?;

        Ok(())
    }

    pub(super) async fn get_expanded_items(
        &self,
        ctx: &R::AsyncContext,
    ) -> joinerror::Result<HashSet<CollectionId>> {
        let store = self.storage.item_store();
        let segkey = segments::SEGKEY_EXPANDED_ITEMS.to_segkey_buf();
        let value = GetItem::get(store.as_ref(), ctx, segkey).await?;
        Ok(AnyValue::deserialize::<HashSet<_>>(&value)?)
    }

    pub(super) async fn list_items_metadata(
        &self,
        ctx: &R::AsyncContext,
        segkey_prefix: SegKeyBuf,
    ) -> joinerror::Result<HashMap<SegKeyBuf, AnyValue>> {
        let data = ListByPrefix::list_by_prefix(
            self.storage.item_store().as_ref(),
            ctx,
            segkey_prefix.to_string().as_str(),
        )
        .await?;

        Ok(data.into_iter().collect())
    }

    pub(super) async fn remove_item_metadata_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        segkey_prefix: SegKeyBuf,
    ) -> joinerror::Result<()> {
        TransactionalRemoveByPrefix::remove_by_prefix(
            self.storage.item_store().as_ref(),
            ctx,
            txn,
            segkey_prefix.to_string().as_str(),
        )
        .await?;

        Ok(())
    }

    // Layout operations

    pub(crate) async fn get_layout_cache(
        &self,
        ctx: &R::AsyncContext,
    ) -> joinerror::Result<HashMap<SegKeyBuf, AnyValue>> {
        let store = self.storage.item_store();
        let segkey = segments::SEGKEY_LAYOUT.to_segkey_buf();
        let value =
            ListByPrefix::list_by_prefix(store.as_ref(), ctx, segkey.to_string().as_str()).await?;
        Ok(value.into_iter().collect())
    }

    pub(super) async fn put_sidebar_layout(
        &self,
        ctx: &R::AsyncContext,
        position: SidebarPosition,
        size: usize,
        visible: bool,
    ) -> joinerror::Result<()> {
        let store = self.storage.item_store();
        let mut txn = self.begin_write(ctx).await?;

        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            &mut txn,
            segments::SEGKEY_LAYOUT_SIDEBAR.join("position"),
            AnyValue::serialize(&position)?,
        )
        .await?;

        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            &mut txn,
            segments::SEGKEY_LAYOUT_SIDEBAR.join("size"),
            AnyValue::serialize(&size)?,
        )
        .await?;

        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            &mut txn,
            segments::SEGKEY_LAYOUT_SIDEBAR.join("visible"),
            AnyValue::serialize(&visible)?,
        )
        .await?;

        Ok(txn.commit()?)
    }

    pub(super) async fn put_panel_layout(
        &self,
        ctx: &R::AsyncContext,
        size: usize,
        visible: bool,
    ) -> joinerror::Result<()> {
        let store = self.storage.item_store();
        let mut txn = self.begin_write(ctx).await?;

        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            &mut txn,
            segments::SEGKEY_LAYOUT_PANEL.join("size"),
            AnyValue::serialize(&size)?,
        )
        .await?;

        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            &mut txn,
            segments::SEGKEY_LAYOUT_PANEL.join("visible"),
            AnyValue::serialize(&visible)?,
        )
        .await?;

        Ok(txn.commit()?)
    }

    pub(super) async fn put_activitybar_layout(
        &self,
        ctx: &R::AsyncContext,
        last_active_container_id: Option<String>,
        position: ActivitybarPosition,
    ) -> joinerror::Result<()> {
        let store = self.storage.item_store();
        let mut txn = self.begin_write(ctx).await?;

        if let Some(last_active_container_id) = last_active_container_id {
            TransactionalPutItem::put_with_context(
                store.as_ref(),
                ctx,
                &mut txn,
                segments::SEGKEY_LAYOUT_ACTIVITYBAR.join("lastActiveContainerId"),
                AnyValue::serialize(&last_active_container_id)?,
            )
            .await?;
        }

        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            &mut txn,
            segments::SEGKEY_LAYOUT_ACTIVITYBAR.join("position"),
            AnyValue::serialize(&position)?,
        )
        .await?;

        Ok(txn.commit()?)
    }

    pub(super) async fn put_editor_layout(
        &self,
        ctx: &R::AsyncContext,
        grid: EditorGridStateEntity,
        panels: HashMap<String, EditorPanelStateEntity>,
        active_group: Option<String>,
    ) -> joinerror::Result<()> {
        let store = self.storage.item_store();
        let mut txn = self.begin_write(ctx).await?;

        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            &mut txn,
            segments::SEGKEY_LAYOUT_EDITOR.join("grid"),
            AnyValue::serialize(&grid)?,
        )
        .await?;

        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            &mut txn,
            segments::SEGKEY_LAYOUT_EDITOR.join("panels"),
            AnyValue::serialize(&panels)?,
        )
        .await?;

        if let Some(active_group) = active_group {
            TransactionalPutItem::put_with_context(
                store.as_ref(),
                ctx,
                &mut txn,
                segments::SEGKEY_LAYOUT_EDITOR.join("activeGroup"),
                AnyValue::serialize(&active_group)?,
            )
            .await?;
        }

        Ok(txn.commit()?)
    }
}

#[cfg(any(test, feature = "integration-tests"))]
impl<R: AppRuntime> StorageService<R> {
    pub fn storage(&self) -> &Arc<dyn WorkspaceStorage<R::AsyncContext>> {
        &self.storage
    }
}
