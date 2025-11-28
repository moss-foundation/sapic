pub mod errors;
pub mod markers;
pub mod subscription;
pub mod task;

pub use tauri::Wry;

use tauri::Runtime as TauriRuntime;

use sapic_core::context::{AnyAsyncContext, ArcContext};

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
