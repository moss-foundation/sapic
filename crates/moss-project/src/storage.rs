pub mod entities;
pub mod segments;

use joinerror::Result;
use moss_applib::AppRuntime;
use moss_db::{DbResultExt, Transaction, primitives::AnyValue};
use moss_storage::{
    CollectionStorage,
    collection_storage::CollectionStorageImpl,
    primitives::segkey::SegKeyBuf,
    storage::operations::{GetItem, ListByPrefix, TransactionalPutItem, TransactionalRemoveItem},
};
use std::{collections::HashMap, path::Path, sync::Arc};

use crate::models::primitives::{
    EntryId, FormDataParamId, HeaderId, PathParamId, QueryParamId, UrlencodedParamId,
};

pub struct StorageService<R: AppRuntime> {
    storage: Arc<dyn CollectionStorage<R::AsyncContext>>,
}

impl<R: AppRuntime> StorageService<R> {
    pub async fn begin_write(&self, ctx: &R::AsyncContext) -> Result<Transaction> {
        Ok(self.storage.begin_write_with_context(ctx).await?)
    }

    #[allow(dead_code)]
    pub async fn begin_read(&self, ctx: &R::AsyncContext) -> Result<Transaction> {
        Ok(self.storage.begin_read_with_context(ctx).await?)
    }

    pub async fn put_entry_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        id: &EntryId,
        order: isize,
    ) -> Result<()> {
        let store = self.storage.resource_store();

        let segkey = segments::segkey_entry_order(&id);
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

    pub async fn put_entry_header_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        entry_id: &EntryId,
        header_id: &HeaderId,
        order: isize,
    ) -> Result<()> {
        let store = self.storage.resource_store();

        let segkey = segments::segkey_entry_header_order(entry_id, header_id);
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

    pub async fn remove_entry_header_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        entry_id: &EntryId,
        header_id: &HeaderId,
    ) -> Result<()> {
        let store = self.storage.resource_store();
        let segkey = segments::segkey_entry_header_order(entry_id, header_id);
        TransactionalRemoveItem::remove(store.as_ref(), ctx, txn, segkey).await?;

        Ok(())
    }

    pub async fn put_entry_path_param_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        entry_id: &EntryId,
        path_param_id: &PathParamId,
        order: isize,
    ) -> Result<()> {
        let store = self.storage.resource_store();

        let segkey = segments::segkey_entry_path_param_order(entry_id, path_param_id);
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

    pub async fn remove_entry_path_param_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        entry_id: &EntryId,
        path_param_id: &PathParamId,
    ) -> Result<()> {
        let store = self.storage.resource_store();
        let segkey = segments::segkey_entry_path_param_order(entry_id, path_param_id);
        TransactionalRemoveItem::remove(store.as_ref(), ctx, txn, segkey).await?;

        Ok(())
    }

    pub async fn put_entry_query_param_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        entry_id: &EntryId,
        query_param_id: &QueryParamId,
        order: isize,
    ) -> Result<()> {
        let store = self.storage.resource_store();

        let segkey = segments::segkey_entry_query_param_order(entry_id, query_param_id);
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

    pub async fn remove_entry_query_param_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        entry_id: &EntryId,
        query_param_id: &QueryParamId,
    ) -> Result<()> {
        let store = self.storage.resource_store();
        let segkey = segments::segkey_entry_query_param_order(entry_id, query_param_id);
        TransactionalRemoveItem::remove(store.as_ref(), ctx, txn, segkey).await?;

        Ok(())
    }

    pub async fn put_entry_body_urlencoded_param_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        entry_id: &EntryId,
        param_id: &UrlencodedParamId,
        order: isize,
    ) -> Result<()> {
        let store = self.storage.resource_store();

        let segkey = segments::segkey_entry_body_urlencoded_param_order(entry_id, param_id);
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

    pub async fn put_entry_body_formdata_param_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        entry_id: &EntryId,
        param_id: &FormDataParamId,
        order: isize,
    ) -> Result<()> {
        let store = self.storage.resource_store();

        let segkey = segments::segkey_entry_body_formdata_param_order(entry_id, param_id);
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

    pub async fn get_all_entry_keys(
        &self,
        ctx: &R::AsyncContext,
    ) -> Result<HashMap<SegKeyBuf, AnyValue>> {
        let store = self.storage.resource_store();
        let value = ListByPrefix::list_by_prefix(
            store.as_ref(),
            ctx,
            &segments::SEGKEY_RESOURCE_ENTRY.to_segkey_buf().to_string(),
        )
        .await?;

        Ok(value.into_iter().collect())
    }

    pub async fn get_entry_keys(
        &self,
        ctx: &R::AsyncContext,
        id: &EntryId,
    ) -> Result<HashMap<SegKeyBuf, AnyValue>> {
        let store = self.storage.resource_store();
        let value = ListByPrefix::list_by_prefix(
            store.as_ref(),
            ctx,
            &segments::SEGKEY_RESOURCE_ENTRY.join(id).to_string(),
        )
        .await?;

        Ok(value.into_iter().collect())
    }

    pub async fn put_expanded_entries(
        &self,
        ctx: &R::AsyncContext,
        expanded_entries: Vec<EntryId>,
    ) -> Result<()> {
        let mut txn = self.begin_write(ctx).await?;
        self.put_expanded_entries_txn(ctx, &mut txn, expanded_entries)
            .await?;
        txn.commit()?;

        Ok(())
    }

    pub async fn put_expanded_entries_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        expanded_entries: Vec<EntryId>,
    ) -> Result<()> {
        let store = self.storage.resource_store();
        TransactionalPutItem::put_with_context(
            store.as_ref(),
            ctx,
            txn,
            segments::SEGKEY_EXPANDED_ENTRIES.to_segkey_buf(),
            AnyValue::serialize(&expanded_entries)?,
        )
        .await?;

        Ok(())
    }

    pub async fn get_expanded_entries(&self, ctx: &R::AsyncContext) -> Result<Vec<EntryId>> {
        let store = self.storage.resource_store();
        let segkey = segments::SEGKEY_EXPANDED_ENTRIES.to_segkey_buf();
        let value = GetItem::get(store.as_ref(), ctx, segkey).await?;
        Ok(AnyValue::deserialize::<Vec<EntryId>>(&value)?)
    }
}

impl<R: AppRuntime> StorageService<R> {
    pub fn new(abs_path: &Path) -> Result<Self> {
        debug_assert!(abs_path.is_absolute());

        let storage = CollectionStorageImpl::new(&abs_path).join_err_with::<()>(|| {
            format!(
                "Failed to open the collection {} state database",
                abs_path.display()
            )
        })?;

        Ok(Self {
            storage: Arc::new(storage),
        })
    }
}

#[cfg(any(test, feature = "integration-tests"))]
impl<R: AppRuntime> StorageService<R> {
    pub fn storage(&self) -> &Arc<dyn CollectionStorage<R::AsyncContext>> {
        &self.storage
    }
}
