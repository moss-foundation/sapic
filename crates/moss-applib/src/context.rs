use anyhow::Result;
use std::{any::Any, future::Future};
use tauri::{Runtime as TauriRuntime, State};
use tokio::time::Duration;

use crate::{Global, task::Task};

pub trait Event: Any + Send + Sync + 'static {}

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

    // fn subscribe<T>(&self, event: T) -> Result<()>
    // where
    //     T: Event + Send + Sync;
}

#[cfg(any(test, feature = "test"))]
#[cfg_attr(docsrs, doc(cfg(feature = "test")))]
pub mod test {
    use super::*;

    use tauri::{AppHandle, Manager, State, test::MockRuntime};

    pub struct MockContext {
        app_handle: AppHandle<MockRuntime>,
    }

    impl MockContext {
        pub fn new(app_handle: AppHandle<MockRuntime>) -> Self {
            Self { app_handle }
        }
    }

    impl<R: TauriRuntime> Context<R> for MockContext {
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
            });
            let task = Task::new(fut, timeout);
            task
        }
    }
}
