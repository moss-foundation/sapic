use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use sapic_system::theme::theme_registry::ThemeRegistry;
use std::sync::Arc;

pub trait AsGlobal: Send + Sync + 'static {}
#[derive(derive_more::Deref, Clone)]
pub struct Global<T: ?Sized + AsGlobal>(pub Arc<T>);

impl<T: ?Sized + AsGlobal> Global<T> {
    pub fn get<R: AppRuntime>(d: &AppDelegate<R>) -> Arc<T> {
        d.global::<Global<T>>().0.clone()
    }
    pub fn set<R: AppRuntime>(d: &AppDelegate<R>, v: Arc<T>) {
        d.set_global(Global(v));
    }
}

impl AsGlobal for dyn ThemeRegistry {}

pub type GlobalThemeRegistry = Global<dyn ThemeRegistry>;
