use std::{
    any::Any,
    borrow::Cow,
    collections::HashMap,
    fmt::{self, Display},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

pub trait ContextValue: Any + Send + Sync + 'static {}

impl ContextValue for u32 {}
impl ContextValue for &'static str {}
impl ContextValue for bool {}
impl ContextValue for i64 {}
impl ContextValue for f64 {}
impl ContextValue for String {}

pub trait AnyContext: WithCanceller {
    type Frozen: AnyAsyncContext;

    fn freeze(self) -> Self::Frozen;

    /// Add or overwrite a value by key.
    fn with_value<V: ContextValue, K: Into<Cow<'static, str>>>(&mut self, key: K, value: V);

    /// Retrieve a value by key, searching parent if absent.
    fn value<V: ContextValue>(&self, key: &str) -> Option<Arc<V>>;

    fn deadline(&self) -> Duration;

    /// Check if context is done: timed out or cancelled, including parent chain.
    fn done(&self) -> Option<Reason>;
}

pub trait AnyAsyncContext: Clone + Send + Sync + 'static {
    type Unfrozen: AnyContext;

    fn background() -> Self::Unfrozen;
    fn background_with_timeout(timeout: Duration) -> Self::Unfrozen;

    fn new(parent: Self) -> Self::Unfrozen;
    fn new_with_timeout(parent: Self, timeout: Duration) -> Self::Unfrozen;

    fn unfreeze(self) -> Result<Self::Unfrozen, &'static str>;

    fn deadline(&self) -> Duration;
    fn value<V: ContextValue>(&self, key: &str) -> Option<Arc<V>>;
    fn done(&self) -> Option<Reason>;
}

pub trait WithCanceller {
    fn get_canceller(&self) -> Canceller;
}

pub type AsyncContext = Arc<MutableContext>;

impl AnyAsyncContext for AsyncContext {
    type Unfrozen = MutableContext;

    fn background() -> Self::Unfrozen {
        MutableContext::background()
    }

    fn background_with_timeout(timeout: Duration) -> Self::Unfrozen {
        MutableContext {
            parent: None,
            deadline: Some(Instant::now() + timeout),
            cancelled: Arc::new(AtomicBool::new(false)),
            values: HashMap::new(),
        }
    }

    #[track_caller]
    fn deadline(&self) -> Duration {
        let deadline = self.deadline.expect("Timeout must be set before");

        let now = Instant::now();
        if deadline > now {
            deadline.duration_since(now)
        } else {
            Duration::from_secs(0)
        }
    }

    fn value<V: ContextValue>(&self, key: &str) -> Option<Arc<V>> {
        if let Some(v) = self.values.get(key) {
            v.clone().downcast::<V>().ok()
        } else if let Some(parent) = &self.parent {
            parent.value::<V>(key)
        } else {
            None
        }
    }

    fn done(&self) -> Option<Reason> {
        if self.cancelled.load(Ordering::Relaxed) {
            return Some(Reason::Canceled);
        }
        if let Some(dl) = self.deadline {
            if Instant::now() >= dl {
                return Some(Reason::Timedout);
            }
        }
        if let Some(parent) = &self.parent {
            parent.done()
        } else {
            None
        }
    }

    fn new(parent: Self) -> Self::Unfrozen {
        MutableContext {
            parent: Some(parent),
            deadline: None,
            cancelled: Arc::new(AtomicBool::new(false)),
            values: HashMap::new(),
        }
    }

    fn new_with_timeout(parent: Self, timeout: Duration) -> Self::Unfrozen {
        MutableContext {
            parent: Some(parent),
            deadline: Some(Instant::now() + timeout),
            cancelled: Arc::new(AtomicBool::new(false)),
            values: HashMap::new(),
        }
    }

