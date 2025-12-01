use moss_app_delegate::AppDelegate;
use moss_applib::AppRuntime;
use moss_storage2::KvStorage;
use sapic_system::{
    configuration::configuration_registry::ConfigurationRegistry, language::LanguagePackRegistry,
    theme::ThemeRegistry,
};
use std::sync::Arc;

use crate::app::settings_storage::SettingsStorage;

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
impl AsGlobal for dyn SettingsStorage {}
impl AsGlobal for dyn KvStorage {}
impl AsGlobal for dyn ThemeRegistry {}
impl AsGlobal for dyn LanguagePackRegistry {}
impl AsGlobal for dyn ConfigurationRegistry {}

pub type GlobalSettingsStorage = Global<dyn SettingsStorage>;
pub type GlobalKvStorage = Global<dyn KvStorage>;
pub type GlobalThemeRegistry = Global<dyn ThemeRegistry>;
pub type GlobalConfigurationRegistry = Global<dyn ConfigurationRegistry>;
pub type GlobalLanguagePackRegistry = Global<dyn LanguagePackRegistry>;
