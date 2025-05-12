mod recourse_key;
pub use recourse_key::*;

mod lease_guard;
pub use lease_guard::*;

mod iterator;
pub use iterator::*;

use anyhow::{Context, Result, anyhow};
use slotmap::SlotMap;
use std::collections::HashSet;

pub struct LeasedSlotMap<K, V>
where
    K: slotmap::Key,
{
    inner: SlotMap<K, V>,
    leased_keys: HashSet<K>,
}

impl<K, V> LeasedSlotMap<K, V>
where
    K: slotmap::Key,
{
    pub fn new() -> Self {
        Self {
            inner: SlotMap::with_key(),
            leased_keys: HashSet::new(),
        }
    }

    pub fn iter(&self) -> LeasedSlotMapIter<'_, K, V> {
        LeasedSlotMapIter {
            iter: self.inner.iter(),
            leased_keys: &self.leased_keys,
        }
    }

    pub fn insert(&mut self, value: V) -> K {
        self.inner.insert(value)
    }

    pub fn lease(&mut self, key: K) -> Result<LeaseGuard<'_, K, V>> {
        if !self.inner.contains_key(key) {
            Err(anyhow!("Key not found"))
        } else if self.leased_keys.contains(&key) {
            Err(anyhow!("Slot is already leased"))
        } else {
            self.leased_keys.insert(key);
            Ok(LeaseGuard { key, slotmap: self })
        }
    }

    pub fn get(&self, key: K) -> Result<&V> {
        if self.leased_keys.contains(&key) {
            Err(anyhow!("Slot is currently leased"))
        } else {
            self.inner.get(key).context("Key not found")
        }
    }

    pub fn get_mut(&mut self, key: K) -> Result<&mut V> {
        if self.leased_keys.contains(&key) {
            Err(anyhow!("Slot is currently leased"))
        } else {
            self.inner.get_mut(key).context("Key not found")
        }
    }

    pub fn remove(&mut self, key: K) -> Result<V> {
        if self.leased_keys.contains(&key) {
            Err(anyhow!("Cannot remove a leased slot"))
        } else {
            self.inner.remove(key).context("Key not found")
        }
    }
}
