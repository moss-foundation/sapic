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

use crate::{Global, task::Task};

pub trait ContextValue: Any + Send + Sync + 'static {}

#[derive(Deref, DerefMut, Default, Clone)]
pub struct ContextValueSet(Arc<DashMap<TypeId, Arc<dyn ContextValue>>>);

pub trait Context<R: TauriRuntime>: Send + Sync {
    fn set_value<T: ContextValue>(&self, value: T);
    fn remove_value<T: ContextValue>(&self);
    fn value<T: ContextValue>(&self) -> Option<Arc<T>>;

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

#[cfg(any(test, feature = "test"))]
#[cfg_attr(docsrs, doc(cfg(feature = "test")))]
pub mod test {
    use super::*;

    use tauri::{AppHandle, Manager, State, test::MockRuntime};

    #[derive(Clone)]
    pub struct MockContext {
        app_handle: AppHandle<MockRuntime>,
        values: ContextValueSet,
    }

    impl MockContext {
        pub fn new(app_handle: AppHandle<MockRuntime>) -> Self {
            Self {
                app_handle,
                values: ContextValueSet::default(),
            }
        }
    }

    impl<R: TauriRuntime> Context<R> for MockContext {
        fn set_value<T: ContextValue>(&self, value: T) {
            self.values.insert(TypeId::of::<T>(), Arc::new(value));
        }

        fn remove_value<T: ContextValue>(&self) {
            self.values.remove(&TypeId::of::<T>());
        }

        fn value<T: ContextValue>(&self) -> Option<Arc<T>> {
            self.values
                .get(&TypeId::of::<T>())
                .and_then(|v| Arc::downcast(v.clone()).ok())
        }

        fn global<T>(&self) -> State<'_, T>
        where
            T: Global + Any + Send + Sync,
        {
            self.app_handle.state()
        }

        fn spawn<T, E, Fut, F>(&self, callback: F, timeout: Option<Duration>) -> Task<T, E>
        where
            Self: Sized,
            T: Send + 'static,
            E: Send + 'static,
            Fut: Future<Output = Result<T, E>> + Send + 'static,
            F: FnOnce(Self) -> Fut + Send + 'static,
        {
            let fut = callback(MockContext {
                app_handle: self.app_handle.clone(),
                values: self.values.clone(),
            });
            let task = Task::new(fut, timeout);
            task
        }
    }
}
