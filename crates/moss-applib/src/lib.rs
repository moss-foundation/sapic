pub mod errors;
pub mod markers;
pub mod subscription;
pub mod task;

pub use markers::*;
pub use tauri::Wry;

use std::{any::Any, sync::Arc};
use tauri::{AppHandle, Runtime as TauriRuntime};

use sapic_core::context::{AnyAsyncContext, ArcContext};

/// A generic app handle that can be used to access the app's runtime context.
/// This is useful for internal plugins that need to access the app's
/// runtime context without knowing the exact runtime type.
pub struct GenericAppHandle {
    inner: Arc<dyn Any + Send + Sync>,
}

impl GenericAppHandle {
    pub fn new<R: tauri::Runtime + 'static>(handle: AppHandle<R>) -> Self {
        Self {
            inner: Arc::new(handle),
        }
    }

    pub fn downcast<R: tauri::Runtime + 'static>(&self) -> Option<AppHandle<R>> {
        self.inner.clone().downcast_ref::<AppHandle<R>>().cloned()
    }
}

pub trait AppRuntime: 'static {
    type AsyncContext: AnyAsyncContext + Clone;
    type EventLoop: TauriRuntime;
}

pub struct TauriAppRuntime<R: TauriRuntime>(std::marker::PhantomData<R>);

impl<R: TauriRuntime> AppRuntime for TauriAppRuntime<R> {
    type AsyncContext = ArcContext;
    type EventLoop = R;
}

#[cfg(any(test, feature = "test"))]
pub mod mock {
    use tauri::test::MockRuntime;

    use super::*;

    pub struct MockAppRuntime;

    impl AppRuntime for MockAppRuntime {
        type AsyncContext = ArcContext;
        type EventLoop = MockRuntime;
    }
}
