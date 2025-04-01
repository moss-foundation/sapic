use slotmap::basic::Iter;
use std::collections::HashSet;

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
    pub(crate) iter: Iter<'a, K, V>,
    pub(crate) leased_keys: &'a HashSet<K>,
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
