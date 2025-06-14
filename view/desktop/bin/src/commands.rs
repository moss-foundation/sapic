mod app;
mod workbench;
mod workspace;

pub use app::*;
pub use workbench::*;
pub use workspace::*;

use moss_app::context::{AppContext, Context, Global, Task};
use moss_workbench::workbench::Workbench;
use std::{any::Any, ops::Deref, sync::Arc, time::Duration};
use tauri::{AppHandle, Manager, Runtime as TauriRuntime, State};

trait ReadWorkbench<R: TauriRuntime> {
    fn workbench(&self) -> State<'_, Workbench<R>>;
}

impl<R: TauriRuntime> ReadWorkbench<R> for AppHandle<R> {
    fn workbench(&self) -> State<'_, Workbench<R>> {
        self.state::<Workbench<R>>()
    }
}

pub struct StateContext<R: TauriRuntime> {
    app_context: AppContext<R>,
}

impl<R: TauriRuntime> Deref for StateContext<R> {
    type Target = AppContext<R>;

    fn deref(&self) -> &Self::Target {
        &self.app_context
    }
}

impl<R: TauriRuntime> From<AppContext<R>> for StateContext<R> {
    fn from(app_context: AppContext<R>) -> Self {
        Self { app_context }
    }
}

impl<R: TauriRuntime> Context<R> for StateContext<R> {
    fn global<T>(&self) -> Arc<T>
    where
        T: Global + Any + Send + Sync,
    {
        self.app_context.global()
    }

    fn spawn<T, E, Fut, F>(&self, callback: F, timeout: Option<Duration>) -> Task<T, E>
    where
        T: Send + 'static,
        E: Send + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
        F: FnOnce(Self) -> Fut + Send + 'static,
    {
        self.app_context
            .spawn(|ctx| callback(StateContext::from(ctx)), timeout)
    }
}
