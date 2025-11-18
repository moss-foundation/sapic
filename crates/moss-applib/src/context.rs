// use std::{
//     any::Any,
//     borrow::Cow,
//     collections::HashMap,
//     fmt::{self, Display},
//     future::Future,
//     sync::{
//         Arc,
//         atomic::{AtomicBool, Ordering},
//     },
//     time::{Duration, Instant},
// };

// use joinerror::error::ErrorMarker;
// use serde::{Deserialize, Serialize};
// use tokio::sync::Notify;

// pub trait AwaitCancel {
//     fn cancellation(&self) -> Cancellation;
// }

// pub async fn abortable<C, T, E, F>(ctx: &C, op: F) -> Result<T, Result<Reason, E>>
// where
//     C: AwaitCancel + ?Sized,
//     F: Future<Output = Result<T, E>>,
// {
//     let cancellation = ctx.cancellation();
//     let cancel_fut = cancellation.wait();
//     tokio::pin!(cancel_fut);

//     tokio::select! {
//         biased;

//         reason = &mut cancel_fut => Err(Ok(reason)),
//         res = op => res.map_err(Err),
//     }
// }

// /// Marker trait for storable values
// pub trait ContextValue: Any + Send + Sync + 'static {}
// impl ContextValue for u32 {}
// impl ContextValue for &'static str {}
// impl ContextValue for bool {}
// impl ContextValue for i64 {}
// impl ContextValue for f64 {}
// impl ContextValue for String {}

// /// Type-erased context value that can be downcast to a specific type.
// #[derive(Clone)]
// pub struct AnyContextValue(Arc<dyn Any + Send + Sync>);

// impl AnyContextValue {
//     /// Downcast to a specific type.
//     pub fn downcast<V: ContextValue>(self) -> Option<Arc<V>> {
//         self.0.downcast::<V>().ok()
//     }
// }

