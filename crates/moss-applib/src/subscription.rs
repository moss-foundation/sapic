use derive_more::{Deref, DerefMut};
use futures::{FutureExt, future::BoxFuture};
use rustc_hash::FxHashMap;
use std::{
    any::TypeId,
    hash::Hash,
    panic::AssertUnwindSafe,
    sync::{Arc, Mutex, Weak},
};

use crate::EventMarker;

pub trait AnySubscription {
    fn type_id(&self) -> TypeId;
    fn unsubscribe(&self);
}

impl<T: EventMarker> AnySubscription for Subscription<T> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn unsubscribe(&self) {
        Subscription::unsubscribe(self);
    }
}

#[derive(Default, Deref, DerefMut)]
pub struct SubscriptionSet<Key, E>
where
    Key: Hash + Eq,
    E: EventMarker,
{
    inner: FxHashMap<Key, Subscription<E>>,
}

impl<Key, E> SubscriptionSet<Key, E>
where
    Key: Hash + Eq,
    E: EventMarker,
{
    pub fn new() -> Self {
        Self {
            inner: FxHashMap::default(),
        }
    }
}

pub type ListenerFuture = BoxFuture<'static, ()>;
type Listener<T> = Arc<dyn Fn(T) -> ListenerFuture + Send + Sync + 'static>;

pub struct Subscription<T: EventMarker> {
    emitter: Weak<Mutex<EventEmitterState<T>>>,
    id: usize,
}

impl<T: EventMarker> Subscription<T> {
    pub fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    pub fn unsubscribe(&self) {
        if let Some(state) = self.emitter.upgrade() {
            state.lock().unwrap().listeners.remove(&self.id);
        }
    }
}

impl<T: EventMarker> Drop for Subscription<T> {
    fn drop(&mut self) {
        self.unsubscribe();
    }
}

pub struct Event<T> {
    state: Arc<Mutex<EventEmitterState<T>>>,
}

impl<T: EventMarker> Event<T> {
    pub fn subscribe<F, Fut>(&self, f: F) -> Subscription<T>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
        T: Clone,
    {
        let mut state_lock = self.state.lock().unwrap();
        let id = state_lock.next_id;
        state_lock.next_id += 1;
        state_lock
            .listeners
            .insert(id, Arc::new(move |event| Box::pin(f(event))));

        Subscription {
            emitter: Arc::downgrade(&self.state),
            id,
        }
    }
}

struct EventEmitterState<T> {
    listeners: FxHashMap<usize, Listener<T>>,
    next_id: usize,
}

pub struct EventEmitter<T: EventMarker> {
    state: Arc<Mutex<EventEmitterState<T>>>,
}

impl<T: EventMarker> EventEmitter<T> {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(EventEmitterState {
                listeners: FxHashMap::default(),
                next_id: 0,
            })),
        }
    }

    pub fn event(&self) -> Event<T> {
        Event {
            state: self.state.clone(),
        }
    }

    pub fn subscribe<F, Fut>(&self, f: F) -> Subscription<T>
    where
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
        T: Clone,
    {
        let mut state_lock = self.state.lock().unwrap();
        let id = state_lock.next_id;
        state_lock.next_id += 1;
        state_lock
            .listeners
            .insert(id, Arc::new(move |event| Box::pin(f(event))));

        Subscription {
            emitter: Arc::downgrade(&self.state),
            id,
        }
    }

    pub async fn fire(&self, value: T)
    where
        T: Clone,
    {
        let listeners = {
            let state_lock = self.state.lock().unwrap();
            state_lock.listeners.values().cloned().collect::<Vec<_>>()
        };

        for listener in listeners {
            let future = listener(value.clone());
            let wrapped_future = AssertUnwindSafe(future);
            if let Err(err) = wrapped_future.catch_unwind().await {
                eprintln!("Listener panicked: {:?}", err);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    pub struct TestEvent {
        pub value: String,
    }

    impl EventMarker for TestEvent {}

    #[tokio::test]
    async fn test_emitter() {
        let emitter: EventEmitter<TestEvent> = EventEmitter::new();

        let subscription = emitter.subscribe(|event: TestEvent| async move {
            println!("Received event: {}", event.value);
        });

        emitter
            .fire(TestEvent {
                value: "Hello, world!".to_string(),
            })
            .await;

        drop(subscription);

        emitter
            .fire(TestEvent {
                value: "2nd event!".to_string(),
            })
            .await;
    }
}
