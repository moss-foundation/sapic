pub mod collection_service;
pub mod environment_service;
pub mod layout_service;
pub mod storage_service;

use anyhow::Result;
use async_trait::async_trait;
use derive_more::Deref;
use futures::Stream;
use moss_applib::{PublicServiceMarker, ServiceMarker};
use moss_collection::Collection as CollectionHandle;
use moss_db::{DatabaseResult, common::Transaction, primitives::AnyValue};
use moss_storage::primitives::segkey::SegKeyBuf;
use std::{
    collections::{HashMap, HashSet},
    pin::Pin,
    sync::Arc,
};

use crate::{
    models::{primitives::*, types::*},
    services::collection_service::{
        CollectionItemCreateParams, CollectionItemDescription, CollectionItemUpdateParams,
        CollectionResult,
    },
    storage::entities::state_store::*,
};

// ########################################################
// ###                Storage Service                   ###
// ########################################################

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

    fn put_sidebar_layout(
        &self,
        position: SidebarPosition,
        size: usize,
        visible: bool,
    ) -> Result<()>;

    fn put_panel_layout(&self, size: usize, visible: bool) -> Result<()>;

    fn put_activitybar_layout(
        &self,
        last_active_container_id: Option<String>,
        position: ActivitybarPosition,
    ) -> Result<()>;

    fn put_editor_layout(
        &self,
        grid: EditorGridStateEntity,
        panels: HashMap<String, EditorPanelStateEntity>,
        active_group: Option<String>,
    ) -> Result<()>;
}

#[derive(Deref)]
pub struct DynStorageService(Arc<dyn AnyStorageService>);

impl DynStorageService {
    pub fn new(service: Arc<dyn AnyStorageService>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl ServiceMarker for DynStorageService {}

// ########################################################
// ###                Layout Service                    ###
// ########################################################

pub trait AnyLayoutService: Send + Sync + ServiceMarker + 'static {
    fn put_sidebar_layout_state(&self, state: SidebarPartStateInfo) -> Result<()>;
    fn put_panel_layout_state(&self, state: PanelPartStateInfo) -> Result<()>;
    fn put_activitybar_layout_state(&self, state: ActivitybarPartStateInfo) -> Result<()>;
    fn put_editor_layout_state(&self, state: EditorPartStateInfo) -> Result<()>;

    fn get_sidebar_layout_state(
        &self,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<SidebarPartStateInfo>;
    fn get_panel_layout_state(
        &self,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<PanelPartStateInfo>;
    fn get_activitybar_layout_state(
        &self,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<ActivitybarPartStateInfo>;
    fn get_editor_layout_state(
        &self,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<Option<EditorPartStateInfo>>;
}

#[derive(Deref)]
pub struct DynLayoutService(Arc<dyn AnyLayoutService>);

impl DynLayoutService {
    pub fn new(service: Arc<dyn AnyLayoutService>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl ServiceMarker for DynLayoutService {}

// ########################################################
// ###               Collection Service                 ###
// ########################################################

#[async_trait]
pub trait AnyCollectionService: Send + Sync + ServiceMarker + 'static {
    async fn collection(&self, id: &CollectionId) -> CollectionResult<Arc<CollectionHandle>>;

    #[allow(private_interfaces)]
    async fn create_collection(
        &self,
        id: &CollectionId,
        params: CollectionItemCreateParams,
    ) -> CollectionResult<CollectionItemDescription>;

    #[allow(private_interfaces)]
    async fn delete_collection(
        &self,
        id: &CollectionId,
    ) -> CollectionResult<Option<CollectionItemDescription>>;

    #[allow(private_interfaces)]
    async fn update_collection(
        &self,
        id: &CollectionId,
        params: CollectionItemUpdateParams,
    ) -> CollectionResult<()>;

    #[allow(private_interfaces)]
    fn list_collections(
        &self,
    ) -> Pin<Box<dyn Stream<Item = CollectionItemDescription> + Send + '_>>;
}

#[derive(Deref)]
pub struct DynCollectionService(Arc<dyn AnyCollectionService>);

impl DynCollectionService {
    pub fn new(service: Arc<dyn AnyCollectionService>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl ServiceMarker for DynCollectionService {}
impl PublicServiceMarker for DynCollectionService {}

// ########################################################
// ###               Environment Service                ###
// ########################################################

pub trait AnyEnvironmentService: Send + Sync + ServiceMarker + 'static {
    fn get_environments(&self) -> Result<Vec<EnvironmentInfo>>;
}

#[derive(Deref)]
pub struct DynEnvironmentService(Arc<dyn AnyEnvironmentService>);

impl DynEnvironmentService {
    pub fn new(service: Arc<dyn AnyEnvironmentService>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl ServiceMarker for DynEnvironmentService {}