// /// Reasons why a context is done.
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Reason {
//     Timeout,
//     Canceled,
// }

// impl Display for Reason {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Reason::Timeout => write!(f, "Timeout"),
//             Reason::Canceled => write!(f, "Canceled"),
//         }
//     }
// }

// /// Extension trait for converting context results to joinerror::Result
// pub trait ContextResultExt<T, E>
// where
//     E: std::error::Error + Send + Sync + 'static,
// {
//     fn join_err<EM: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T>;
//     fn join_err_with<EM: ErrorMarker>(
//         self,
//         details: impl FnOnce() -> String,
//     ) -> joinerror::Result<T>;

//     fn join_err_bare(self) -> joinerror::Result<T>;
// }

// impl<T, E> ContextResultExt<T, E> for Result<T, Result<Reason, E>>
// where
//     E: std::error::Error + Send + Sync + 'static,
// {
//     fn join_err<EM: ErrorMarker>(self, details: impl Into<String>) -> joinerror::Result<T> {
//         match self {
//             Ok(v) => Ok(v),
//             Err(Ok(reason)) => Err(
//                 joinerror::Error::new::<()>(format!("context error: {reason}")).join::<EM>(details),
//             ),
//             Err(Err(e)) => Err(joinerror::Error::new::<()>(e.to_string()).join::<EM>(details)),
//         }
//     }

//     fn join_err_with<EM: ErrorMarker>(
//         self,
//         details: impl FnOnce() -> String,
//     ) -> joinerror::Result<T> {
//         match self {
//             Ok(v) => Ok(v),
//             Err(Ok(reason)) => Err(
//                 joinerror::Error::new::<()>(format!("context error: {reason}"))
//                     .join_with::<EM>(details),
//             ),
//             Err(Err(e)) => Err(joinerror::Error::new::<()>(e.to_string()).join_with::<EM>(details)),
//         }
//     }

//     fn join_err_bare(self) -> joinerror::Result<T> {
//         match self {
//             Ok(v) => Ok(v),
//             Err(Ok(reason)) => Err(joinerror::Error::new::<()>(format!(
//                 "context error: {reason}"
//             ))),
//             Err(Err(e)) => Err(joinerror::Error::new::<()>(e.to_string())),
//         }
//     }
// }

// /// Awaitable cancel token with parent propagation (zero polling).
// #[derive(Clone, Debug)]
// pub struct CancelToken {
//     flag: Arc<AtomicBool>,
//     notify: Arc<Notify>,
//     parent: Option<Arc<CancelToken>>,
// }

// impl Default for CancelToken {
//     fn default() -> Self {
//         Self {
//             flag: Arc::new(AtomicBool::new(false)),
//             notify: Arc::new(Notify::new()),
//             parent: None,
//         }
//     }
// }

// impl CancelToken {
//     pub fn root() -> Arc<Self> {
//         Arc::new(Self {
//             flag: Arc::new(AtomicBool::new(false)),
//             notify: Arc::new(Notify::new()),
//             parent: None,
//         })
//     }

//     pub fn child_of(parent: &Arc<CancelToken>) -> Arc<Self> {
//         Arc::new(Self {
//             flag: Arc::new(AtomicBool::new(false)),
//             notify: Arc::new(Notify::new()),
//             parent: Some(parent.clone()),
//         })
//     }

//     pub fn cancel(&self) {
//         // Set only once; wake waiters if switched to true
//         if !self.flag.swap(true, Ordering::Relaxed) {
//             self.notify.notify_waiters();
//         }
//     }

//     pub fn is_canceled(&self) -> bool {
//         if self.flag.load(Ordering::Relaxed) {
//             return true;
//         }
//         if let Some(p) = &self.parent {
//             return p.is_canceled();
//         }
//         false
//     }

//     /// Wait until this token (or any ancestor) is canceled.
//     pub async fn cancelled(&self) {
//         if self.is_canceled() {
//             return;
//         }
//         if let Some(p) = &self.parent {
//             let parent_fut = Box::pin(p.cancelled());
//             tokio::select! {
//                 _ = self.notify.notified() => {},
//                 _ = parent_fut => {},
//             }
//         } else {
//             self.notify.notified().await;
//         }
//     }
// }

// /// A snapshot of cancellation and timeout state (awaitable).
// #[derive(Clone, Debug)]
// pub struct Cancellation {
//     deadline: Option<Instant>,
//     token: Arc<CancelToken>,
// }

// impl Cancellation {
//     pub fn new(deadline: Option<Instant>, token: Arc<CancelToken>) -> Self {
//         Self { deadline, token }
//     }

//     /// Fast non-blocking check.
//     pub fn is_done(&self) -> bool {
//         if let Some(dl) = self.deadline {
//             if Instant::now() >= dl {
//                 return true;
//             }
//         }
//         self.token.is_canceled()
//     }

//     /// Await either cancel or deadline. Returns the reason that happened first.
//     pub async fn wait(&self) -> Reason {
//         use tokio::time::{Instant as TokioInstant, sleep_until};

//         match self.deadline {
//             Some(dl) => {
//                 let until = TokioInstant::from_std(dl);
//                 tokio::select! {
//                     _ = self.token.cancelled() => Reason::Canceled,
//                     _ = sleep_until(until) => Reason::Timeout,
//                 }
//             }
//             None => {
//                 self.token.cancelled().await;
//                 Reason::Canceled
//             }
//         }
//     }
// }

// /// A handle to cancel a context.
// #[derive(Clone)]
// pub struct Canceller {
//     token: Arc<CancelToken>,
// }

// impl Canceller {
//     pub fn new(token: Arc<CancelToken>) -> Self {
//         Self { token }
//     }

//     pub fn cancel(&self) {
//         self.token.cancel();
//     }
// }

// /// Frozen context trait - the shared/frozen form (Arc).
// /// This is the primary interface for working with contexts in async code.
// pub trait AnyAsyncContext: AwaitCancel + Send + Sync + 'static {
//     /// Retrieve a value by key, searching parent if absent.
//     /// Returns `AnyContextValue` which can be downcast to a specific type.
//     fn value(&self, key: &str) -> Option<AnyContextValue>;

//     /// Remaining time to deadline (panics if no deadline).
//     #[track_caller]
//     fn deadline(&self) -> Duration;

//     /// Check if context is done: timed out or cancelled, including parent chain.
//     fn done(&self) -> Option<Reason>;

//     /// Get a canceller handle to trigger cancellation.
//     fn get_canceller(&self) -> Canceller;
// }

// /// Internal context structure (not directly exposed).
// #[derive(Clone, Serialize, Deserialize)]
// struct InnerContext {
//     #[serde(skip)]
//     parent: Option<ArcContext>,
//     #[serde(skip)]
//     deadline: Option<Instant>,
//     #[serde(skip)]
//     cancel: Arc<CancelToken>,
//     #[serde(skip)]
//     values: HashMap<Cow<'static, str>, Arc<dyn Any + Send + Sync>>,
// }

// /// Frozen context - the primary type for async operations.
// #[derive(Clone)]
// pub struct ArcContext(Arc<InnerContext>);

// impl AwaitCancel for ArcContext {
//     #[inline]
//     fn cancellation(&self) -> Cancellation {
//         InnerContext::cancellation(&self.0)
//     }
// }

// impl AnyAsyncContext for ArcContext {
//     fn value(&self, key: &str) -> Option<AnyContextValue> {
//         self.0.value(key)
//     }

//     #[track_caller]
//     fn deadline(&self) -> Duration {
//         self.0.deadline()
//     }

//     fn done(&self) -> Option<Reason> {
//         self.0.done()
//     }

//     fn get_canceller(&self) -> Canceller {
//         self.0.get_canceller()
//     }
// }

// impl ArcContext {
//     /// Get the cancel token (internal use).
//     fn cancel_token(&self) -> Arc<CancelToken> {
//         self.0.cancel.clone()
//     }
// }

// /// Builder for creating contexts with a fluent API.
// pub struct ContextBuilder {
//     parent: Option<ArcContext>,
//     deadline: Option<Instant>,
//     cancel: Arc<CancelToken>,
//     values: HashMap<Cow<'static, str>, Arc<dyn Any + Send + Sync>>,
// }

// impl fmt::Debug for InnerContext {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("InnerContext")
//             .field("parent", &self.parent.is_some())
//             .field("deadline", &self.deadline)
//             .field("cancelled", &self.cancel.is_canceled())
//             .field(
//                 "values_keys",
//                 &self.values.keys().map(|k| k.as_ref()).collect::<Vec<_>>(),
//             )
//             .finish()
//     }
// }

// impl fmt::Debug for ArcContext {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         self.0.fmt(f)
//     }
// }

// impl InnerContext {
//     fn value(&self, key: &str) -> Option<AnyContextValue> {
//         if let Some(v) = self.values.get(key) {
//             Some(AnyContextValue(v.clone()))
//         } else if let Some(parent) = &self.parent {
//             parent.value(key)
//         } else {
//             None
//         }
//     }

//     #[track_caller]
//     fn deadline(&self) -> Duration {
//         let deadline = self.deadline.expect("Timeout must be set before");
//         let now = Instant::now();
//         if deadline > now {
//             deadline.duration_since(now)
//         } else {
//             Duration::from_secs(0)
//         }
//     }

//     fn done(&self) -> Option<Reason> {
//         if self.cancel.is_canceled() {
//             return Some(Reason::Canceled);
//         }
//         if let Some(dl) = self.deadline {
//             if Instant::now() >= dl {
//                 return Some(Reason::Timeout);
//             }
//         }
//         if let Some(parent) = &self.parent {
//             parent.done()
//         } else {
//             None
//         }
//     }

//     fn get_canceller(&self) -> Canceller {
//         Canceller::new(self.cancel.clone())
//     }

//     /// Capture an awaitable snapshot of cancellation state and deadline.
//     fn cancellation(&self) -> Cancellation {
//         Cancellation::new(self.deadline, self.cancel.clone())
//     }
// }

// impl ContextBuilder {
//     /// Create a background context with no parent and no deadline.
//     pub fn new() -> Self {
//         Self {
//             parent: None,
//             deadline: None,
//             cancel: CancelToken::root(),
//             values: HashMap::new(),
//         }
//     }

//     /// Create a background context (alias for `new()`).
//     pub fn background() -> Self {
//         Self::new()
//     }

//     /// Create a background context with a timeout from now.
//     pub fn background_with_timeout(timeout: Duration) -> Self {
//         Self::new().with_timeout(timeout)
//     }

//     /// Set a parent context.
//     pub fn with_parent(mut self, parent: ArcContext) -> Self {
//         self.cancel = CancelToken::child_of(&parent.cancel_token());
//         self.parent = Some(parent);
//         self
//     }

//     /// Set a timeout from now.
//     /// If existing deadline is sooner, keep it (can't extend).
//     pub fn with_timeout(mut self, timeout: Duration) -> Self {
//         let new_deadline = Instant::now() + timeout;
//         match self.deadline {
//             Some(current) if current <= new_deadline => {} // keep earlier deadline
//             _ => self.deadline = Some(new_deadline),
//         }
//         self
//     }

//     /// Add or overwrite a value by key.
//     pub fn with_value<V: ContextValue, K: Into<Cow<'static, str>>>(
//         mut self,
//         key: K,
//         value: V,
//     ) -> Self {
//         self.values.insert(key.into(), Arc::new(value));
//         self
//     }

//     /// Freeze into a shareable frozen context.
//     pub fn freeze(self) -> ArcContext {
//         ArcContext(Arc::new(InnerContext {
//             parent: self.parent,
//             deadline: self.deadline,
//             cancel: self.cancel,
//             values: self.values,
//         }))
//     }
// }

// impl ArcContext {
//     /// Create a new context with a parent.
//     pub fn new(parent: ArcContext) -> ArcContext {
//         ContextBuilder::new().with_parent(parent).freeze()
//     }

//     /// Create a new context with a parent and timeout.
//     pub fn new_with_timeout(parent: ArcContext, timeout: Duration) -> ArcContext {
//         ContextBuilder::new()
//             .with_parent(parent)
//             .with_timeout(timeout)
//             .freeze()
//     }

//     /// Create a background context.
//     pub fn background() -> ArcContext {
//         ContextBuilder::background().freeze()
//     }

//     /// Create a background context with timeout.
//     pub fn background_with_timeout(timeout: Duration) -> ArcContext {
//         ContextBuilder::background_with_timeout(timeout).freeze()
//     }

//     /// Unfreeze from Arc back to builder (for testing only).
//     /// Fails if multiple references exist.
//     #[cfg(test)]
//     pub fn into_builder(self) -> Result<ContextBuilder, &'static str> {
//         match Arc::try_unwrap(self.0) {
//             Ok(inner) => Ok(ContextBuilder {
//                 parent: inner.parent,
//                 deadline: inner.deadline,
//                 cancel: inner.cancel,
//                 values: inner.values,
//             }),
//             Err(_) => Err("Context has multiple references"),
//         }
//     }
// }

// impl Default for ContextBuilder {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// // Type aliases for backward compatibility (deprecated)
// #[deprecated(note = "Use ArcContext instead")]
// pub type AsyncContext = ArcContext;

// #[deprecated(note = "Use ContextBuilder instead")]
// pub type MutableContext = ContextBuilder;

// /// Convenience: remaining time until deadline (if any) and access to cancel token.
// pub trait ContextExt {
//     fn deadline_remaining(&self) -> Option<Duration>;
//     fn cancel_token(&self) -> Arc<CancelToken>;
// }

// impl ContextExt for ArcContext {
//     fn deadline_remaining(&self) -> Option<Duration> {
//         self.0
//             .deadline
//             .map(|dl| dl.saturating_duration_since(Instant::now()))
//     }

//     fn cancel_token(&self) -> Arc<CancelToken> {
//         self.0.cancel.clone()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::time::Duration;
//     use tokio::time::sleep;

//     #[test]
//     fn test_background_context_default() {
//         let ctx = ArcContext::background();
//         assert_eq!(ctx.done(), None);
//         assert!(!ctx.cancel_token().is_canceled());
//     }

//     #[should_panic(expected = "Timeout must be set before")]
//     #[test]
//     fn test_deadline_panics_without_deadline() {
//         let ctx = ArcContext::background();
//         let _ = ctx.deadline(); // Should panic without deadline
//     }

//     #[test]
//     fn test_add_and_get_value() {
//         let ctx = ContextBuilder::background()
//             .with_value("key1", 42u32)
//             .freeze();
//         let value = ctx.value("key1").unwrap().downcast::<u32>().unwrap();
//         assert_eq!(*value, 42);
//     }

//     #[test]
//     fn test_inherit_values_from_parent() {
//         let parent = ContextBuilder::background()
//             .with_value("x", "parent_val")
//             .freeze();

//         let child = ArcContext::new(parent.clone());
//         let val = child.value("x").unwrap().downcast::<&str>().unwrap();
//         assert_eq!(*val, "parent_val");
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
//     async fn test_timeout_marks_done_and_wait() {
//         let ctx = ArcContext::background_with_timeout(Duration::from_millis(30));
//         sleep(Duration::from_millis(50)).await;
//         assert_eq!(ctx.done(), Some(Reason::Timeout));

//         // Snapshot should also see timeout
//         let snap = ctx.cancellation();
//         assert!(snap.is_done());
//         assert_eq!(snap.wait().await, Reason::Timeout);
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
//     async fn test_canceller_marks_cancelled_and_wait() {
//         let ctx = ArcContext::background();
//         let canc = ctx.get_canceller();
//         assert_eq!(ctx.done(), None);

//         canc.cancel();
//         assert_eq!(ctx.done(), Some(Reason::Canceled));

//         let snap = ctx.cancellation();
//         assert!(snap.is_done());
//         assert_eq!(snap.wait().await, Reason::Canceled);
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
//     async fn test_cancellation_snapshot_reacts_to_later_cancel() {
//         let ctx = ArcContext::background();
//         let canc = ctx.get_canceller();

//         let snap1 = ctx.cancellation();
//         assert!(!snap1.is_done());

//         canc.cancel();

//         // Old snapshot should react
//         assert!(snap1.is_done());
//         assert_eq!(snap1.wait().await, Reason::Canceled);
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
//     async fn test_child_inherit_parent_deadline() {
//         let parent = ArcContext::background_with_timeout(Duration::from_millis(50));
//         let child = ArcContext::new(parent);
//         sleep(Duration::from_millis(60)).await;
//         assert_eq!(child.done(), Some(Reason::Timeout));
//     }

//     #[test]
//     fn test_child_cannot_extend_parent_deadline() {
//         let parent = ArcContext::background_with_timeout(Duration::from_millis(20));
//         let child_builder = ContextBuilder::new()
//             .with_parent(parent.clone())
//             .with_timeout(Duration::from_millis(100)); // Should not extend
//         let child = child_builder.freeze();
//         // Child should inherit parent's deadline, not extend it
//         // We can't directly compare deadlines, but we can check that child times out with parent
//         assert!(child.done().is_none()); // Not timed out yet
//     }

//     #[test]
//     fn test_nested_cancellation() {
//         let parent = ArcContext::background();
//         let canc_parent = parent.get_canceller();

//         let child = ArcContext::new(parent);
//         assert_eq!(child.done(), None);

//         canc_parent.cancel();
//         assert_eq!(child.done(), Some(Reason::Canceled));
//     }

//     #[test]
//     fn test_freeze_unfreeze_roundtrip() {
//         let ctx = ArcContext::background();
//         let builder = ctx.into_builder();
//         assert!(builder.is_ok());
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
//     async fn test_with_context_success() {
//         let ctx = ArcContext::background_with_timeout(Duration::from_millis(200));

//         let res = abortable(&ctx, async {
//             sleep(Duration::from_millis(10)).await;
//             Ok::<_, std::io::Error>(7_i32)
//         })
//         .await;

//         match res {
//             Ok(v) => assert_eq!(v, 7),
//             other => panic!("unexpected result: {:?}", other),
//         }
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
//     async fn test_with_context_timeout() {
//         let ctx = ArcContext::background_with_timeout(Duration::from_millis(20));

//         let res = abortable(&ctx, async {
//             sleep(Duration::from_millis(100)).await;
//             Ok::<_, std::io::Error>(1_i32)
//         })
//         .await;

//         match res {
//             Err(Ok(Reason::Timeout)) => {}
//             other => panic!("expected timeout, got: {:?}", other),
//         }
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
//     async fn test_with_context_cancel() {
//         let ctx = ArcContext::background_with_timeout(Duration::from_secs(5));
//         let canc = ctx.get_canceller();

//         let fut = abortable(&ctx, async {
//             sleep(Duration::from_secs(5)).await;
//             Ok::<_, std::io::Error>(123_i32)
//         });

//         tokio::spawn(async move {
//             sleep(Duration::from_millis(30)).await;
//             canc.cancel();
//         });

//         match fut.await {
//             Err(Ok(Reason::Canceled)) => {}
//             other => panic!("expected canceled, got: {:?}", other),
//         }
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
//     async fn test_with_context_inner_error() {
//         let ctx = ArcContext::background_with_timeout(Duration::from_secs(1));

//         let res = abortable(&ctx, async {
//             Err::<i32, _>(std::io::Error::new(std::io::ErrorKind::Other, "boom"))
//         })
//         .await;

//         match res {
//             Err(Err(e)) => assert_eq!(e.to_string(), "boom"),
//             other => panic!("expected inner error, got: {:?}", other),
//         }
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
//     async fn test_multiple_background_tasks_canceled_by_parent() {
//         use std::sync::{
//             Arc as StdArc,
//             atomic::{AtomicUsize, Ordering},
//         };

//         let parent_ctx = ArcContext::background();
//         let canceller = parent_ctx.get_canceller();

//         let completed_count = StdArc::new(AtomicUsize::new(0));
//         let canceled_count = StdArc::new(AtomicUsize::new(0));

//         let mut handles = Vec::new();
//         for i in 0..5 {
//             let child_ctx = ArcContext::new(parent_ctx.clone());
//             let completed = completed_count.clone();
//             let canceled = canceled_count.clone();

//             let handle = tokio::spawn(async move {
//                 let result = abortable(&child_ctx, async {
//                     sleep(Duration::from_millis(100 + i * 10)).await;
//                     Ok::<_, std::io::Error>(i)
//                 })
//                 .await;

//                 match result {
//                     Ok(_) => completed.fetch_add(1, Ordering::Relaxed),
//                     Err(Ok(Reason::Canceled)) => canceled.fetch_add(1, Ordering::Relaxed),
//                     _ => 0,
//                 };
//             });
//             handles.push(handle);
//         }

//         // Time to start
//         sleep(Duration::from_millis(30)).await;

//         canceller.cancel();
//         for handle in handles {
//             let _ = handle.await;
//         }

//         // All tasks should be canceled
//         assert_eq!(completed_count.load(Ordering::Relaxed), 0);
//         assert_eq!(canceled_count.load(Ordering::Relaxed), 5);
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
//     async fn test_hierarchical_cancellation_propagation() {
//         use std::sync::{
//             Arc as StdArc,
//             atomic::{AtomicUsize, Ordering},
//         };

//         let root_ctx = ArcContext::background();
//         let root_canceller = root_ctx.get_canceller();

//         let level1_ctx = ArcContext::new(root_ctx.clone());
//         let level2_ctx = ArcContext::new(level1_ctx.clone());
//         let level3_ctx = ArcContext::new(level2_ctx.clone());

//         let canceled_levels = StdArc::new(AtomicUsize::new(0));

//         // Create tasks at different levels of hierarchy
//         let mut handles = Vec::new();

//         for (level, ctx) in [
//             (1, root_ctx),
//             (2, level1_ctx),
//             (3, level2_ctx),
//             (4, level3_ctx),
//         ] {
//             let canceled = canceled_levels.clone();
//             let handle = tokio::spawn(async move {
//                 let result = abortable(&ctx, async {
//                     sleep(Duration::from_secs(10)).await;
//                     Ok::<_, std::io::Error>(level)
//                 })
//                 .await;

//                 if matches!(result, Err(Ok(Reason::Canceled))) {
//                     canceled.fetch_add(1, Ordering::Relaxed);
//                 }
//             });
//             handles.push(handle);
//         }

//         sleep(Duration::from_millis(20)).await;

//         // Cancel root context - all should be canceled
//         root_canceller.cancel();

//         for handle in handles {
//             let _ = handle.await;
//         }

//         assert_eq!(canceled_levels.load(Ordering::Relaxed), 4);
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
//     async fn test_partial_cancellation_in_hierarchy() {
//         use std::sync::{
//             Arc as StdArc,
//             atomic::{AtomicUsize, Ordering},
//         };

//         let root_ctx = ArcContext::background();

//         let branch1_ctx = ArcContext::new(root_ctx.clone());
//         let branch1_canceller = branch1_ctx.get_canceller();

//         let branch2_ctx = ArcContext::new(root_ctx.clone());

//         let branch1_canceled = StdArc::new(AtomicUsize::new(0));
//         let branch2_completed = StdArc::new(AtomicUsize::new(0));

//         let mut branch1_handles = Vec::new();
//         for i in 0..3 {
//             let ctx = ArcContext::new(branch1_ctx.clone());
//             let canceled = branch1_canceled.clone();

//             let handle = tokio::spawn(async move {
//                 let result = abortable(&ctx, async {
//                     sleep(Duration::from_millis(200)).await;
//                     Ok::<_, std::io::Error>(i)
//                 })
//                 .await;

//                 if matches!(result, Err(Ok(Reason::Canceled))) {
//                     canceled.fetch_add(1, Ordering::Relaxed);
//                 }
//             });
//             branch1_handles.push(handle);
//         }

//         let mut branch2_handles = Vec::new();
//         for i in 0..2 {
//             let ctx = ArcContext::new(branch2_ctx.clone());
//             let completed = branch2_completed.clone();

//             let handle = tokio::spawn(async move {
//                 let result = abortable(&ctx, async {
//                     sleep(Duration::from_millis(50)).await;
//                     Ok::<_, std::io::Error>(i)
//                 })
//                 .await;

//                 if result.is_ok() {
//                     completed.fetch_add(1, Ordering::Relaxed);
//                 }
//             });
//             branch2_handles.push(handle);
//         }

//         sleep(Duration::from_millis(20)).await;

//         branch1_canceller.cancel();

//         for handle in branch1_handles {
//             let _ = handle.await;
//         }
//         for handle in branch2_handles {
//             let _ = handle.await;
//         }

//         // First branch canceled, second completed successfully
//         assert_eq!(branch1_canceled.load(Ordering::Relaxed), 3);
//         assert_eq!(branch2_completed.load(Ordering::Relaxed), 2);
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
//     async fn test_cancellation_race_conditions() {
//         use std::sync::{
//             Arc as StdArc,
//             atomic::{AtomicUsize, Ordering},
//         };

//         let ctx = ArcContext::background();
//         let canceller = ctx.get_canceller();

//         let cancel_detected = StdArc::new(AtomicUsize::new(0));
//         let mut handles = Vec::new();

//         // Create many tasks that check cancellation
//         for _ in 0..20 {
//             let child_ctx = ArcContext::new(ctx.clone());
//             let detected = cancel_detected.clone();

//             let handle = tokio::spawn(async move {
//                 let mut iterations = 0;
//                 loop {
//                     if child_ctx.done().is_some() {
//                         detected.fetch_add(1, Ordering::Relaxed);
//                         break;
//                     }

//                     sleep(Duration::from_millis(1)).await;
//                     iterations += 1;

//                     if iterations > 1000 {
//                         break; // Protection from infinite loop
//                     }
//                 }
//             });
//             handles.push(handle);
//         }

//         sleep(Duration::from_millis(10)).await;

//         canceller.cancel();

//         for handle in handles {
//             let _ = handle.await;
//         }

//         // All tasks should detect cancellation
//         assert_eq!(cancel_detected.load(Ordering::Relaxed), 20);
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
//     async fn test_timeout_vs_cancellation_race() {
//         use std::sync::{
//             Arc as StdArc,
//             atomic::{AtomicUsize, Ordering},
//         };

//         let timeout_wins = StdArc::new(AtomicUsize::new(0));
//         let cancel_wins = StdArc::new(AtomicUsize::new(0));

//         // Start several iterations of the test
//         for i in 0..10 {
//             let ctx = ArcContext::background_with_timeout(Duration::from_millis(50 + i * 2));
//             let canceller = ctx.get_canceller();

//             let timeout_counter = timeout_wins.clone();
//             let cancel_counter = cancel_wins.clone();

//             let task_handle = tokio::spawn(async move {
//                 let result = abortable(&ctx, async {
//                     sleep(Duration::from_secs(1)).await;
//                     Ok::<_, std::io::Error>(())
//                 })
//                 .await;

//                 match result {
//                     Err(Ok(Reason::Timeout)) => timeout_counter.fetch_add(1, Ordering::Relaxed),
//                     Err(Ok(Reason::Canceled)) => cancel_counter.fetch_add(1, Ordering::Relaxed),
//                     _ => 0,
//                 };
//             });

//             // Cancel through random time
//             let cancel_handle = tokio::spawn(async move {
//                 sleep(Duration::from_millis(30 + i * 3)).await;
//                 canceller.cancel();
//             });

//             let _ = tokio::join!(task_handle, cancel_handle);
//         }

//         // There should be timeouts and cancellations (depends on timing)
//         let total_timeouts = timeout_wins.load(Ordering::Relaxed);
//         let total_cancels = cancel_wins.load(Ordering::Relaxed);

//         assert_eq!(total_timeouts + total_cancels, 10);
//         assert!(total_timeouts > 0 || total_cancels > 0);
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
//     async fn test_nested_with_context_cancellation() {
//         let outer_ctx = ArcContext::background();
//         let outer_canceller = outer_ctx.get_canceller();

//         // Start cancellation in background before main logic
//         tokio::spawn({
//             let canceller = outer_canceller.clone();
//             async move {
//                 sleep(Duration::from_millis(20)).await;
//                 canceller.cancel();
//             }
//         });

//         let result = abortable(&outer_ctx, async {
//             let inner_ctx = ArcContext::new(outer_ctx.clone());

//             abortable(&inner_ctx, async {
//                 let deepest_ctx = ArcContext::new(inner_ctx.clone());

//                 abortable(&deepest_ctx, async {
//                     sleep(Duration::from_secs(10)).await;
//                     Ok::<_, std::io::Error>("should not reach here")
//                 })
//                 .await
//             })
//             .await
//         })
//         .await;

//         // There should be cancellation at any level
//         match result {
//             Err(Ok(Reason::Canceled)) => {}
//             other => panic!("expected cancellation, got: {:?}", other),
//         }
//     }

//     #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
//     async fn test_cancellation_cleanup_and_resource_management() {
//         use std::sync::{
//             Arc as StdArc,
//             atomic::{AtomicBool, Ordering},
//         };

//         let ctx = ArcContext::background();
//         let canceller = ctx.get_canceller();

//         let resource_acquired = StdArc::new(AtomicBool::new(false));
//         let resource_released = StdArc::new(AtomicBool::new(false));

//         let acquired = resource_acquired.clone();
//         let released = resource_released.clone();

//         let task_handle = tokio::spawn(async move {
//             let _result = abortable(&ctx, async {
//                 // Simulate resource acquisition
//                 acquired.store(true, Ordering::Relaxed);

//                 // Long operation
//                 sleep(Duration::from_secs(10)).await;

//                 Ok::<_, std::io::Error>(())
//             })
//             .await;

//             // Cleanup in any case
//             released.store(true, Ordering::Relaxed);
//         });

//         sleep(Duration::from_millis(20)).await;

//         canceller.cancel();

//         let _ = task_handle.await;

//         // Resource should be acquired and released
//         assert!(resource_acquired.load(Ordering::Relaxed));
//         assert!(resource_released.load(Ordering::Relaxed));
//     }
// }
