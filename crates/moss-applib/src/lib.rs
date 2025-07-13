pub mod context;
pub mod ctx;
pub mod markers;
pub mod providers;
pub mod subscription;
pub mod task;

pub use markers::*;

use tauri::Runtime as TauriRuntime;

use crate::{
    context::Context,
    ctx::{AnyAsyncContext, AnyContext},
};

pub trait AppRuntime: 'static {
    type Context: AnyContext<Frozen = Self::AsyncContext>;
    type AsyncContext: AnyAsyncContext<Unfrozen = Self::Context>;
    type EventLoop: TauriRuntime;
}

pub trait ReadGlobal<R: TauriRuntime, C: Context<R>> {
    fn global(ctx: &C) -> &Self;
}