    fn unfreeze(self) -> Result<Self::Unfrozen, &'static str> {
        match Arc::try_unwrap(self) {
            Ok(inner) => Ok(inner),
            Err(_) => Err("Context has multiple references"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MutableContext {
    parent: Option<AsyncContext>,
    #[serde(skip)]
    deadline: Option<Instant>,
    cancelled: Arc<AtomicBool>,
    #[serde(skip)]
    values: HashMap<Cow<'static, str>, Arc<dyn Any + Send + Sync>>,
}

impl Default for MutableContext {
    fn default() -> Self {
        MutableContext::background()
    }
}

impl fmt::Debug for MutableContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("parent", &self.parent)
            .field("deadline", &self.deadline)
            .field("cancelled", &self.cancelled.load(Ordering::Relaxed))
            .field("values_keys", &self.values.keys())
            .finish()
    }
}

impl WithCanceller for MutableContext {
    fn get_canceller(&self) -> Canceller {
        Canceller::new(self.cancelled.clone())
    }
}

impl AnyContext for MutableContext {
    type Frozen = AsyncContext;

    fn with_value<V: ContextValue, K: Into<Cow<'static, str>>>(&mut self, key: K, value: V) {
        self.values.insert(key.into(), Arc::new(value));
    }

    #[track_caller]
    fn deadline(&self) -> Duration {
        let deadline = self.deadline.expect("Timeout must be set before");

        let now = Instant::now();
        if deadline > now {
            deadline.duration_since(now)
        } else {
            Duration::from_secs(0)
        }
    }

    fn value<V: ContextValue>(&self, key: &str) -> Option<Arc<V>> {
        if let Some(v) = self.values.get(key) {
            v.clone().downcast::<V>().ok()
        } else if let Some(parent) = &self.parent {
            parent.value::<V>(key)
        } else {
            None
        }
    }

    fn done(&self) -> Option<Reason> {
        if self.cancelled.load(Ordering::Relaxed) {
            return Some(Reason::Canceled);
        }
        if let Some(dl) = self.deadline {
            if Instant::now() >= dl {
                return Some(Reason::Timedout);
            }
        }
        if let Some(parent) = &self.parent {
            parent.done()
        } else {
            None
        }
    }

    fn freeze(self) -> Self::Frozen {
        Arc::new(self)
    }

    // fn freeze(self) -> Self::Frozen {
    //     Arc::new(self)
    // }
}

impl From<&AsyncContext> for MutableContext {
    fn from(parent: &AsyncContext) -> Self {
        Self {
            parent: Some(parent.clone()),
            deadline: parent.deadline,
            cancelled: Arc::new(AtomicBool::new(false)),
            values: HashMap::new(),
        }
    }
}

impl MutableContext {
    /// Create a background context with no parent and no deadline.
    pub fn background() -> Self {
        MutableContext {
            parent: None,
            deadline: None,
            cancelled: Arc::new(AtomicBool::new(false)),
            values: HashMap::new(),
        }
    }

    pub fn background_with_timeout(timeout: Duration) -> Self {
        Self {
            parent: None,
            deadline: Some(Instant::now() + timeout),
            cancelled: Arc::new(AtomicBool::new(false)),
            values: HashMap::new(),
        }
    }

    pub fn new_with_timeout(parent: AsyncContext, timeout: Duration) -> Self {
        Self {
            parent: Some(parent),
            deadline: Some(Instant::now() + timeout),
            cancelled: Arc::new(AtomicBool::new(false)),
            values: HashMap::new(),
        }
    }

    /// Freeze into an Arc for sharing.
    pub fn freeze(self) -> AsyncContext {
        Arc::new(self)
    }

    /// Unfreeze from Arc back to owned. Fails if multiple references exist.
    pub fn unfreeze(ctx: AsyncContext) -> Result<Self, &'static str> {
        match Arc::try_unwrap(ctx) {
            Ok(inner) => Ok(inner),
            Err(_) => Err("Context has multiple references"),
        }
    }

    /// Add a deadline as a timeout from now.
    pub fn set_timeout(&mut self, timeout: Duration) {
        let new_deadline = Instant::now() + timeout;
        match self.deadline {
            Some(current) if current <= new_deadline => {}
            _ => self.deadline = Some(new_deadline),
        }
    }

    /// Capture a snapshot of cancellation state and deadline.
    pub fn cancellation(&self) -> Cancellation {
        // Gather all atomic bools up the parent chain
        let mut cancels = Vec::new();
        let mut current: Option<&MutableContext> = Some(self);
        while let Some(ctx) = current {
            cancels.push(ctx.cancelled.clone());
            current = ctx.parent.as_deref();
        }
        Cancellation::new(self.deadline, cancels)
    }
}

