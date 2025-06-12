use rustc_hash::FxHashMap;
use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    ops::Deref,
    pin::Pin,
    sync::Arc,
    task::{Context as TaskContext, Poll},
    time::Duration,
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::{sync::oneshot, time::Instant};

pub trait Global: 'static {}

pub trait ReadGlobal<R: TauriRuntime, C: Context<R>> {
    fn global(ctx: &C) -> &Self;
}

pub trait Context<R: TauriRuntime>: Send + Sync {
    fn global<T>(&self) -> Arc<T>
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

pub struct AppContextBuilder<R: TauriRuntime> {
    globals_by_type: FxHashMap<TypeId, Arc<dyn Any + Send + Sync>>,
    _marker: PhantomData<R>,
}

impl<R: TauriRuntime> AppContextBuilder<R> {
    pub fn new() -> Self {
        Self {
            globals_by_type: FxHashMap::default(),
            _marker: PhantomData,
        }
    }

    pub fn set_global<T>(&mut self, global: T)
    where
        T: Global + Any + Send + Sync,
    {
        self.globals_by_type
            .insert(TypeId::of::<T>(), Arc::new(global));
    }

    pub fn build(self, app_handle: AppHandle<R>) -> AppContext<R> {
        AppContext {
            app_handle,
            globals_by_type: Arc::new(self.globals_by_type),
        }
    }
}

pub enum TaskResult<T, E> {
    Ok(T),
    Err(E),
    Timeout,
    Cancelled,
}

pub struct Task<T, E> {
    inner: Pin<Box<dyn Future<Output = TaskResult<T, E>> + Send>>,
    cancel: Option<oneshot::Sender<()>>,
}

impl<T: Send + 'static, E: Send + 'static> Task<T, E> {
    pub fn new<F>(future: F, timeout: Option<Duration>) -> Self
    where
        F: Future<Output = Result<T, E>> + Send + 'static,
    {
        let (tx, mut rx) = oneshot::channel::<()>();
        let fut = async move {
            match timeout {
                Some(duration) => {
                    let deadline = Instant::now() + duration;
                    tokio::select! {
                        res = future => match res {
                            Ok(val) => TaskResult::Ok(val),
                            Err(e) => TaskResult::Err(e),
                        },
                        _ = &mut rx => TaskResult::Cancelled,
                        _ = tokio::time::sleep(deadline.saturating_duration_since(Instant::now())) => TaskResult::Timeout,
                    }
                }
                None => {
                    tokio::select! {
                        res = future => match res {
                            Ok(val) => TaskResult::Ok(val),
                            Err(e) => TaskResult::Err(e),
                        },
                        _ = &mut rx => TaskResult::Cancelled,
                    }
                }
            }
        };

        Task {
            inner: Box::pin(fut),
            cancel: Some(tx),
        }
    }

    pub fn cancel(&mut self) {
        if let Some(tx) = self.cancel.take() {
            let _ = tx.send(());
        }
    }
}

impl<T: Send + 'static, E: Send + 'static> Future for Task<T, E> {
    type Output = TaskResult<T, E>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(cx)
    }
}

#[derive(Clone)]
pub struct AppContext<R: TauriRuntime> {
    app_handle: AppHandle<R>,
    globals_by_type: Arc<FxHashMap<TypeId, Arc<dyn Any + Send + Sync>>>,
}

impl<R: TauriRuntime> Context<R> for AppContext<R> {
    fn global<T>(&self) -> Arc<T>
    where
        T: Global + Any + Send + Sync,
    {
        let raw_arc = self
            .globals_by_type
            .get(&TypeId::of::<T>())
            .cloned()
            .expect(&format!(
                "Global resource {} expected to be registered",
                std::any::type_name::<T>()
            ));

        Arc::downcast::<T>(raw_arc).expect(&format!(
            "Global resource {} is registered with the wrong type id",
            std::any::type_name::<T>()
        ))
    }

    fn spawn<T, E, Fut, F>(&self, callback: F, timeout: Option<Duration>) -> Task<T, E>
    where
        T: Send + 'static,
        E: Send + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
        F: FnOnce(Self) -> Fut + Send + 'static,
    {
        let fut = callback(AppContext {
            app_handle: self.app_handle.clone(),
            globals_by_type: self.globals_by_type.clone(),
        });
        Task::new(fut, timeout)
    }
}

impl<R: TauriRuntime> Deref for AppContext<R> {
    type Target = AppHandle<R>;

    fn deref(&self) -> &Self::Target {
        &self.app_handle
    }
}
