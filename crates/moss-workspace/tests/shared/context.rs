// #![cfg(feature = "integration-tests")]

// use std::{any::TypeId, sync::Arc};

// use async_trait::async_trait;
// use moss_applib::context_old::{Context, ContextValue, ContextValueSet};
// use moss_workspace::context::{AnyWorkspaceContext, Subscribe};
// use tauri::{AppHandle, Manager, test::MockRuntime};

// pub struct MockWorkspaceContext {
//     app_handle: AppHandle<MockRuntime>,
//     values: ContextValueSet,
// }

// impl Context<MockRuntime> for MockWorkspaceContext {
//     fn global<T>(&self) -> tauri::State<'_, T>
//     where
//         T: moss_applib::GlobalMarker + std::any::Any + Send + Sync,
//     {
//         self.app_handle.state()
//     }

//     fn set_value<T: ContextValue>(&self, value: T) {
//         self.values.insert(TypeId::of::<T>(), Arc::new(value));
//     }

//     fn remove_value<T: ContextValue>(&self) {
//         self.values.remove(&TypeId::of::<T>());
//     }

//     fn value<T: ContextValue>(&self) -> Option<Arc<T>> {
//         self.values
//             .get(&TypeId::of::<T>())
//             .and_then(|v| Arc::downcast(v.clone()).ok())
//     }

//     fn spawn<T, E, Fut, F>(
//         &self,
//         callback: F,
//         timeout: Option<std::time::Duration>,
//     ) -> moss_applib::task::Task<T, E>
//     where
//         Self: Sized,
//         T: Send + 'static,
//         E: Send + 'static,
//         Fut: std::future::Future<Output = anyhow::Result<T, E>> + Send + 'static,
//         F: FnOnce(Self) -> Fut + Send + 'static,
//     {
//         let fut = callback(MockWorkspaceContext {
//             app_handle: self.app_handle.clone(),
//             values: self.values.clone(),
//         });
//         moss_applib::task::Task::new(fut, timeout)
//     }
// }

// #[async_trait]
// impl AnyWorkspaceContext<MockRuntime> for MockWorkspaceContext {
//     async fn subscribe(&self, _subscription: Subscribe) {}
// }

// impl MockWorkspaceContext {
//     pub fn new(app_handle: AppHandle<MockRuntime>) -> Self {
//         Self {
//             app_handle,
//             values: ContextValueSet::default(),
//         }
//     }
// }
