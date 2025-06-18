pub mod context;
pub mod markers;
pub mod subscription;
pub mod task;

pub use markers::*;

use tauri::Runtime as TauriRuntime;

use crate::context::Context;

pub trait ReadGlobal<R: TauriRuntime, C: Context<R>> {
    fn global(ctx: &C) -> &Self;
}