/// A handle to cancel a context.
#[derive(Clone)]
pub struct Canceller {
    cancelled: Arc<AtomicBool>,
}

impl Canceller {
    pub fn new(cancelled: Arc<AtomicBool>) -> Self {
        Canceller { cancelled }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }
}

/// Reasons why a context is done.
#[derive(Debug, PartialEq, Eq)]
pub enum Reason {
    Timedout,
    Canceled,
}

impl Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A snapshot of cancellation and timeout state.
#[derive(Clone, Debug)]
pub struct Cancellation {
    deadline: Option<Instant>,
    cancellations: Vec<Arc<AtomicBool>>,
}

impl Cancellation {
    /// Create a new snapshot.
    pub fn new(deadline: Option<Instant>, cancellations: Vec<Arc<AtomicBool>>) -> Self {
        Cancellation {
            deadline,
            cancellations,
        }
    }

    /// Check if cancelled or timed out.
    pub fn is_done(&self) -> bool {
        if let Some(dl) = self.deadline {
            if Instant::now() >= dl {
                return true;
            }
        }
        self.cancellations.iter().any(|c| c.load(Ordering::Relaxed))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::{thread, time::Duration};

//     #[test]
//     fn test_background_context_default() {
//         let ctx = MutableContext::new();
//         assert!(ctx.parent.is_none());
//         assert!(ctx.deadline.is_none());
//         assert!(!ctx.cancelled.load(Ordering::Relaxed));
//         assert!(ctx.values.is_empty());
//         assert_eq!(ctx.done(), None);
//     }

//     #[test]
//     fn test_add_and_get_value() {
//         let mut ctx = MutableContext::new();
//         ctx.with_value("key1", 42u32);
//         let value = ctx.value::<u32>("key1");
//         assert_eq!(*value.unwrap(), 42);
//     }

//     #[test]
//     fn test_inherit_values_from_parent() {
//         let mut parent = MutableContext::new();
//         parent.with_value("x", "parent_val");
//         let parent = parent.freeze();

//         let child = MutableContext::from(&parent);
//         let val: Arc<&str> = child.value("x").unwrap();
//         assert_eq!(*val, "parent_val");
//     }

//     #[test]
//     fn test_timeout_marks_done() {
//         let ctx = MutableContext::with_timeout(Duration::from_millis(10));
//         thread::sleep(Duration::from_millis(20));
//         assert_eq!(ctx.done(), Some(Reason::Timedout));
//     }

//     #[test]
//     fn test_canceller_marks_cancelled() {
//         let mut ctx = MutableContext::new();
//         let canc = ctx.add_cancel();
//         assert_eq!(ctx.done(), None);
//         canc.cancel();
//         assert_eq!(ctx.done(), Some(Reason::Canceled));
//     }

//     #[test]
//     fn test_cancellation_snapshot() {
//         let mut ctx = MutableContext::new();
//         let canc = ctx.add_cancel();
//         let snap1 = ctx.cancellation();
//         assert!(!snap1.is_done());
//         canc.cancel();
//         let snap2 = ctx.cancellation();
//         assert!(snap2.is_done());
//     }

//     #[test]
//     fn test_child_inherit_parent_deadline() {
//         let parent_ctx = MutableContext::with_timeout(Duration::from_millis(50));
//         let parent = parent_ctx.freeze();
//         let child = MutableContext::from(&parent);
//         thread::sleep(Duration::from_millis(60));
//         assert_eq!(child.done(), Some(Reason::Timedout));
//     }

//     #[test]
//     fn test_child_cannot_extend_parent_deadline() {
//         let parent_ctx = MutableContext::with_timeout(Duration::from_millis(20));
//         let parent = parent_ctx.freeze();
//         let mut child = MutableContext::from(&parent);
//         child.set_timeout(Duration::from_millis(100));
//         assert_eq!(child.deadline, parent.deadline);
//     }

//     #[test]
//     fn test_nested_cancellation() {
//         let mut parent_ctx = MutableContext::new();
//         let canc_parent = parent_ctx.add_cancel();
//         let parent = parent_ctx.freeze();
//         let child = MutableContext::from(&parent);
//         assert_eq!(child.done(), None);
//         canc_parent.cancel();
//         assert_eq!(child.done(), Some(Reason::Canceled));
//     }
// }
