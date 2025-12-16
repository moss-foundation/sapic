use derive_more::Deref;
use moss_activity_broadcaster::{AppActivityBroadcaster, ToLocation, handle::ActivityHandle};
use moss_applib::AppRuntime;
use std::path::PathBuf;
use tauri::{AppHandle as TauriAppHandle, Manager};

pub mod broadcast {
    pub use moss_activity_broadcaster::ToLocation;
}

/// A wrapper around `tauri::AppHandle` that provides
/// access to the global state and actions.
#[derive(Deref)]
pub struct AppDelegate<R: AppRuntime> {
    #[deref]
    tao_handle: TauriAppHandle<R::EventLoop>,
    broadcaster: AppActivityBroadcaster<R::EventLoop>,
}

impl<R: AppRuntime> AppDelegate<R> {
    pub fn new(app_handle: TauriAppHandle<R::EventLoop>) -> Self {
        Self {
            tao_handle: app_handle.clone(),
            broadcaster: AppActivityBroadcaster::new(app_handle),
        }
    }

    pub fn handle(&self) -> TauriAppHandle<R::EventLoop> {
        self.tao_handle.clone()
    }

    #[cfg(debug_assertions)]
    #[cfg(not(feature = "integration-tests"))]
    pub fn resource_dir(&self) -> PathBuf {
        PathBuf::from(std::env::var("DEV_RESOURCE_DIR").expect("DEV_RESOURCE_DIR is not set"))
    }

    #[cfg(not(debug_assertions))]
    #[cfg(not(feature = "integration-tests"))]
    pub fn resource_dir(&self) -> PathBuf {
        self.tao_handle
            .path()
            .resource_dir()
            .expect("Cannot resolve resource dir")
    }

    #[cfg(debug_assertions)]
    #[cfg(not(feature = "integration-tests"))]
    pub fn user_dir(&self) -> PathBuf {
        PathBuf::from(std::env::var("DEV_USER_DIR").expect("DEV_USER_DIR is not set"))
    }

    #[cfg(not(debug_assertions))]
    #[cfg(not(feature = "integration-tests"))]
    pub fn user_dir(&self) -> PathBuf {
        self.tao_handle
            .path()
            .app_data_dir()
            .expect("Cannot resolve user dir")
    }

    #[cfg(debug_assertions)]
    #[cfg(not(feature = "integration-tests"))]
    pub fn user_extensions_dir(&self) -> PathBuf {
        self.user_dir().join("extensions")
    }

    #[cfg(not(debug_assertions))]
    #[cfg(not(feature = "integration-tests"))]
    pub fn user_extensions_dir(&self) -> PathBuf {
        self.user_dir().join("extensions")
    }

    #[cfg(debug_assertions)]
    #[cfg(not(feature = "integration-tests"))]
    pub fn tmp_dir(&self) -> PathBuf {
        self.user_dir().join("tmp")
    }

    #[cfg(not(debug_assertions))]
    #[cfg(not(feature = "integration-tests"))]
    pub fn tmp_dir(&self) -> PathBuf {
        self.tao_handle
            .path()
            .temp_dir()
            .expect("Cannot resolve tmp dir")
    }

    #[cfg(debug_assertions)]
    #[cfg(not(feature = "integration-tests"))]
    pub fn logs_dir(&self) -> PathBuf {
        self.user_dir().join("logs")
    }

    #[cfg(not(debug_assertions))]
    #[cfg(not(feature = "integration-tests"))]
    pub fn logs_dir(&self) -> PathBuf {
        self.user_dir().join("logs")
    }

    #[cfg(debug_assertions)]
    #[cfg(not(feature = "integration-tests"))]
    pub fn globals_dir(&self) -> PathBuf {
        self.user_dir().join("globals")
    }

    #[cfg(not(debug_assertions))]
    #[cfg(not(feature = "integration-tests"))]
    pub fn globals_dir(&self) -> PathBuf {
        self.user_dir().join("globals")
    }

    #[cfg(debug_assertions)]
    #[cfg(not(feature = "integration-tests"))]
    pub fn workspaces_dir(&self) -> PathBuf {
        self.user_dir().join("workspaces")
    }

    #[cfg(not(debug_assertions))]
    #[cfg(not(feature = "integration-tests"))]
    pub fn workspaces_dir(&self) -> PathBuf {
        self.user_dir().join("workspaces")
    }

    pub fn global<T>(&self) -> &T
    where
        T: Send + Sync + 'static,
    {
        self.tao_handle.state::<T>().inner()
    }

    pub fn set_global<T>(&self, value: T)
    where
        T: Send + Sync + 'static,
    {
        self.tao_handle.manage(value);
    }

    pub fn emit_oneshot(&self, to: ToLocation<'_>) -> joinerror::Result<()> {
        self.broadcaster.emit_oneshot(to)
    }

    pub fn emit_continual<'a>(
        &'a self,
        to: ToLocation<'a>,
    ) -> joinerror::Result<ActivityHandle<'a, R::EventLoop>> {
        self.broadcaster.emit_continual(to)
    }
}

impl<R: AppRuntime> Clone for AppDelegate<R> {
    fn clone(&self) -> Self {
        Self {
            tao_handle: self.tao_handle.clone(),
            broadcaster: self.broadcaster.clone(),
        }
    }
}

#[cfg(feature = "integration-tests")]
pub mod test {
    use std::path::PathBuf;

    pub struct ResourceDir(pub PathBuf);
    pub struct UserDir(pub PathBuf);
}

impl<R: AppRuntime> AppDelegate<R> {
    #[cfg(feature = "integration-tests")]
    pub fn set_resource_dir(&self, resource_dir: PathBuf) {
        self.tao_handle.manage(test::ResourceDir(resource_dir));
    }

    #[cfg(feature = "integration-tests")]
    pub fn resource_dir(&self) -> PathBuf {
        self.tao_handle
            .state::<test::ResourceDir>()
            .inner()
            .0
            .clone()
    }

    #[cfg(feature = "integration-tests")]
    pub fn set_user_dir(&self, user_dir: PathBuf) {
        self.tao_handle.manage(test::UserDir(user_dir));
    }

    #[cfg(feature = "integration-tests")]
    pub fn user_dir(&self) -> PathBuf {
        self.tao_handle.state::<test::UserDir>().inner().0.clone()
    }

    #[cfg(feature = "integration-tests")]
    pub fn user_extensions_dir(&self) -> PathBuf {
        self.user_dir().join("extensions")
    }

    #[cfg(feature = "integration-tests")]
    pub fn tmp_dir(&self) -> PathBuf {
        self.user_dir().join("tmp")
    }

    #[cfg(feature = "integration-tests")]
    pub fn logs_dir(&self) -> PathBuf {
        self.user_dir().join("logs")
    }

    #[cfg(feature = "integration-tests")]
    pub fn globals_dir(&self) -> PathBuf {
        self.user_dir().join("globals")
    }

    #[cfg(feature = "integration-tests")]
    pub fn workspaces_dir(&self) -> PathBuf {
        self.user_dir().join("workspaces")
    }
}
