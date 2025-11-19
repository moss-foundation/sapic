// TODO: should be moved to core crate

use futures::{FutureExt, StreamExt, future::BoxFuture, stream::FuturesUnordered};
use rustc_hash::FxHashMap;
use std::{
    any::TypeId,
    panic::AssertUnwindSafe,
    sync::{Arc, Weak},
};
use tokio::sync::{Mutex, RwLock, mpsc};
use tokio_util::sync::CancellationToken;

use crate::EventMarker;

pub type ListenerFuture = BoxFuture<'static, ()>;
type ListenerCallback<T> = Arc<dyn Fn(T) -> ListenerFuture + Send + Sync + 'static>;

#[derive(Clone)]
pub struct Listener<T> {
    pub id: usize,
    pub callback: ListenerCallback<T>,
    pub limit: Option<u64>,
    pub cancellation_token: CancellationToken,
}

pub struct Subscription<T: EventMarker> {
    id: usize,
    emitter: Weak<RwLock<EventEmitterState<T>>>,
    cancellation_token: CancellationToken,
    _unsubscribe_tx: mpsc::UnboundedSender<usize>,
}

impl<T: EventMarker> Subscription<T> {
    pub fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    pub async fn unsubscribe(&self) {
        self.cancellation_token.cancel();

        if let Some(state) = self.emitter.upgrade() {
            let mut state_lock = state.write().await;
            state_lock.listeners.remove(&self.id);
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

impl<T: EventMarker> Drop for Subscription<T> {
    fn drop(&mut self) {
        self.cancellation_token.cancel();
        let _ = self._unsubscribe_tx.send(self.id);
    }
}

pub struct Event<T> {
    state: Arc<RwLock<EventEmitterState<T>>>,
    unsubscribe_tx: mpsc::UnboundedSender<usize>,
}

impl<T: EventMarker> Event<T> {
    /// Subscribe to events with unlimited executions
    pub async fn subscribe<F, Fut>(&self, f: F) -> Subscription<T>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
        T: Clone,
    {
        self.limited(None, f).await
    }

    /// Subscribe to events with a one-time execution
    pub async fn once<F, Fut>(&self, f: F) -> Subscription<T>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
        T: Clone,
    {
        self.limited(Some(1), f).await
    }

    /// Subscribe to events with limited executions
    pub async fn limited<F, Fut>(&self, limit: Option<u64>, f: F) -> Subscription<T>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
        T: Clone,
    {
        let cancellation_token = CancellationToken::new();
        let id = {
            let mut state_lock = self.state.write().await;
            let id = state_lock.next_id;
            state_lock.next_id += 1;

            let listener = Listener {
                id,
                limit,
                callback: Arc::new(move |event| Box::pin(f(event))),
                cancellation_token: cancellation_token.clone(),
            };

            state_lock.listeners.insert(id, listener);
            id
        };

        Subscription {
            emitter: Arc::downgrade(&self.state),
            id,
            cancellation_token,
            _unsubscribe_tx: self.unsubscribe_tx.clone(),
        }
    }
}

struct EventEmitterState<T> {
    listeners: FxHashMap<usize, Listener<T>>,
    next_id: usize,
}

pub struct EventEmitter<T: EventMarker> {
    state: Arc<RwLock<EventEmitterState<T>>>,
    unsubscribe_tx: mpsc::UnboundedSender<usize>,
    unsubscribe_rx: Arc<Mutex<mpsc::UnboundedReceiver<usize>>>,
}

impl<T: EventMarker> EventEmitter<T> {
    pub fn new() -> Self {
        let (unsubscribe_sender, unsubscribe_receiver) = mpsc::unbounded_channel();

        Self {
            state: Arc::new(RwLock::new(EventEmitterState {
                listeners: FxHashMap::default(),
                next_id: 0,
            })),
            unsubscribe_tx: unsubscribe_sender,
            unsubscribe_rx: Arc::new(Mutex::new(unsubscribe_receiver)),
        }
    }

    pub fn event(&self) -> Event<T> {
        Event {
            state: self.state.clone(),
            unsubscribe_tx: self.unsubscribe_tx.clone(),
        }
    }

    /// Subscribe to events with unlimited executions
    pub async fn subscribe<F, Fut>(&self, f: F) -> Subscription<T>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
        T: Clone,
    {
        self.subscribe_limited(None, f).await
    }

    /// Subscribe to events with a one-time execution
    pub async fn subscribe_once<F, Fut>(&self, f: F) -> Subscription<T>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
        T: Clone,
    {
        self.subscribe_limited(Some(1), f).await
    }

    /// Subscribe to events with limited executions
    pub async fn subscribe_limited<F, Fut>(&self, limit: Option<u64>, f: F) -> Subscription<T>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
        T: Clone,
    {
        let cancellation_token = CancellationToken::new();
        let id = {
            let mut state_lock = self.state.write().await;
            let id = state_lock.next_id;
            state_lock.next_id += 1;

            let listener = Listener {
                id,
                limit,
                callback: Arc::new(move |event| Box::pin(f(event))),
                cancellation_token: cancellation_token.clone(),
            };

            state_lock.listeners.insert(id, listener);
            id
        };

        Subscription {
            emitter: Arc::downgrade(&self.state),
            id,
            cancellation_token,
            _unsubscribe_tx: self.unsubscribe_tx.clone(),
        }
    }

    async fn process_unsubscribe_requests(&self) {
        let mut state_lock = self.state.write().await;
        let mut receiver_lock = self.unsubscribe_rx.lock().await;

        // Try to receive all pending unsubscribe requests without blocking
        while let Ok(id) = receiver_lock.try_recv() {
            state_lock.listeners.remove(&id);
        }
    }

    /// Fire an event to all listeners
    pub async fn fire(&self, value: T)
    where
        T: Clone,
    {
        // Process any pending unsubscribe requests first
        self.process_unsubscribe_requests().await;

        let mut futures: FuturesUnordered<_> = FuturesUnordered::new();
        let mut listeners_to_remove: Vec<usize> = Vec::new();

        // Collect all listeners and prepare futures
        {
            let mut state_lock = self.state.write().await;
            let listeners_to_process: Vec<_> = state_lock.listeners.iter_mut().collect();

            for (id, listener) in listeners_to_process {
                // Check if listener was cancelled
                if listener.cancellation_token.is_cancelled() {
                    listeners_to_remove.push(*id);
                    continue;
                }

                match listener.limit {
                    None => {
                        // Unlimited listener
                        let callback = Arc::clone(&listener.callback);
                        let future = callback(value.clone());
                        let wrapped_future = AssertUnwindSafe(future);
                        futures.push(
                            async move {
                                if let Err(err) = wrapped_future.catch_unwind().await {
                                    eprintln!("Listener panicked: {:?}", err);
                                }
                            }
                            .boxed(),
                        );
                    }
                    Some(limit) => {
                        if limit > 0 {
                            // Limited listener with remaining executions
                            let callback = Arc::clone(&listener.callback);
                            let future = callback(value.clone());
                            let wrapped_future = AssertUnwindSafe(future);
                            futures.push(
                                async move {
                                    if let Err(err) = wrapped_future.catch_unwind().await {
                                        eprintln!("Listener panicked: {:?}", err);
                                    }
                                }
                                .boxed(),
                            );

                            listener.limit = Some(limit - 1);

                            // Mark for removal if this was the last execution
                            if limit == 1 {
                                listeners_to_remove.push(*id);
                            }
                        } else {
                            // Exhausted listener, mark for removal
                            listeners_to_remove.push(*id);
                        }
                    }
                }
            }

            // Remove exhausted and cancelled listeners
            for id in &listeners_to_remove {
                state_lock.listeners.remove(id);
            }
        }

        // Execute all futures concurrently
        while futures.next().await.is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[derive(Clone)]
    pub struct TestEvent {
        #[allow(unused)]
        value: String,
    }

    impl EventMarker for TestEvent {}

    #[tokio::test]
    async fn test_once_subscription() {
        let counter = Arc::new(AtomicU32::new(0));
        let emitter: EventEmitter<TestEvent> = EventEmitter::new();

        let counter_clone = counter.clone();
        let _subscription = emitter
            .subscribe_once(move |_: TestEvent| {
                let counter = counter_clone.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            })
            .await;

        // Fire event twice
        emitter
            .fire(TestEvent {
                value: "first".to_string(),
            })
            .await;
        emitter
            .fire(TestEvent {
                value: "second".to_string(),
            })
            .await;

        // Should only be called once
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_limited_subscription() {
        let counter = Arc::new(AtomicU32::new(0));
        let emitter: EventEmitter<TestEvent> = EventEmitter::new();

        let counter_clone = counter.clone();
        let _subscription = emitter
            .subscribe_limited(Some(3), move |_: TestEvent| {
                let counter = counter_clone.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            })
            .await;

        // Fire event 5 times
        for i in 0..5 {
            emitter
                .fire(TestEvent {
                    value: format!("event {}", i),
                })
                .await;
        }

        // Should only be called 3 times
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_event_struct() {
        let emitter: EventEmitter<TestEvent> = EventEmitter::new();
        let event = emitter.event();

        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let _subscription = event
            .subscribe(move |_: TestEvent| {
                let counter = counter_clone.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            })
            .await;

        emitter
            .fire(TestEvent {
                value: "test".to_string(),
            })
            .await;
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_multiple_listeners() {
        let counter1 = Arc::new(AtomicU32::new(0));
        let counter2 = Arc::new(AtomicU32::new(0));
        let emitter: EventEmitter<TestEvent> = EventEmitter::new();

        let counter1_clone = counter1.clone();
        let _sub1 = emitter
            .subscribe(move |_: TestEvent| {
                let counter = counter1_clone.clone();
                async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            })
            .await;

        let counter2_clone = counter2.clone();
        let _sub2 = emitter
            .subscribe(move |_: TestEvent| {
                let counter = counter2_clone.clone();
                async move {
                    counter.fetch_add(2, Ordering::SeqCst);
                }
            })
            .await;

        emitter
            .fire(TestEvent {
                value: "test".to_string(),
            })
            .await;

        assert_eq!(counter1.load(Ordering::SeqCst), 1);
        assert_eq!(counter2.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_auto_unsubscribe_on_drop() {
        let counter = Arc::new(AtomicU32::new(0));
        let emitter: EventEmitter<TestEvent> = EventEmitter::new();

        {
            let counter_clone = counter.clone();
            let _subscription = emitter
                .subscribe(move |_: TestEvent| {
                    let counter = counter_clone.clone();
                    async move {
                        counter.fetch_add(1, Ordering::SeqCst);
                    }
                })
                .await;

            // Fire event while subscription is alive
            emitter
                .fire(TestEvent {
                    value: "first".to_string(),
                })
                .await;

            // subscription goes out of scope here and should be dropped
        }

        // Fire event after subscription is dropped
        emitter
            .fire(TestEvent {
                value: "second".to_string(),
            })
            .await;

        // Should only be called once (before drop)
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}
