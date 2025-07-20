use moss_applib::{AppRuntime, ServiceMarker};
use moss_storage::common::VariableStore;
use std::sync::Arc;

pub struct StorageService<R: AppRuntime> {
    variable_store: Arc<dyn VariableStore<R::AsyncContext>>,
}

impl<R: AppRuntime> ServiceMarker for StorageService<R> {}

impl<R: AppRuntime> StorageService<R> {
    pub fn new(variable_store: Arc<dyn VariableStore<R::AsyncContext>>) -> Self {
        Self { variable_store }
    }
}
