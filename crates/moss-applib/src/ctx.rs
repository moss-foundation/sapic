use std::{
    any::Any,
    borrow::Cow,
    collections::HashMap,
    fmt,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant as StdInstant},
};

pub type Context = Arc<MutableContext>;

#[derive(Clone)]
pub struct MutableContext {
    parent: Option<Context>,
    deadline: Option<StdInstant>,
    cancelled: Arc<AtomicBool>,
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

    /// Freeze into an Arc for sharing.
    pub fn freeze(self) -> Context {
        Arc::new(self)
    }

    /// Unfreeze from Arc back to owned. Fails if multiple references exist.
    pub fn unfreeze(ctx: Context) -> Result<Self, &'static str> {
        match Arc::try_unwrap(ctx) {
            Ok(inner) => Ok(inner),
            Err(_) => Err("Context has multiple references"),
        }
    }

    /// Create a new child context inheriting deadline and values.
    pub fn new(parent: &Context) -> Self {
        MutableContext {
            parent: Some(parent.clone()),
            deadline: parent.deadline,
            cancelled: Arc::new(AtomicBool::new(false)),
            values: HashMap::new(),
        }
    }

    /// Add or overwrite a value by key.
    pub fn with_value<V: Send + Sync + 'static, K: Into<Cow<'static, str>>>(
        &mut self,
        key: K,
        value: V,
    ) {
        self.values.insert(key.into(), Arc::new(value));
    }

    /// Retrieve a value by key, searching parent if absent.
    pub fn value<V: Send + Sync + 'static>(&self, key: &str) -> Option<Arc<V>> {
        if let Some(v) = self.values.get(key) {
            v.clone().downcast::<V>().ok()
        } else if let Some(parent) = &self.parent {
            parent.value::<V>(key)
        } else {
            None
        }
    }

    /// Add a deadline as a timeout from now.
    pub fn with_timeout(&mut self, timeout: Duration) {
        let new_deadline = StdInstant::now() + timeout;
        match self.deadline {
            Some(current) if current <= new_deadline => {}
            _ => self.deadline = Some(new_deadline),
        }
    }

    /// Return a canceller to cancel this context.
    pub fn add_cancel(&mut self) -> Canceller {
        Canceller::new(self.cancelled.clone())
    }

    /// Check if context is done: timed out or cancelled, including parent chain.
    pub fn done(&self) -> Option<Reason> {
        if self.cancelled.load(Ordering::Relaxed) {
            return Some(Reason::Canceled);
        }
        if let Some(dl) = self.deadline {
            if StdInstant::now() >= dl {
                return Some(Reason::Timedout);
            }
        }
        if let Some(parent) = &self.parent {
            parent.done()
        } else {
            None
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

/// A snapshot of cancellation and timeout state.
#[derive(Clone, Debug)]
pub struct Cancellation {
    deadline: Option<StdInstant>,
    cancellations: Vec<Arc<AtomicBool>>,
}

impl Cancellation {
    /// Create a new snapshot.
    pub fn new(deadline: Option<StdInstant>, cancellations: Vec<Arc<AtomicBool>>) -> Self {
        Cancellation {
            deadline,
            cancellations,
        }
    }

    /// Check if cancelled or timed out.
    pub fn is_done(&self) -> bool {
        if let Some(dl) = self.deadline {
            if StdInstant::now() >= dl {
                return true;
            }
        }
        self.cancellations.iter().any(|c| c.load(Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time::Duration};

    #[test]
    fn test_background_context_default() {
        let ctx = MutableContext::background();
        assert!(ctx.parent.is_none());
        assert!(ctx.deadline.is_none());
        assert!(!ctx.cancelled.load(Ordering::Relaxed));
        assert!(ctx.values.is_empty());
        assert_eq!(ctx.done(), None);
    }

    #[test]
    fn test_add_and_get_value() {
        let mut ctx = MutableContext::background();
        ctx.with_value("key1", 42u32);
        let value = ctx.value::<u32>("key1");
        assert_eq!(*value.unwrap(), 42);
    }

    #[test]
    fn test_inherit_values_from_parent() {
        let mut parent = MutableContext::background();
        parent.with_value("x", "parent_val");
        let parent = parent.freeze();

        let child = MutableContext::new(&parent);
        let val: Arc<&str> = child.value("x").unwrap();
        assert_eq!(*val, "parent_val");
    }

    #[test]
    fn test_timeout_marks_done() {
        let mut ctx = MutableContext::background();
        ctx.with_timeout(Duration::from_millis(10));
        thread::sleep(Duration::from_millis(20));
        assert_eq!(ctx.done(), Some(Reason::Timedout));
    }

    #[test]
    fn test_canceller_marks_cancelled() {
        let mut ctx = MutableContext::background();
        let canc = ctx.add_cancel();
        assert_eq!(ctx.done(), None);
        canc.cancel();
        assert_eq!(ctx.done(), Some(Reason::Canceled));
    }

    #[test]
    fn test_cancellation_snapshot() {
        let mut ctx = MutableContext::background();
        let canc = ctx.add_cancel();
        let snap1 = ctx.cancellation();
        assert!(!snap1.is_done());
        canc.cancel();
        let snap2 = ctx.cancellation();
        assert!(snap2.is_done());
    }

    #[test]
    fn test_child_inherit_parent_deadline() {
        let mut parent_ctx = MutableContext::background();
        parent_ctx.with_timeout(Duration::from_millis(50));
        let parent = parent_ctx.freeze();
        let child = MutableContext::new(&parent);
        thread::sleep(Duration::from_millis(60));
        assert_eq!(child.done(), Some(Reason::Timedout));
    }

    #[test]
    fn test_child_cannot_extend_parent_deadline() {
        let mut parent_ctx = MutableContext::background();
        parent_ctx.with_timeout(Duration::from_millis(20));
        let parent = parent_ctx.freeze();
        let mut child = MutableContext::new(&parent);
        child.with_timeout(Duration::from_millis(100));
        assert_eq!(child.deadline, parent.deadline);
    }

    #[test]
    fn test_nested_cancellation() {
        let mut parent_ctx = MutableContext::background();
        let canc_parent = parent_ctx.add_cancel();
        let parent = parent_ctx.freeze();
        let child = MutableContext::new(&parent);
        assert_eq!(child.done(), None);
        canc_parent.cancel();
        assert_eq!(child.done(), Some(Reason::Canceled));
    }
}
