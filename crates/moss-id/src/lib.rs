use moss_common::models::primitives::Identifier;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex};

struct IdRegistry {
    next_ids: Mutex<HashMap<TypeId, AtomicUsize>>,
}

impl IdRegistry {
    fn new() -> Arc<IdRegistry> {
        Arc::new(IdRegistry {
            next_ids: Mutex::new(HashMap::new()),
        })
    }

    fn next_id<T: ?Sized + 'static>(&self) -> Identifier {
        let type_id = std::any::TypeId::of::<T>();
        let mut lock = self.next_ids.lock().unwrap();
        if !lock.contains_key(&type_id) {
            lock.insert(type_id, AtomicUsize::new(0));
        }
        Identifier::new(lock.get(&type_id).unwrap())
    }
}

mod tests {
    use super::*;

    #[test]
    fn test() {
        let registry = IdRegistry::new();

        let i32_id = registry.next_id::<i32>();
        let i64_id = registry.next_id::<i64>();

        dbg!(i32_id, i64_id);
    }
}
