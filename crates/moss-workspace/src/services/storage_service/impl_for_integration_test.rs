use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use async_trait::async_trait;
use moss_applib::{AppRuntime, ServiceMarker};
use moss_db::{DatabaseResult, Transaction, primitives::AnyValue};
use moss_storage::{WorkspaceStorage, primitives::segkey::SegKeyBuf};

use crate::{
    models::primitives::{ActivitybarPosition, CollectionId, SidebarPosition},
    services::{AnyStorageService, storage_service::StorageService},
    storage::entities::state_store::{EditorGridStateEntity, EditorPanelStateEntity},
};

pub struct StorageServiceForIntegrationTest<R: AppRuntime> {
    real: Arc<StorageService<R>>,
}

impl<R: AppRuntime> StorageServiceForIntegrationTest<R> {
    pub fn storage(&self) -> &Arc<dyn WorkspaceStorage<R::AsyncContext>> {
        &self.real.storage
    }

    pub fn real(&self) -> &Arc<StorageService<R>> {
        &self.real
    }
}

impl<R: AppRuntime> ServiceMarker for StorageServiceForIntegrationTest<R> {}

impl<R: AppRuntime> From<StorageService<R>> for StorageServiceForIntegrationTest<R> {
    fn from(real: StorageService<R>) -> Self {
        Self {
            real: Arc::new(real),
        }
    }
}

#[async_trait]
impl<R: AppRuntime> AnyStorageService<R> for StorageServiceForIntegrationTest<R> {
    async fn begin_write(&self, ctx: &R::AsyncContext) -> anyhow::Result<Transaction> {
        self.real.begin_write(ctx).await
    }

    async fn put_item_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        id: &str,
        order: usize,
    ) -> anyhow::Result<()> {
        self.real.put_item_order_txn(ctx, txn, id, order).await
    }

    async fn put_expanded_items_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        expanded_entries: &HashSet<CollectionId>,
    ) -> anyhow::Result<()> {
        self.real
            .put_expanded_items_txn(ctx, txn, expanded_entries)
            .await
    }

    async fn get_expanded_items(
        &self,
        ctx: &R::AsyncContext,
    ) -> anyhow::Result<HashSet<CollectionId>> {
        self.real.get_expanded_items(ctx).await
    }

    async fn remove_item_metadata_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        segkey_prefix: SegKeyBuf,
    ) -> DatabaseResult<()> {
        self.real
            .remove_item_metadata_txn(ctx, txn, segkey_prefix)
            .await
    }

    async fn list_items_metadata(
        &self,
        ctx: &R::AsyncContext,
        segkey_prefix: SegKeyBuf,
    ) -> DatabaseResult<HashMap<SegKeyBuf, AnyValue>> {
        self.real.list_items_metadata(ctx, segkey_prefix).await
    }

    async fn get_layout_cache(
        &self,
        ctx: &R::AsyncContext,
    ) -> anyhow::Result<HashMap<SegKeyBuf, AnyValue>> {
        self.real.get_layout_cache(ctx).await
    }

    async fn put_sidebar_layout(
        &self,
        ctx: &R::AsyncContext,
        position: SidebarPosition,
        size: usize,
        visible: bool,
    ) -> anyhow::Result<()> {
        self.real
            .put_sidebar_layout(ctx, position, size, visible)
            .await
    }

    async fn put_panel_layout(
        &self,
        ctx: &R::AsyncContext,
        size: usize,
        visible: bool,
    ) -> anyhow::Result<()> {
        self.real.put_panel_layout(ctx, size, visible).await
    }

    async fn put_activitybar_layout(
        &self,
        ctx: &R::AsyncContext,
        last_active_container_id: Option<String>,
        position: ActivitybarPosition,
    ) -> anyhow::Result<()> {
        self.real
            .put_activitybar_layout(ctx, last_active_container_id, position)
            .await
    }

    async fn put_editor_layout(
        &self,
        ctx: &R::AsyncContext,
        grid: EditorGridStateEntity,
        panels: HashMap<String, EditorPanelStateEntity>,
        active_group: Option<String>,
    ) -> anyhow::Result<()> {
        self.real
            .put_editor_layout(ctx, grid, panels, active_group)
            .await
    }
}
