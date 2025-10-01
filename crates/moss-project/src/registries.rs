pub mod http_headers;
pub mod resource_statuses;

use derive_more::Deref;
use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use resource_statuses::ResourceStatusRegistry;
use std::sync::Arc;

use crate::registries::http_headers::HttpHeaderRegistry;

#[derive(Deref, Clone)]
pub struct GlobalResourceStatusRegistry(Arc<dyn ResourceStatusRegistry>);

impl dyn ResourceStatusRegistry {
    pub fn global<R: AppRuntime>(delegate: &AppDelegate<R>) -> Arc<dyn ResourceStatusRegistry> {
        delegate.global::<GlobalResourceStatusRegistry>().0.clone()
    }

    pub fn set_global<R: AppRuntime>(
        delegate: &AppDelegate<R>,
        v: Arc<dyn ResourceStatusRegistry>,
    ) {
        delegate.set_global(GlobalResourceStatusRegistry(v));
    }
}

#[derive(Deref, Clone)]
pub struct GlobalHttpHeaderRegistry(Arc<dyn HttpHeaderRegistry>);

impl dyn HttpHeaderRegistry {
    pub fn global<R: AppRuntime>(delegate: &AppDelegate<R>) -> Arc<dyn HttpHeaderRegistry> {
        delegate.global::<GlobalHttpHeaderRegistry>().0.clone()
    }

    pub fn set_global<R: AppRuntime>(delegate: &AppDelegate<R>, v: Arc<dyn HttpHeaderRegistry>) {
        delegate.set_global(GlobalHttpHeaderRegistry(v));
    }
}
