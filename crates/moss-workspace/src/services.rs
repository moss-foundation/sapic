pub mod collection_service;
pub mod environment_service;
pub mod layout_service;
pub mod storage_service;

use anyhow::Result;
use async_trait::async_trait;
use derive_more::Deref;
use futures::Stream;
use moss_applib::{AppRuntime, PublicServiceMarker, ServiceMarker};
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

#[async_trait]
pub trait AnyStorageService<R: AppRuntime>: Send + Sync + ServiceMarker + 'static {
    async fn begin_write(&self, ctx: &R::AsyncContext) -> Result<Transaction>;
    async fn put_item_order_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        id: &str,
        order: usize,
    ) -> Result<()>;
    async fn put_expanded_items_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        expanded_entries: &HashSet<CollectionId>,
    ) -> Result<()>;
    async fn get_expanded_items(&self, ctx: &R::AsyncContext) -> Result<HashSet<CollectionId>>;
    async fn remove_item_metadata_txn(
        &self,
        ctx: &R::AsyncContext,
        txn: &mut Transaction,
        segkey_prefix: SegKeyBuf,
    ) -> DatabaseResult<()>;
    async fn list_items_metadata(
        &self,
        ctx: &R::AsyncContext,
        segkey_prefix: SegKeyBuf,
    ) -> DatabaseResult<HashMap<SegKeyBuf, AnyValue>>;
    async fn get_layout_cache(&self, ctx: &R::AsyncContext)
    -> Result<HashMap<SegKeyBuf, AnyValue>>;

    async fn put_sidebar_layout(
        &self,
        ctx: &R::AsyncContext,
        position: SidebarPosition,
        size: usize,
        visible: bool,
    ) -> Result<()>;

    async fn put_panel_layout(
        &self,
        ctx: &R::AsyncContext,
        size: usize,
        visible: bool,
    ) -> Result<()>;

    async fn put_activitybar_layout(
        &self,
        ctx: &R::AsyncContext,
        last_active_container_id: Option<String>,
        position: ActivitybarPosition,
    ) -> Result<()>;

    async fn put_editor_layout(
        &self,
        ctx: &R::AsyncContext,
        grid: EditorGridStateEntity,
        panels: HashMap<String, EditorPanelStateEntity>,
        active_group: Option<String>,
    ) -> Result<()>;
}

#[derive(Deref)]
pub struct DynStorageService<R: AppRuntime>(Arc<dyn AnyStorageService<R>>);

impl<R: AppRuntime> DynStorageService<R> {
    pub fn new(service: Arc<dyn AnyStorageService<R>>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl<R: AppRuntime> ServiceMarker for DynStorageService<R> {}
impl<R: AppRuntime> PublicServiceMarker for DynStorageService<R> {}

// ########################################################
// ###                Layout Service                    ###
// ########################################################

#[async_trait]
pub trait AnyLayoutService<R: AppRuntime>: Send + Sync + ServiceMarker + 'static {
    async fn put_sidebar_layout_state(
        &self,
        ctx: &R::AsyncContext,
        state: SidebarPartStateInfo,
    ) -> Result<()>;
    async fn put_panel_layout_state(
        &self,
        ctx: &R::AsyncContext,
        state: PanelPartStateInfo,
    ) -> Result<()>;
    async fn put_activitybar_layout_state(
        &self,
        ctx: &R::AsyncContext,
        state: ActivitybarPartStateInfo,
    ) -> Result<()>;
    async fn put_editor_layout_state(
        &self,
        ctx: &R::AsyncContext,
        state: EditorPartStateInfo,
    ) -> Result<()>;

    async fn get_sidebar_layout_state(
        &self,
        ctx: &R::AsyncContext,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<SidebarPartStateInfo>;
    async fn get_panel_layout_state(
        &self,
        ctx: &R::AsyncContext,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<PanelPartStateInfo>;
    async fn get_activitybar_layout_state(
        &self,
        ctx: &R::AsyncContext,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<ActivitybarPartStateInfo>;
    async fn get_editor_layout_state(
        &self,
        ctx: &R::AsyncContext,
        cache: &mut HashMap<SegKeyBuf, AnyValue>,
    ) -> Result<Option<EditorPartStateInfo>>;
}

#[derive(Deref)]
pub struct DynLayoutService<R: AppRuntime>(Arc<dyn AnyLayoutService<R>>);

impl<R: AppRuntime> DynLayoutService<R> {
    pub fn new(service: Arc<dyn AnyLayoutService<R>>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl<R: AppRuntime> ServiceMarker for DynLayoutService<R> {}

// ########################################################
// ###               Collection Service                 ###
// ########################################################

#[async_trait]
pub trait AnyCollectionService<R: AppRuntime>: Send + Sync + ServiceMarker + 'static {
    async fn collection(&self, id: &CollectionId) -> CollectionResult<Arc<CollectionHandle<R>>>;

    #[allow(private_interfaces)]
    async fn create_collection(
        &self,
        ctx: &R::AsyncContext,
        id: &CollectionId,
        params: CollectionItemCreateParams,
    ) -> CollectionResult<CollectionItemDescription>;

    #[allow(private_interfaces)]
    async fn delete_collection(
        &self,
        ctx: &R::AsyncContext,
        id: &CollectionId,
    ) -> CollectionResult<Option<CollectionItemDescription>>;

    #[allow(private_interfaces)]
    async fn update_collection(
        &self,
        ctx: &R::AsyncContext,
        id: &CollectionId,
        params: CollectionItemUpdateParams,
    ) -> CollectionResult<()>;

    #[allow(private_interfaces)]
    async fn list_collections(
        &self,
        ctx: &R::AsyncContext,
    ) -> Pin<Box<dyn Stream<Item = CollectionItemDescription> + Send + '_>>;
}

#[derive(Deref)]
pub struct DynCollectionService<R: AppRuntime>(Arc<dyn AnyCollectionService<R>>);

impl<R: AppRuntime> DynCollectionService<R> {
    pub fn new(service: Arc<dyn AnyCollectionService<R>>) -> Arc<Self> {
        Arc::new(Self(service))
    }
}

impl<R: AppRuntime> ServiceMarker for DynCollectionService<R> {}
impl<R: AppRuntime> PublicServiceMarker for DynCollectionService<R> {}

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
