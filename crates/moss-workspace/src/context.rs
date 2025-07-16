// TODO: Will be removed soon, need this as an example for the future

use async_trait::async_trait;
use moss_applib::{
    GlobalMarker,
    context_old::{Context, ContextValue},
    subscription::{Subscription, SubscriptionSet},
    task::Task,
};
use moss_collection::collection::OnDidChangeEvent;
use std::{any::Any, sync::Arc, time::Duration};
use tauri::{AppHandle, Manager, Runtime as TauriRuntime};
use tokio::sync::RwLock;

use crate::models::primitives::CollectionId;

#[async_trait]
pub trait AnyWorkspaceContext<R: TauriRuntime>: Context<R> {
    async fn subscribe(&self, subscription: Subscribe);
}

pub struct WorkspaceContextState {
    on_collection_did_change: SubscriptionSet<CollectionId, OnDidChangeEvent>,
}

impl WorkspaceContextState {
    pub fn new() -> Self {
        Self {
            on_collection_did_change: SubscriptionSet::new(),
        }
    }
}

pub struct WorkspaceContext<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    state: Arc<RwLock<WorkspaceContextState>>,
}

impl<R: TauriRuntime> WorkspaceContext<R> {
    pub fn new(app_handle: AppHandle<R>, state: Arc<RwLock<WorkspaceContextState>>) -> Self {
        Self { app_handle, state }
    }
}

impl<R: TauriRuntime> Context<R> for WorkspaceContext<R> {
    fn global<T>(&self) -> tauri::State<'_, T>
    where
        T: GlobalMarker + Any + Send + Sync,
    {
        self.app_handle.state::<T>()
    }

    fn spawn<T, E, Fut, F>(&self, callback: F, timeout: Option<Duration>) -> Task<T, E>
    where
        Self: Sized,
        T: Send + 'static,
        E: Send + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
        F: FnOnce(Self) -> Fut + Send + 'static,
    {
        let fut = callback(WorkspaceContext {
            app_handle: self.app_handle.clone(),
            state: self.state.clone(),
        });
        Task::new(fut, timeout)
    }

    fn set_value<T: ContextValue>(&self, _value: T) {
        todo!()
    }

    fn value<T: ContextValue>(&self) -> Option<Arc<T>> {
        todo!()
    }

    fn remove_value<T: ContextValue>(&self) {
        todo!()
    }
}

#[async_trait]
impl<R: TauriRuntime> AnyWorkspaceContext<R> for WorkspaceContext<R> {
    async fn subscribe(&self, s: Subscribe) {
        match s {
            Subscribe::OnCollectionDidChange(key, s) => {
                let mut state_lock = self.state.write().await;
                state_lock.on_collection_did_change.insert(key, s);
            }
        }
    }
}

pub enum Subscribe {
    OnCollectionDidChange(CollectionId, Subscription<OnDidChangeEvent>),
}
