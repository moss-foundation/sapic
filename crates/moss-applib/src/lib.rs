pub mod context;
pub mod errors;
pub mod markers;
pub mod subscription;
pub mod task;

pub use markers::*;

use tauri::{AppHandle as TauriAppHandle, Manager, Runtime as TauriRuntime};

use crate::context::{AnyAsyncContext, AnyContext, AsyncContext, MutableContext};

/// A wrapper around `tauri::AppHandle` that provides
/// access to the global state and actions.
pub struct AppHandle<R: AppRuntime> {
    app_handle: TauriAppHandle<R::EventLoop>,
}

impl<R: AppRuntime> AppHandle<R> {
    pub fn new(app_handle: TauriAppHandle<R::EventLoop>) -> Self {
        Self { app_handle }
    }

    pub fn global<T>(&self) -> &T
    where
        T: Send + Sync + 'static,
    {
        self.app_handle.state::<T>().inner()
    }
}

impl<R: AppRuntime> Clone for AppHandle<R> {
    fn clone(&self) -> Self {
        Self {
            app_handle: self.app_handle.clone(),
        }
    }
}

pub trait AppRuntime: 'static {
    type Context: AnyContext<Frozen = Self::AsyncContext>;
    type AsyncContext: AnyAsyncContext<Unfrozen = Self::Context>;
    type EventLoop: TauriRuntime;
}

pub struct TauriAppRuntime<R: TauriRuntime>(std::marker::PhantomData<R>);

impl<R: TauriRuntime> AppRuntime for TauriAppRuntime<R> {
    type Context = MutableContext;
    type AsyncContext = AsyncContext;
    type EventLoop = R;
}

#[cfg(any(test, feature = "test"))]
pub mod mock {
    use tauri::test::MockRuntime;

    use super::*;

    pub struct MockAppRuntime;

    impl AppRuntime for MockAppRuntime {
        type Context = MutableContext;
        type AsyncContext = AsyncContext;
        type EventLoop = MockRuntime;
    }
}
