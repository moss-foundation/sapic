pub mod context;
pub mod errors;
pub mod markers;
pub mod subscription;
pub mod task;

pub use markers::*;
pub use tauri::Wry;

use tauri::Runtime as TauriRuntime;

use crate::context::{AnyAsyncContext, AnyContext, AsyncContext, MutableContext};

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
