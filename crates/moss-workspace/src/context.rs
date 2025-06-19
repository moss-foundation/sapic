use async_trait::async_trait;
use moss_applib::{
    context::Context,
    subscription::{Subscription, SubscriptionSet},
    task::Task,
};
use moss_collection::collection::OnDidChangeEvent;
use std::{sync::Arc, time::Duration};
use tauri::{AppHandle, Manager, Runtime as TauriRuntime};
use tokio::sync::RwLock;

use crate::models::primitives::CollectionId;

pub struct WorkspaceContextState {
    // collection_contexts: FxHashMap<Uuid, CollectionContext>,
    on_collection_did_change: SubscriptionSet<CollectionId, OnDidChangeEvent>,
}

pub struct NewWorkspaceContext<'a, R: TauriRuntime> {
    app_handle: AppHandle<R>,
    state: &'a WorkspaceContextState,
}

pub struct WorkspaceContext<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    // state: Arc<RwLock<WorkspaceContextState>>,
    state: WorkspaceContextState,
}

impl<R: TauriRuntime> WorkspaceContext<R> {
    pub fn new(app_handle: AppHandle<R>) -> Self {
        Self {
            app_handle,
            state: WorkspaceContextState {
                on_collection_did_change: SubscriptionSet::new(),
            },
        }
    }
}

impl<R: TauriRuntime> Context<R> for WorkspaceContext<R> {
    fn global<T>(&self) -> tauri::State<'_, T>
    where
        T: moss_applib::Global + std::any::Any + Send + Sync,
    {
        self.app_handle.state::<T>()
    }

    fn spawn<T, E, Fut, F>(&self, callback: F, timeout: Option<Duration>) -> Task<T, E>
    where
        Self: Sized,
        T: Send + 'static,
        E: Send + 'static,
        Fut: Future<Output = anyhow::Result<T, E>> + Send + 'static,
        F: FnOnce(Self) -> Fut + Send + 'static,
    {
        let fut = callback(WorkspaceContext {
            app_handle: self.app_handle.clone(),
            state: self.state.clone(),
        });
        Task::new(fut, timeout)
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

#[async_trait]
pub trait AnyWorkspaceContext<R: TauriRuntime>: Context<R> {
    async fn subscribe(&self, subscription: Subscribe);
}
