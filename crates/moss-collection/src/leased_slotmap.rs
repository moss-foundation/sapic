use anyhow::{anyhow, Context, Result};
use slotmap::basic::Iter;
use slotmap::SlotMap;
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

pub struct LeaseGuard<'a, K, V>
where
    K: slotmap::Key,
{
    key: K,
    slotmap: &'a mut LeasedSlotMap<K, V>,
}

impl<'a, K, V> Drop for LeaseGuard<'a, K, V>
where
    K: slotmap::Key,
{
    fn drop(&mut self) {
        self.slotmap.leased_keys.remove(&self.key);
    }
}

impl<'a, K, V> Deref for LeaseGuard<'a, K, V>
where
    K: slotmap::Key,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.slotmap.inner[self.key]
    }
}

impl<'a, K, V> DerefMut for LeaseGuard<'a, K, V>
where
    K: slotmap::Key,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slotmap.inner[self.key]
    }
}

pub struct IterSlot<'a, V> {
    value: &'a V,
    is_leased: bool,
}

impl<'a, V> IterSlot<'a, V> {
    pub fn value(&self) -> &'a V {
        self.value
    }

    pub fn is_leased(&self) -> bool {
        self.is_leased
    }
}

pub struct LeasedSlotMapIter<'a, K, V>
where
    K: slotmap::Key,
{
    iter: Iter<'a, K, V>,
    leased_keys: &'a HashSet<K>,
}

impl<'a, K, V> Iterator for LeasedSlotMapIter<'a, K, V>
where
    K: slotmap::Key,
{
    type Item = (K, IterSlot<'a, V>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(key, value)| {
            let is_leased = self.leased_keys.contains(&key);
            (key, IterSlot { value, is_leased })
        })
    }
}

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
