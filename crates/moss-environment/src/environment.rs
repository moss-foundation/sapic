use moss_applib::{AppRuntime, providers::ServiceProvider};
use moss_bindingutils::primitives::ChangeString;
use std::{marker::PhantomData, path::Path, sync::Arc};

use crate::{
    models::{
        primitives::VariableId,
        types::{AddVariableParams, UpdateVariableParams},
    },
    services::{
        metadata_service::MetadataService, storage_service::StorageService,
        sync_service::SyncService, variable_service::VariableService, *,
    },
};

pub struct ModifyEnvironmentParams {
    pub color: Option<ChangeString>,
    pub vars_to_add: Vec<AddVariableParams>,
    pub vars_to_update: Vec<UpdateVariableParams>,
    pub vars_to_delete: Vec<VariableId>,
}

pub trait AnyEnvironment<R: AppRuntime> {
    type StorageService: AnyStorageService<R>;
    type VariableService: AnyVariableService<R>;
    type SyncService: AnySyncService<R>;
    type MetadataService: AnyMetadataService<R>;

    async fn modify(&self, params: ModifyEnvironmentParams) -> joinerror::Result<()>;
}

pub struct Environment<R: AppRuntime> {
    #[allow(dead_code)]
    abs_path: Arc<Path>,
    #[allow(dead_code)]
    services: ServiceProvider,

    _marker: PhantomData<R>,
}

unsafe impl<R: AppRuntime> Send for Environment<R> {}
unsafe impl<R: AppRuntime> Sync for Environment<R> {}

impl<R: AppRuntime> Environment<R> {
    pub(super) fn new(abs_path: Arc<Path>, services: ServiceProvider) -> Self {
        Self {
            abs_path,
            services,
            _marker: PhantomData,
        }
    }
}

impl<R: AppRuntime> AnyEnvironment<R> for Environment<R> {
    type StorageService = StorageService<R>;
    type SyncService = SyncService;
    type MetadataService = MetadataService;
    type VariableService = VariableService<R, Self::StorageService, Self::SyncService>;

    async fn modify(&self, params: ModifyEnvironmentParams) -> joinerror::Result<()> {
        let variable_service = self.services.get::<Self::VariableService>();

        Ok(())
    }
}
