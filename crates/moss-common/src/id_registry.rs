use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, atomic::AtomicUsize};
use tokio::sync::Mutex;

use crate::models::primitives::Identifier;

pub struct IdRegistry {
    inner: Mutex<HashMap<TypeId, Arc<AtomicUsize>>>,
}

impl IdRegistry {
    pub fn new() -> Arc<IdRegistry> {
        Arc::new(IdRegistry {
            inner: Mutex::new(HashMap::new()),
        })
    }

    pub async fn next_id<T: ?Sized + 'static>(&self) -> Identifier {
        let type_id = std::any::TypeId::of::<T>();
        let mut lock = self.inner.lock().await;
        Identifier::new(
            lock.entry(type_id)
                .or_insert_with(|| Arc::new(AtomicUsize::new(0))),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStruct {
        #[allow(dead_code)]
        name: String,
        #[allow(dead_code)]
        value: i32,
    }

    #[tokio::test]
    async fn test_id_registry_single_type() {
        let registry = IdRegistry::new();

        let id_0 = registry.next_id::<i32>().await;
        let id_1 = registry.next_id::<i32>().await;
        let id_2 = registry.next_id::<i32>().await;

        assert_eq!(id_0.to_usize(), 0);
        assert_eq!(id_1.to_usize(), 1);
        assert_eq!(id_2.to_usize(), 2);
    }

    #[tokio::test]
    async fn test_id_registry_multi_type() {
        let registry = IdRegistry::new();

        let i32_id_0 = registry.next_id::<i32>().await;
        let i32_id_1 = registry.next_id::<i32>().await;
        let i64_id_0 = registry.next_id::<i64>().await;
        let i64_id_1 = registry.next_id::<i64>().await;

        assert_eq!(i32_id_0.to_usize(), 0);
        assert_eq!(i32_id_1.to_usize(), 1);
        assert_eq!(i64_id_0.to_usize(), 0);
        assert_eq!(i64_id_1.to_usize(), 1);
    }

    #[tokio::test]
    async fn test_id_registry_custom_type() {
        let registry = IdRegistry::new();
        let id_0 = registry.next_id::<TestStruct>().await;
        let id_1 = registry.next_id::<TestStruct>().await;
        let id_2 = registry.next_id::<TestStruct>().await;

        assert_eq!(id_0.to_usize(), 0);
        assert_eq!(id_1.to_usize(), 1);
        assert_eq!(id_2.to_usize(), 2);
    }
}
