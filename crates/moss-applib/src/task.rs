// TODO: should be moved to core crate

use std::{
    future::Future,
    pin::Pin,
    task::{Context as TaskContext, Poll},
};

use sapic_core::context::{AnyAsyncContext, ArcContext, AwaitCancel, Reason};
use tokio::task::JoinHandle;

use sapic_errors::{Cancelled, Timeout};

/// A detached task that runs in the background with context support.
/// Can be cancelled via context or awaited to get the result.
#[must_use = "detached tasks should be awaited or explicitly dropped"]
pub struct DetachedTask<T> {
    handle: JoinHandle<joinerror::Result<T>>,
    ctx: ArcContext,
}

impl<T> DetachedTask<T> {
    /// Cancels the detached task through its context.
    pub fn cancel(&self) {
        self.ctx.get_canceller().cancel();
    }

    /// Aborts the detached task immediately without graceful cancellation.
    pub fn abort(&self) {
        self.handle.abort();
    }

    /// Checks if the task is finished.
    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }

    /// Get a reference to the task's context.
    pub fn context(&self) -> &ArcContext {
        &self.ctx
    }

    /// Wraps this detached task to log any errors when it completes.
    /// Returns a new DetachedTask that logs errors automatically.
    /// Usage: `task.detach().log_if_err("processing data")`
    #[track_caller]
    pub fn log_if_err(self, details: &str) -> Self
    where
        T: Send + 'static,
    {
        use moss_logging::ResultSessionLogExt;

        let location = std::panic::Location::caller();
        let location_str = format!("{}:{}", location.file(), location.line());
        let details_str = format!("{} at {}", details, location_str);

        let ctx = self.ctx.clone();
        let handle = self.handle;

        // Create new handle that wraps the original and logs errors
        let new_handle = tokio::spawn(async move {
            let result = match handle.await {
                Ok(inner_result) => inner_result.log_err(&details_str),
                Err(join_error) => {
                    // Task panicked or was aborted
                    moss_logging::session::error!(format!(
                        "Task panicked or aborted: {} - {:?}",
                        details_str, join_error
                    ));
                    // Convert JoinError to a joinerror::Error
                    Err(joinerror::Error::new::<sapic_errors::Internal>(format!(
                        "Task join error: {:?}",
                        join_error
                    )))
                }
            };
            result
        });

        DetachedTask {
            handle: new_handle,
            ctx,
        }
    }
}

impl<T> Future for DetachedTask<T> {
    type Output = Result<joinerror::Result<T>, tokio::task::JoinError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.handle).poll(cx)
    }
}

/// Task is a primitive that allows work to happen with context-based cancellation and timeout.
///
/// It implements [`Future`] so you can `.await` on it.
///
/// If you drop a task, it will be cancelled via context. Calling [`Task::detach`] allows
/// the task to continue running in the background.
#[must_use = "tasks do nothing unless you `.await` or `.detach()` them"]
pub struct Task<T> {
    state: TaskState<T>,
}

enum TaskState<T> {
    /// A task that is ready to return a value immediately
    Ready(Option<joinerror::Result<T>>),
    /// A task that is currently running with context
    Spawned {
        inner: Pin<Box<dyn Future<Output = joinerror::Result<T>> + Send>>,
        ctx: ArcContext,
    },
}

impl<T: Send + 'static> Task<T> {
    /// Creates a new task that will resolve with a ready value.
    /// This is useful for optimization when you already have a value.
    pub fn ready(result: joinerror::Result<T>) -> Self {
        Task {
            state: TaskState::Ready(Some(result)),
        }
    }

    /// Creates a task with context support.
    /// The task will be cancelled if the context is cancelled or times out.
    pub fn with_context<F>(ctx: &ArcContext, future: F) -> Self
    where
        F: Future<Output = joinerror::Result<T>> + Send + 'static,
    {
        let cancellation = ctx.cancellation();
        let ctx_clone = ctx.clone();

        let fut = async move {
            let cancel_fut = cancellation.wait();
            tokio::pin!(cancel_fut);

            tokio::select! {
                biased;
                reason = cancel_fut => {
                    Err(reason_to_error(reason))
                }
                result = future => result,
            }
        };

        Task {
            state: TaskState::Spawned {
                inner: Box::pin(fut),
                ctx: ctx_clone,
            },
        }
    }

    /// Creates a background task without any context (will never timeout or cancel automatically).
    /// For most use cases, prefer `with_context` or `with_timeout`.
    pub fn spawn<F>(future: F) -> Self
    where
        F: Future<Output = joinerror::Result<T>> + Send + 'static,
    {
        let ctx = ArcContext::background();
        Self::with_context(&ctx, future)
    }

    /// Creates a task with a timeout but no parent context.
    pub fn with_timeout<F>(timeout: std::time::Duration, future: F) -> Self
    where
        F: Future<Output = joinerror::Result<T>> + Send + 'static,
    {
        let ctx = ArcContext::background_with_timeout(timeout);
        Self::with_context(&ctx, future)
    }

    /// Cancels the task if it's still running (through its context).
    pub fn cancel(&self) {
        if let TaskState::Spawned { ctx, .. } = &self.state {
            ctx.get_canceller().cancel();
        }
    }

    /// Get a reference to the task's context (if spawned).
    pub fn context(&self) -> Option<&ArcContext> {
        match &self.state {
            TaskState::Ready(_) => None,
            TaskState::Spawned { ctx, .. } => Some(ctx),
        }
    }

    /// Detaches the task and runs it in the background.
    /// Returns a `DetachedTask` handle that can be used to cancel or await the task.
    pub fn detach(self) -> DetachedTask<T> {
        match self.state {
            TaskState::Ready(result) => {
                let ctx = ArcContext::background();
                // For ready tasks, spawn a task that immediately returns the value
                let handle = tokio::spawn(async move { result.unwrap() });
                DetachedTask { handle, ctx }
            }
            TaskState::Spawned { inner, ctx } => {
                let handle = tokio::spawn(async move {
                    struct TaskFuture<T> {
                        inner: Pin<Box<dyn Future<Output = joinerror::Result<T>> + Send>>,
                    }

                    impl<T> Future for TaskFuture<T> {
                        type Output = joinerror::Result<T>;

                        fn poll(
                            mut self: Pin<&mut Self>,
                            cx: &mut TaskContext<'_>,
                        ) -> Poll<Self::Output> {
                            self.inner.as_mut().poll(cx)
                        }
                    }

                    TaskFuture { inner }.await
                });
                DetachedTask { handle, ctx }
            }
        }
    }
}

