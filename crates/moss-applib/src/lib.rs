pub mod context;
pub mod context_old;
pub mod markers;
pub mod providers;
pub mod subscription;
pub mod task;

pub use markers::*;

use tauri::Runtime as TauriRuntime;

use crate::{
    context::{AnyAsyncContext, AnyContext},
    context_old::Context,
};

pub trait AppRuntime: 'static {
    type Context: AnyContext<Frozen = Self::AsyncContext>;
    type AsyncContext: AnyAsyncContext<Unfrozen = Self::Context>;
    type EventLoop: TauriRuntime;
}

pub trait ReadGlobal<R: TauriRuntime, C: Context<R>> {
    fn global(ctx: &C) -> &Self;
}
