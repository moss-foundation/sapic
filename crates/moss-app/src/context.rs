use async_trait::async_trait;
use moss_applib::{Global, context::Context, task::Task};
use std::{any::Any, future::Future, ops::Deref, time::Duration};
use tauri::{AppHandle, Manager, Runtime as TauriRuntime, State};

use crate::app::App;

#[async_trait]
pub trait AnyAppContext<R: TauriRuntime>: Context<R> {
    fn app_handle(&self) -> AppHandle<R>;
}

#[derive(Clone)]
pub struct AppContext<R: TauriRuntime> {
    app_handle: AppHandle<R>,
}

impl<R: TauriRuntime> Context<R> for AppContext<R> {
    fn global<T>(&self) -> State<'_, T>
    where
        T: Global + Any + Send + Sync,
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
        });
        Task::new(fut, timeout)
    }
}

impl<R: TauriRuntime> Deref for AppContext<R> {
    type Target = AppHandle<R>;

    fn deref(&self) -> &Self::Target {
        &self.app_handle
    }
}

impl<R: TauriRuntime> From<App<R>> for AppContext<R> {
    fn from(app: App<R>) -> Self {
        AppContext {
            app_handle: app.app_handle().clone(),
        }
    }
}

impl<R: TauriRuntime> From<State<'_, App<R>>> for AppContext<R> {
    fn from(app: State<'_, App<R>>) -> Self {
        AppContext {
            app_handle: app.app_handle().clone(),
        }
    }
}

impl<R: TauriRuntime> From<&State<'_, App<R>>> for AppContext<R> {
    fn from(app: &State<'_, App<R>>) -> Self {
        AppContext {
            app_handle: app.app_handle().clone(),
        }
    }
}
