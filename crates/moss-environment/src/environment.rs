use moss_applib::{AppRuntime, providers::ServiceProvider};
use std::{path::Path, sync::Arc};

pub struct Environment<R: AppRuntime> {
    #[allow(dead_code)]
    abs_path: Arc<Path>,
    #[allow(dead_code)]
    services: ServiceProvider,

    _marker: std::marker::PhantomData<R>,
}

unsafe impl<R: AppRuntime> Send for Environment<R> {}
unsafe impl<R: AppRuntime> Sync for Environment<R> {}

impl<R: AppRuntime> Environment<R> {
    pub(super) fn new(abs_path: Arc<Path>, services: ServiceProvider) -> Self {
        Self {
            abs_path,
            services,
            _marker: std::marker::PhantomData,
        }
    }
}
