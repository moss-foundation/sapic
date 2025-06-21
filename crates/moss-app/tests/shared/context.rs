use std::{
    any::{Any, TypeId},
    sync::Arc,
    time::Duration,
};

use moss_app::context::AnyAppContext;
use moss_applib::{
    Global,
    context::{Context, ContextValue, ContextValueSet},
    task::Task,
};
use tauri::{AppHandle, Manager, State, test::MockRuntime};

pub struct MockAppContext {
    app_handle: AppHandle<MockRuntime>,
    values: ContextValueSet,
}

impl Context<MockRuntime> for MockAppContext {
    fn set_value<T: ContextValue>(&self, value: T) {
        self.values.insert(TypeId::of::<T>(), Arc::new(value));
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
        let fut = callback(MockAppContext {
            app_handle: self.app_handle.clone(),
            values: self.values.clone(),
        });
        let task = Task::new(fut, timeout);
        task
    }
}

impl AnyAppContext<MockRuntime> for MockAppContext {
    fn app_handle(&self) -> AppHandle<MockRuntime> {
        self.app_handle.clone()
    }
}

impl MockAppContext {
    pub fn new(app_handle: AppHandle<MockRuntime>) -> Self {
        Self {
            app_handle,
            values: ContextValueSet::default(),
        }
    }
}
