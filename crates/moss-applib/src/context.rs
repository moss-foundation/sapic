use anyhow::Result;
use dashmap::DashMap;
use derive_more::{Deref, DerefMut};
use std::{
    any::{Any, TypeId},
    future::Future,
    sync::Arc,
};
use tauri::{Runtime as TauriRuntime, State};
use tokio::time::Duration;

use crate::{GlobalMarker, task::Task};

pub trait ContextValue: Any + Send + Sync + 'static {}

#[derive(Deref, DerefMut, Default, Clone)]
pub struct ContextValueSet(Arc<DashMap<TypeId, Arc<dyn ContextValue>>>);

pub trait Context<R: TauriRuntime>: Send + Sync {
    fn set_value<T: ContextValue>(&self, value: T);
    fn remove_value<T: ContextValue>(&self);
    fn value<T: ContextValue>(&self) -> Option<Arc<T>>;

    fn global<T>(&self) -> State<'_, T>
    where
        T: GlobalMarker + Any + Send + Sync;

    fn spawn<T, E, Fut, F>(&self, callback: F, timeout: Option<Duration>) -> Task<T, E>
    where
        Self: Sized,
        T: Send + 'static,
        E: Send + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
        F: FnOnce(Self) -> Fut + Send + 'static;
}
