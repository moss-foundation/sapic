use std::ops::{Deref, DerefMut};

use super::LeasedSlotMap;

pub struct LeaseGuard<'a, K, V>
where
    K: slotmap::Key,
{
    pub(crate) key: K,
    pub(crate) slotmap: &'a mut LeasedSlotMap<K, V>,
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
