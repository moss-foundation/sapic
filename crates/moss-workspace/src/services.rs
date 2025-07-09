pub mod collection_service;
pub mod environment_service;
pub mod layout_service;
pub mod storage_service;

use anyhow::Result;
use derive_more::Deref;
use moss_applib::ServiceMarker;
use moss_db::{DatabaseResult, common::Transaction, primitives::AnyValue};
use moss_storage::primitives::segkey::SegKeyBuf;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::models::primitives::CollectionId;

// FIXME: The result types are a bit mixed right now,
// but I think we'll fix that when we switch to using the joinerror library.

pub trait AnyStorageService: Send + Sync + ServiceMarker + 'static {
    fn begin_write(&self) -> Result<Transaction>;
    fn put_item_order_txn(&self, txn: &mut Transaction, id: &str, order: usize) -> Result<()>;
    fn put_expanded_items_txn(
        &self,
        txn: &mut Transaction,
        expanded_entries: &HashSet<CollectionId>,
    ) -> Result<()>;
    fn get_expanded_items(&self) -> Result<HashSet<CollectionId>>;
    fn remove_item_metadata_txn(
        &self,
        txn: &mut Transaction,
        segkey_prefix: SegKeyBuf,
    ) -> DatabaseResult<()>;
    fn list_items_metadata(
        &self,
        segkey_prefix: SegKeyBuf,
    ) -> DatabaseResult<HashMap<SegKeyBuf, AnyValue>>;
    fn get_layout_cache(&self) -> Result<HashMap<SegKeyBuf, AnyValue>>;
}

#[derive(Deref)]
pub struct DynStorageService(Arc<dyn AnyStorageService>);

impl DynStorageService {
    pub fn new(service: Arc<dyn AnyStorageService>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl ServiceMarker for DynStorageService {}
