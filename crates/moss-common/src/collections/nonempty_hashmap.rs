use std::{collections::HashMap, hash::Hash};

#[macro_export]
macro_rules! nonempty_hashmap {
    ($k:expr => $v:expr, $( $key:expr => $val:expr ),* $(,)?) => {{
        let mut map = $crate::collections::nonempty_hashmap::NonEmptyHashMap::new($k, $v);
        $(
            map.insert($key, $val);
        )*
        map
    }};
    ($k:expr => $v:expr) => {
        $crate::collections::nonempty_hashmap::NonEmptyHashMap::new($k, $v)
    };
}

/// Non-empty hash map.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NonEmptyHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    pub head: (K, V),
    pub tail: HashMap<K, V>,
}

impl<K, V> NonEmptyHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    /// Create a new non-empty hash map with an initial key-value pair.
    pub fn new(key: K, value: V) -> Self {
        Self {
            head: (key, value),
            tail: HashMap::new(),
        }
    }

    /// Create a non-empty hash map with an initial key-value pair and a HashMap.
    pub fn from_hash_map(key: K, value: V, tail: HashMap<K, V>) -> Self {
        Self {
            head: (key, value),
            tail,
        }
    }

    /// Converts a `HashMap` to a `NonEmptyHashMap`.
    /// Returns `None` if the hash map is empty.
    pub fn from_hash_map_option(map: HashMap<K, V>) -> Option<Self> {
        let mut iter = map.into_iter();
        iter.next().map(|(key, value)| {
            let mut non_empty_map = NonEmptyHashMap::new(key, value);
            for (k, v) in iter {
                non_empty_map.insert(k, v);
            }
            non_empty_map
        })
    }

    /// Create a non-empty hash map from a key-value pair tuple.
    pub fn from_pair(head: (K, V)) -> Self {
        Self {
            head,
            tail: HashMap::new(),
        }
    }

    /// Get the first key-value pair. Never fails.
    pub fn first(&self) -> (&K, &V) {
        (&self.head.0, &self.head.1)
    }

    /// Get the first key-value pair mutably.
    pub fn first_mut(&mut self) -> (&K, &mut V) {
        (&self.head.0, &mut self.head.1)
    }

    /// Get the possibly-empty tail of the map.
    pub fn tail(&self) -> &HashMap<K, V> {
        &self.tail
    }

    /// Get the possibly-empty tail of the map, mutably.
    pub fn tail_mut(&mut self) -> &mut HashMap<K, V> {
        &mut self.tail
    }

    /// Insert a key-value pair into the map.
    /// Returns the previous value if the key was already present.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if key == self.head.0 {
            Some(std::mem::replace(&mut self.head.1, value))
        } else {
            self.tail.insert(key, value)
        }
    }

    /// Get a reference to the value corresponding to the key.
    pub fn get(&self, key: &K) -> Option<&V> {
        if *key == self.head.0 {
            Some(&self.head.1)
        } else {
            self.tail.get(key)
        }
    }

    /// Get a mutable reference to the value corresponding to the key.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if *key == self.head.0 {
            Some(&mut self.head.1)
        } else {
            self.tail.get_mut(key)
        }
    }

    /// Return true if the map contains the given key.
    pub fn contains_key(&self, key: &K) -> bool {
        *key == self.head.0 || self.tail.contains_key(key)
    }

    /// Remove a key from the map, returning the value at the key if it was previously in the map.
    /// If the removed key is the head key and there are other elements, promotes one from tail to head.
    /// Returns `None` if trying to remove the last remaining element.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        if *key == self.head.0 {
            if self.tail.is_empty() {
                // Cannot remove the last element
                None
            } else {
                // Move an element from tail to head
                if let Some((new_head_key, new_head_value)) = self.tail.drain().next() {
                    let old_head =
                        std::mem::replace(&mut self.head, (new_head_key, new_head_value));
                    Some(old_head.1)
                } else {
                    None
                }
            }
        } else {
            self.tail.remove(key)
        }
    }

    /// The length of the map (always >= 1).
    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }

    /// Whether the map has only one key-value pair.
    pub fn is_singleton(&self) -> bool {
        self.tail.is_empty()
    }

    /// An iterator over the key-value pairs of the map.
    pub fn iter(&self) -> NonEmptyHashMapIter<'_, K, V> {
        NonEmptyHashMapIter {
            head: Some(&self.head),
            tail_iter: self.tail.iter(),
        }
    }

    /// An iterator over the keys of the map.
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        std::iter::once(&self.head.0).chain(self.tail.keys())
    }

    /// An iterator over the values of the map.
    pub fn values(&self) -> impl Iterator<Item = &V> {
        std::iter::once(&self.head.1).chain(self.tail.values())
    }

    /// A mutable iterator over the values of the map.
    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        std::iter::once(&mut self.head.1).chain(self.tail.values_mut())
    }

    /// Convert to a `HashMap`.
    pub fn into_hash_map(self) -> HashMap<K, V> {
        let mut map = self.tail;
        map.insert(self.head.0, self.head.1);
        map
    }

    /// Split the map into head and tail.
    pub fn split(self) -> ((K, V), HashMap<K, V>) {
        (self.head, self.tail)
    }

    /// Gets the head (first inserted) key-value pair.
    /// Alias for `first()` to maintain compatibility.
    pub fn head(&self) -> (&K, &V) {
        self.first()
    }

    /// Gets a mutable reference to the head value.
    /// Alias for `first_mut()` to maintain compatibility.
    pub fn head_mut(&mut self) -> (&K, &mut V) {
        self.first_mut()
    }
}

