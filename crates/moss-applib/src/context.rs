use std::any::Any;
use tauri::{Runtime as TauriRuntime, State};
use tokio::time::Duration;

use crate::{Global, task::Task};

pub trait Context<R: TauriRuntime>: Send + Sync {
    fn global<T>(&self) -> State<'_, T>
    where
        T: Global + Any + Send + Sync;

    fn spawn<T, E, Fut, F>(&self, callback: F, timeout: Option<Duration>) -> Task<T, E>
    where
        Self: Sized,
        T: Send + 'static,
        E: Send + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
        F: FnOnce(Self) -> Fut + Send + 'static;
}
