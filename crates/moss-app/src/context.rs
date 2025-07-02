use async_trait::async_trait;
use moss_applib::{
    GlobalMarker,
    context::{Context, ContextValue, ContextValueSet},
    task::Task,
};
use std::{
    any::{Any, TypeId},
    future::Future,
    sync::Arc,
    time::Duration,
};
use tauri::{AppHandle, Manager, Runtime as TauriRuntime, State};

use crate::app::App;

pub mod ctxkeys {
    use moss_applib::context::ContextValue;

    /// The id of the workspace that is currently active.
    #[derive(Debug, Deref, From, PartialEq, Eq, Hash)]
    pub struct WorkspaceId(String);

    impl ContextValue for WorkspaceId {}
}

#[async_trait]
pub trait AnyAppContext<R: TauriRuntime>: Context<R> {
    fn app_handle(&self) -> AppHandle<R>;
}

#[derive(Clone)]
pub struct AppContext<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    values: ContextValueSet,
}

impl<R: TauriRuntime> AnyAppContext<R> for AppContext<R> {
    fn app_handle(&self) -> AppHandle<R> {
        self.app_handle.clone()
    }
}

impl<R: TauriRuntime> Context<R> for AppContext<R> {
    fn global<T>(&self) -> State<'_, T>
    where
        T: GlobalMarker + Any + Send + Sync,
    {
        self.app_handle.state::<T>()
    }

    fn spawn<T, E, Fut, F>(&self, callback: F, timeout: Option<Duration>) -> Task<T, E>
    where
        T: Send + 'static,
        E: Send + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
        F: FnOnce(Self) -> Fut + Send + 'static,
    {
        let fut = callback(AppContext {
            app_handle: self.app_handle.clone(),
            values: self.values.clone(),
        });
        Task::new(fut, timeout)
    }

    fn set_value<T: ContextValue>(&self, value: T) {
        self.values.insert(TypeId::of::<T>(), Arc::new(value));
    }

    fn remove_value<T: ContextValue>(&self) {
        self.values.remove(&TypeId::of::<T>());
    }

    fn value<T: ContextValue>(&self) -> Option<Arc<T>> {
        self.values
            .get(&TypeId::of::<T>())
            .and_then(|v| Arc::downcast(v.clone()).ok())
    }
}

impl<R: TauriRuntime> From<App<R>> for AppContext<R> {
    fn from(app: App<R>) -> Self {
        AppContext {
            app_handle: app.app_handle().clone(),
            values: app.state::<ContextValueSet>().inner().clone(),
        }
    }
}

impl<R: TauriRuntime> From<State<'_, App<R>>> for AppContext<R> {
    fn from(app: State<'_, App<R>>) -> Self {
        AppContext {
            app_handle: app.app_handle().clone(),
            values: app.state::<ContextValueSet>().inner().clone(),
        }
    }
}

impl<R: TauriRuntime> From<&State<'_, App<R>>> for AppContext<R> {
    fn from(app: &State<'_, App<R>>) -> Self {
        AppContext {
            app_handle: app.app_handle().clone(),
            values: app.state::<ContextValueSet>().inner().clone(),
        }
    }
}