pub struct NonEmptyHashMapIter<'a, K, V> {
    head: Option<&'a (K, V)>,
    tail_iter: std::collections::hash_map::Iter<'a, K, V>,
}

impl<'a, K, V> Iterator for NonEmptyHashMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(head) = self.head.take() {
            Some((&head.0, &head.1))
        } else {
            self.tail_iter.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let head_count = if self.head.is_some() { 1 } else { 0 };
        let (tail_min, tail_max) = self.tail_iter.size_hint();
        (head_count + tail_min, tail_max.map(|max| head_count + max))
    }
}

impl<'a, K, V> ExactSizeIterator for NonEmptyHashMapIter<'a, K, V> {
    fn len(&self) -> usize {
        let head_len = if self.head.is_some() { 1 } else { 0 };
        head_len + self.tail_iter.len()
    }
}

pub struct NonEmptyHashMapIntoIter<K, V> {
    head: Option<(K, V)>,
    tail_iter: std::collections::hash_map::IntoIter<K, V>,
}

impl<K, V> Iterator for NonEmptyHashMapIntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(head) = self.head.take() {
            Some(head)
        } else {
            self.tail_iter.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let head_count = if self.head.is_some() { 1 } else { 0 };
        let (tail_min, tail_max) = self.tail_iter.size_hint();
        (head_count + tail_min, tail_max.map(|max| head_count + max))
    }
}

impl<K, V> ExactSizeIterator for NonEmptyHashMapIntoIter<K, V> {
    fn len(&self) -> usize {
        let head_len = if self.head.is_some() { 1 } else { 0 };
        head_len + self.tail_iter.len()
    }
}

impl<K, V> IntoIterator for NonEmptyHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    type Item = (K, V);
    type IntoIter = NonEmptyHashMapIntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        NonEmptyHashMapIntoIter {
            head: Some(self.head),
            tail_iter: self.tail.into_iter(),
        }
    }
}

impl<'a, K, V> IntoIterator for &'a NonEmptyHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    type Item = (&'a K, &'a V);
    type IntoIter = NonEmptyHashMapIter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K, V> From<NonEmptyHashMap<K, V>> for HashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    fn from(non_empty_map: NonEmptyHashMap<K, V>) -> Self {
        non_empty_map.into_hash_map()
    }
}

impl<K, V> TryFrom<HashMap<K, V>> for NonEmptyHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    type Error = ();

    fn try_from(map: HashMap<K, V>) -> Result<Self, Self::Error> {
        NonEmptyHashMap::from_hash_map_option(map).ok_or(())
    }
}

impl<K, V> TryFrom<Vec<(K, V)>> for NonEmptyHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    type Error = ();

    fn try_from(mut vec: Vec<(K, V)>) -> Result<Self, Self::Error> {
        if let Some(head) = vec.pop() {
            let mut non_empty_map = NonEmptyHashMap::from_pair(head);
            for (k, v) in vec {
                non_empty_map.insert(k, v);
            }
            Ok(non_empty_map)
        } else {
            Err(())
        }
    }
}