impl<T: Send + 'static> Future for Task<T> {
    type Output = joinerror::Result<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() };
        match &mut this.state {
            TaskState::Ready(result) => Poll::Ready(result.take().unwrap()),
            TaskState::Spawned { inner, .. } => inner.as_mut().poll(cx),
        }
    }
}

/// Helper function to convert context Reason to joinerror Error
fn reason_to_error(reason: Reason) -> joinerror::Error {
    match reason {
        Reason::Timeout => joinerror::Error::new::<Timeout>("operation timed out"),
        Reason::Canceled => joinerror::Error::new::<Cancelled>("operation was cancelled"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sapic_core::context::ArcContext;
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_task_with_context_success() {
        let ctx = ArcContext::background_with_timeout(Duration::from_millis(200));

        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(10)).await;
            Ok::<_, joinerror::Error>(42)
        });

        let result = task.await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_task_with_context_timeout() {
        let ctx = ArcContext::background_with_timeout(Duration::from_millis(20));

        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(200)).await;
            Ok::<_, joinerror::Error>(42)
        });

        let result = task.await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<Timeout>());
    }

    #[tokio::test]
    async fn test_task_with_context_cancel() {
        let ctx = ArcContext::background_with_timeout(Duration::from_secs(5));
        let canceller = ctx.get_canceller();

        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_secs(5)).await;
            Ok::<_, joinerror::Error>(42)
        });

        // Cancel after a short delay
        tokio::spawn(async move {
            sleep(Duration::from_millis(10)).await;
            canceller.cancel();
        });

        let result = task.await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<Cancelled>());
    }

    #[tokio::test]
    async fn test_task_ready() {
        let task = Task::ready(Ok(99));
        let result = task.await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 99);
    }

    #[tokio::test]
    async fn test_task_ready_error() {
        let error = joinerror::Error::new::<sapic_errors::Internal>("test error");
        let task: Task<i32> = Task::ready(Err(error));
        let result = task.await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<sapic_errors::Internal>());
    }

    #[tokio::test]
    async fn test_task_spawn() {
        let task = Task::spawn(async {
            sleep(Duration::from_millis(10)).await;
            Ok(123)
        });

        let result = task.await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 123);
    }

    #[tokio::test]
    async fn test_task_with_timeout_success() {
        let task = Task::with_timeout(Duration::from_millis(100), async {
            sleep(Duration::from_millis(10)).await;
            Ok(42)
        });

        let result = task.await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_task_with_timeout_expired() {
        let task = Task::with_timeout(Duration::from_millis(10), async {
            sleep(Duration::from_millis(200)).await;
            Ok(42)
        });

        let result = task.await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<Timeout>());
    }

    #[tokio::test]
    async fn test_task_cancel_method() {
        let ctx = ArcContext::background();
        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(200)).await;
            Ok(42)
        });

        // Cancel immediately
        task.cancel();

        let result = task.await;
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<Cancelled>());
    }

    #[tokio::test]
    async fn test_detach_success() {
        let ctx = ArcContext::background();
        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(50)).await;
            Ok(42)
        });

        let detached = task.detach();
        let result = detached.await.unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_detach_cancel() {
        let ctx = ArcContext::background();
        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(200)).await;
            Ok(42)
        });

        let detached = task.detach();

        // Cancel through detached handle
        sleep(Duration::from_millis(10)).await;
        detached.cancel();

        let result = detached.await.unwrap();
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<Cancelled>());
    }

    #[tokio::test]
    async fn test_detach_abort() {
        let ctx = ArcContext::background();
        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(200)).await;
            Ok(42)
        });

        let detached = task.detach();

        // Abort immediately
        sleep(Duration::from_millis(10)).await;
        detached.abort();

        let result = detached.await;
        assert!(result.is_err()); // JoinError
    }

    #[tokio::test]
    async fn test_detach_is_finished() {
        let ctx = ArcContext::background();
        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(50)).await;
            Ok(42)
        });

        let detached = task.detach();

        // Should not be finished immediately
        assert!(!detached.is_finished());

        // Wait for completion
        sleep(Duration::from_millis(100)).await;

        // Should be finished now
        assert!(detached.is_finished());
    }

    #[tokio::test]
    async fn test_context_hierarchy() {
        let parent_ctx = ArcContext::background();
        let child_ctx = ArcContext::new(parent_ctx.clone());

        let task1 = Task::with_context(&parent_ctx, async {
            sleep(Duration::from_millis(200)).await;
            Ok(1)
        });

        let task2 = Task::with_context(&child_ctx, async {
            sleep(Duration::from_millis(200)).await;
            Ok(2)
        });

        // Cancel parent - should cancel both tasks
        let parent_ctx_clone = parent_ctx.clone();
        tokio::spawn(async move {
            sleep(Duration::from_millis(10)).await;
            parent_ctx_clone.get_canceller().cancel();
        });

        let result1 = task1.await;
        let result2 = task2.await;

        assert!(result1.is_err());
        assert!(result1.unwrap_err().is::<Cancelled>());
        assert!(result2.is_err());
        assert!(result2.unwrap_err().is::<Cancelled>());
    }

    #[tokio::test]
    async fn test_detach_log_if_err_success() {
        let ctx = ArcContext::background();
        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(10)).await;
            Ok(42)
        });

        // Should not log anything for success
        let detached = task.detach().log_if_err("test operation");
        let result = detached.await.unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_detach_log_if_err_error() {
        let ctx = ArcContext::background();
        let task: Task<i32> = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(10)).await;
            Err(joinerror::Error::new::<sapic_errors::Internal>(
                "test error",
            ))
        });

        // Should log the error
        let detached = task.detach().log_if_err("failed operation");
        let result = detached.await.unwrap();
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_detach_log_if_err_timeout() {
        let task = Task::with_timeout(Duration::from_millis(10), async {
            sleep(Duration::from_millis(200)).await;
            Ok(42)
        });

        // Should log timeout warning
        let detached = task.detach().log_if_err("timeout operation");
        let result = detached.await.unwrap();
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<Timeout>());
    }

    #[tokio::test]
    async fn test_detach_log_if_err_cancelled() {
        let ctx = ArcContext::background();
        let canceller = ctx.get_canceller();

        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(200)).await;
            Ok(42)
        });

        let detached = task.detach().log_if_err("cancelled operation");

        // Cancel after a short delay
        tokio::spawn(async move {
            sleep(Duration::from_millis(10)).await;
            canceller.cancel();
        });

        let result = detached.await.unwrap();
        assert!(result.is_err());
        assert!(result.unwrap_err().is::<Cancelled>());
    }

    #[tokio::test]
    async fn test_log_if_err_fire_and_forget() {
        let ctx = ArcContext::background();
        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(10)).await;
            Ok(())
        });

        // Fire and forget pattern - explicitly drop the handle
        let _ = task.detach().log_if_err("background work");

        // Give it time to complete
        sleep(Duration::from_millis(50)).await;
    }

    #[tokio::test]
    async fn test_ready_is_fast() {
        use std::time::Instant;

        let start = Instant::now();

        // Create 1000 ready tasks - should be very fast
        let tasks: Vec<_> = (0..1000).map(|i| Task::ready(Ok(i))).collect();

        // Await all tasks
        for task in tasks {
            let _ = task.await;
        }

        let elapsed = start.elapsed();
        // Ready tasks should complete almost instantly (well under 100ms)
        assert!(elapsed.as_millis() < 100, "Took too long: {:?}", elapsed);
    }

    #[tokio::test]
    async fn test_detach_ready_task() {
        let task = Task::ready(Ok(99));
        let detached = task.detach();
        let result = detached.await.unwrap();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 99);
    }

    #[tokio::test]
    async fn test_task_context_access() {
        let ctx = ArcContext::background();
        let task = Task::with_context(&ctx, async {
            sleep(Duration::from_millis(10)).await;
            Ok(42)
        });

        // Should have context
        assert!(task.context().is_some());

        let result = task.await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_ready_task_no_context() {
        let task = Task::ready(Ok(42));

        // Ready tasks have no context
        assert!(task.context().is_none());

        let result = task.await;
        assert!(result.is_ok());
    }
}
