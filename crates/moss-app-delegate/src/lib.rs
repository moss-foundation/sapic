use moss_activity_broadcaster::{AppActivityBroadcaster, ToLocation, handle::ActivityHandle};
use moss_applib::AppRuntime;
use tauri::{AppHandle as TauriAppHandle, Manager};

/// A wrapper around `tauri::AppHandle` that provides
/// access to the global state and actions.
pub struct AppDelegate<R: AppRuntime> {
    app_handle: TauriAppHandle<R::EventLoop>,
    broadcaster: AppActivityBroadcaster<R::EventLoop>,
}

impl<R: AppRuntime> AppDelegate<R> {
    pub fn new(app_handle: TauriAppHandle<R::EventLoop>) -> Self {
        Self {
            app_handle: app_handle.clone(),
            broadcaster: AppActivityBroadcaster::new(app_handle),
        }
    }

    pub fn global<T>(&self) -> &T
    where
        T: Send + Sync + 'static,
    {
        self.app_handle.state::<T>().inner()
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
            app_handle: self.app_handle.clone(),
            broadcaster: self.broadcaster.clone(),
        }
    }
}