impl<K, V> From<(K, V)> for NonEmptyHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    fn from((key, value): (K, V)) -> Self {
        NonEmptyHashMap::new(key, value)
    }
}

impl<K, V> From<((K, V), HashMap<K, V>)> for NonEmptyHashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    fn from((head, tail): ((K, V), HashMap<K, V>)) -> Self {
        NonEmptyHashMap { head, tail }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let map = NonEmptyHashMap::new("key1".to_string(), 42);
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&"key1".to_string()), Some(&42));
        assert!(map.is_singleton());
    }

    #[test]
    fn test_macro() {
        let map = nonempty_hashmap!["key1".to_string() => 42, "key2".to_string() => 100];
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&"key1".to_string()), Some(&42));
        assert_eq!(map.get(&"key2".to_string()), Some(&100));

        let single_map = nonempty_hashmap!["key1".to_string() => 42];
        assert_eq!(single_map.len(), 1);
        assert!(single_map.is_singleton());
    }

    #[test]
    fn test_from_pair() {
        let map = NonEmptyHashMap::from_pair(("key1".to_string(), 42));
        assert_eq!(map.len(), 1);
        assert_eq!(map.get(&"key1".to_string()), Some(&42));
    }

    #[test]
    fn test_insert() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);

        // Insert new key
        assert_eq!(map.insert("key2".to_string(), 100), None);
        assert_eq!(map.len(), 2);
        assert!(!map.is_singleton());

        // Update existing key
        assert_eq!(map.insert("key1".to_string(), 99), Some(42));
        assert_eq!(map.get(&"key1".to_string()), Some(&99));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_get() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);
        map.insert("key2".to_string(), 100);

        assert_eq!(map.get(&"key1".to_string()), Some(&42));
        assert_eq!(map.get(&"key2".to_string()), Some(&100));
        assert_eq!(map.get(&"key3".to_string()), None);
    }

    #[test]
    fn test_get_mut() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);
        map.insert("key2".to_string(), 100);

        if let Some(value) = map.get_mut(&"key1".to_string()) {
            *value = 999;
        }

        assert_eq!(map.get(&"key1".to_string()), Some(&999));
    }

    #[test]
    fn test_contains_key() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);
        map.insert("key2".to_string(), 100);

        assert!(map.contains_key(&"key1".to_string()));
        assert!(map.contains_key(&"key2".to_string()));
        assert!(!map.contains_key(&"key3".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);
        map.insert("key2".to_string(), 100);
        map.insert("key3".to_string(), 200);

        // Remove non-head key
        assert_eq!(map.remove(&"key2".to_string()), Some(100));
        assert_eq!(map.len(), 2);
        assert!(!map.contains_key(&"key2".to_string()));

        // Remove head key (should promote from tail)
        assert_eq!(map.remove(&"key1".to_string()), Some(42));
        assert_eq!(map.len(), 1);
        assert!(!map.contains_key(&"key1".to_string()));
        assert!(map.contains_key(&"key3".to_string()));

        // Cannot remove last element
        assert_eq!(map.remove(&"key3".to_string()), None);
        assert_eq!(map.len(), 1);
        assert!(map.contains_key(&"key3".to_string()));
    }

    #[test]
    fn test_len_and_is_singleton() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);
        assert_eq!(map.len(), 1);
        assert!(map.is_singleton());

        map.insert("key2".to_string(), 100);
        assert_eq!(map.len(), 2);
        assert!(!map.is_singleton());

        map.insert("key3".to_string(), 200);
        assert_eq!(map.len(), 3);
        assert!(!map.is_singleton());
    }

    #[test]
    fn test_iter() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);
        map.insert("key2".to_string(), 100);
        map.insert("key3".to_string(), 200);

        let mut items: Vec<_> = map.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        assert_eq!(items.len(), 3);
        assert!(items.contains(&(&"key1".to_string(), &42)));
        assert!(items.contains(&(&"key2".to_string(), &100)));
        assert!(items.contains(&(&"key3".to_string(), &200)));
    }

    #[test]
    fn test_keys() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);
        map.insert("key2".to_string(), 100);

        let mut keys: Vec<_> = map.keys().cloned().collect();
        keys.sort();

        assert_eq!(keys, vec!["key1".to_string(), "key2".to_string()]);
    }

    #[test]
    fn test_values() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);
        map.insert("key2".to_string(), 100);

        let mut values: Vec<_> = map.values().cloned().collect();
        values.sort();

        assert_eq!(values, vec![42, 100]);
    }

    #[test]
    fn test_values_mut() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);
        map.insert("key2".to_string(), 100);

        for value in map.values_mut() {
            *value *= 2;
        }

        assert_eq!(map.get(&"key1".to_string()), Some(&84));
        assert_eq!(map.get(&"key2".to_string()), Some(&200));
    }

    #[test]
    fn test_into_hash_map() {
        let mut non_empty_map = NonEmptyHashMap::new("key1".to_string(), 42);
        non_empty_map.insert("key2".to_string(), 100);

        let hash_map = non_empty_map.into_hash_map();
        assert_eq!(hash_map.len(), 2);
        assert_eq!(hash_map.get("key1"), Some(&42));
        assert_eq!(hash_map.get("key2"), Some(&100));
    }

    #[test]
    fn test_head() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);
        map.insert("key2".to_string(), 100);

        let (key, value) = map.head();
        assert_eq!(key, &"key1".to_string());
        assert_eq!(value, &42);
    }

    #[test]
    fn test_head_mut() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);

        let (key, value) = map.head_mut();
        assert_eq!(key, &"key1".to_string());
        *value = 999;

        assert_eq!(map.get(&"key1".to_string()), Some(&999));
    }

    #[test]
    fn test_into_iterator() {
        let mut map = NonEmptyHashMap::new("key1".to_string(), 42);
        map.insert("key2".to_string(), 100);

        let mut items: Vec<_> = map.into_iter().collect();
        items.sort_by_key(|(k, _)| k.clone());

        assert_eq!(items.len(), 2);
        assert!(items.contains(&("key1".to_string(), 42)));
        assert!(items.contains(&("key2".to_string(), 100)));
    }

    #[test]
    fn test_from_hash_map() {
        let mut hash_map = HashMap::new();
        hash_map.insert("key1".to_string(), 42);
        hash_map.insert("key2".to_string(), 100);

        let non_empty_map = NonEmptyHashMap::try_from(hash_map).unwrap();
        assert_eq!(non_empty_map.len(), 2);
        assert!(non_empty_map.contains_key(&"key1".to_string()));
        assert!(non_empty_map.contains_key(&"key2".to_string()));
    }

    #[test]
    fn test_from_empty_hash_map() {
        let hash_map: HashMap<String, i32> = HashMap::new();
        let result = NonEmptyHashMap::try_from(hash_map);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_vec() {
        let vec = vec![("key1".to_string(), 42), ("key2".to_string(), 100)];
        let non_empty_map = NonEmptyHashMap::try_from(vec).unwrap();

        assert_eq!(non_empty_map.len(), 2);
        assert!(non_empty_map.contains_key(&"key1".to_string()));
        assert!(non_empty_map.contains_key(&"key2".to_string()));
    }

    #[test]
    fn test_from_empty_vec() {
        let vec: Vec<(String, i32)> = vec![];
        let result = NonEmptyHashMap::try_from(vec);
        assert!(result.is_err());
    }

    #[test]
    fn test_debug() {
        let map = NonEmptyHashMap::new("key1".to_string(), 42);
        let debug_str = format!("{:?}", map);
        assert!(debug_str.contains("NonEmptyHashMap"));
    }

    #[test]
    fn test_partial_eq() {
        let map1 = NonEmptyHashMap::new("key1".to_string(), 42);
        let map2 = NonEmptyHashMap::new("key1".to_string(), 42);
        let map3 = NonEmptyHashMap::new("key1".to_string(), 43);

        assert_eq!(map1, map2);
        assert_ne!(map1, map3);
    }

    #[test]
    fn test_clone() {
        let mut original = NonEmptyHashMap::new("key1".to_string(), 42);
        original.insert("key2".to_string(), 100);

        let cloned = original.clone();
        assert_eq!(original, cloned);

        // Ensure they're independent
        drop(original);
        assert_eq!(cloned.len(), 2);
    }
}
