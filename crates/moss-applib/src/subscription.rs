use rustc_hash::FxHashMap;
use std::{
    any::TypeId,
    hash::Hash,
    sync::{Arc, Mutex, RwLock, Weak},
};

use crate::Event;

pub trait AnySubscription {
    fn type_id(&self) -> TypeId;
    fn unsubscribe(&self);
}

impl<T: Event> AnySubscription for Subscription<T> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn unsubscribe(&self) {
        Subscription::unsubscribe(self);
    }
}

pub struct SubscriptionSet<Key>
where
    Key: Hash + Eq,
{
    subscriptions: RwLock<FxHashMap<Key, Vec<Box<dyn AnySubscription>>>>,
}

impl<Key> SubscriptionSet<Key>
where
    Key: Hash + Eq,
{
    pub fn insert(&self, key: Key, s: impl AnySubscription + 'static) {
        let mut subscriptions_lock = self.subscriptions.write().unwrap();
        subscriptions_lock
            .entry(key)
            .or_insert_with(Vec::new)
            .push(Box::new(s));
    }

    pub fn remove(&self, key: Key, type_id: Option<TypeId>) {
        let mut subscriptions_lock = self.subscriptions.write().unwrap();
        if let Some(type_id) = type_id {
            if let Some(subscriptions) = subscriptions_lock.get_mut(&key) {
                subscriptions.retain(|s| s.type_id() != type_id);
            }
        } else {
            subscriptions_lock.remove(&key);
        }
    }
}

type Listener<T> = Arc<dyn Fn(&T) + Send + Sync + 'static>;

pub struct Subscription<T: Event> {
    emitter: Weak<Mutex<EmitterState<T>>>,
    id: usize,
}

impl<T: Event> Subscription<T> {
    pub fn unsubscribe(&self) {
        if let Some(state) = self.emitter.upgrade() {
            state.lock().unwrap().listeners.remove(&self.id);
        }
    }
}

impl<T: Event> Drop for Subscription<T> {
    fn drop(&mut self) {
        self.unsubscribe();
    }
}

struct EmitterState<T> {
    listeners: FxHashMap<usize, Listener<T>>,
    next_id: usize,
}

pub struct Emitter<T: Event> {
    state: Arc<Mutex<EmitterState<T>>>,
}

impl<T: Event> Emitter<T> {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(EmitterState {
                listeners: FxHashMap::default(),
                next_id: 0,
            })),
        }
    }

    pub fn subscribe<F>(&self, f: F) -> Subscription<T>
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let mut state_lock = self.state.lock().unwrap();
        let id = state_lock.next_id;
        state_lock.next_id += 1;
        state_lock.listeners.insert(id, Arc::new(f));

        Subscription {
            emitter: Arc::downgrade(&self.state),
            id,
        }
    }

    pub fn fire(&self, value: T) {
        let listeners = {
            let state_lock = self.state.lock().unwrap();
            state_lock.listeners.values().cloned().collect::<Vec<_>>()
        };

        for listener in listeners {
            listener(&value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestEvent {
        pub value: String,
    }

    impl Event for TestEvent {}

    #[test]
    fn test_emitter() {
        let emitter: Emitter<TestEvent> = Emitter::new();

        let subscription = emitter.subscribe(|event: &TestEvent| {
            println!("Received event: {}", event.value);
        });

        emitter.fire(TestEvent {
            value: "Hello, world!".to_string(),
        });

        drop(subscription);

        emitter.fire(TestEvent {
            value: "2nd event!".to_string(),
        });
    }
}
