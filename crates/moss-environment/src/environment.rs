use moss_applib::{AppRuntime, providers::ServiceProvider};

use std::{marker::PhantomData, path::Path, sync::Arc};

use crate::{
    AnyEnvironment, ModifyEnvironmentParams,
    services::{
        metadata_service::MetadataService, storage_service::StorageService,
        sync_service::SyncService, variable_service::VariableService, *,
    },
};

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
        let sync_service = self.services.get::<Self::SyncService>();
        let variable_service = self.services.get::<Self::VariableService>();

        variable_service.batch_add(params.vars_to_add).await?;
        variable_service.batch_remove(params.vars_to_delete).await?;

        <Self::SyncService as AnySyncService<R>>::save(sync_service).await?;

        Ok(())
    }
}
