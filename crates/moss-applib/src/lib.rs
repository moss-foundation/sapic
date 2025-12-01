pub use tauri::Wry;

use joinerror::error::ErrorMarker;
use sapic_core::context::{AnyAsyncContext, ArcContext};
use tauri::Runtime as TauriRuntime;

pub trait TauriResultExt<T> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T>;
    fn join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> joinerror::Result<T>;
    fn join_err_bare(self) -> joinerror::Result<T>;
}

impl<T> TauriResultExt<T> for Result<T, tauri::Error> {
    fn join_err<E: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<()>(e.to_string()).join::<E>(details))
    }

    fn join_err_with<E: ErrorMarker>(
        self,
        details: impl FnOnce() -> String,
    ) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<()>(e.to_string()).join_with::<E>(details))
    }

    fn join_err_bare(self) -> joinerror::Result<T> {
        self.map_err(|e| joinerror::Error::new::<()>(e.to_string()))
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
