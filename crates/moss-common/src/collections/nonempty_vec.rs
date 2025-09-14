use std::vec;

#[macro_export]
macro_rules! nonempty {
    ($h:expr, $( $x:expr ),* $(,)?) => {{
        let tail = vec![$($x),*];
        $crate::collections::nonempty_vec::NonEmptyVec { head: $h, tail }
    }};
    ($h:expr) => {
        $crate::collections::nonempty_vec::NonEmptyVec {
            head: $h,
            tail: vec![],
        }
    };
}

/// Non-empty vector.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NonEmptyVec<T> {
    pub head: T,
    pub tail: Vec<T>,
}

pub struct NonEmptyVecIter<'a, T> {
    head: Option<&'a T>,
    tail: &'a [T],
}

impl<'a, T> Iterator for NonEmptyVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.head.take() {
            Some(value)
        } else if let Some((first, rest)) = self.tail.split_first() {
            self.tail = rest;
            Some(first)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for NonEmptyVecIter<'a, T> {
    fn len(&self) -> usize {
        let head_len = if self.head.is_some() { 1 } else { 0 };
        head_len + self.tail.len()
    }
}

impl<'a, T> DoubleEndedIterator for NonEmptyVecIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some((last, rest)) = self.tail.split_last() {
            self.tail = rest;
            Some(last)
        } else {
            self.head.take()
        }
    }
}

pub struct IterMut<'a, T> {
    head: Option<&'a mut T>,
    tail: &'a mut [T],
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.head.take() {
            Some(value)
        } else if let Some((first, rest)) = std::mem::take(&mut self.tail).split_first_mut() {
            self.tail = rest;
            Some(first)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        let head_len = if self.head.is_some() { 1 } else { 0 };
        head_len + self.tail.len()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some((last, rest)) = std::mem::take(&mut self.tail).split_last_mut() {
            self.tail = rest;
            Some(last)
        } else {
            self.head.take()
        }
    }
}

pub struct NonEmptyVecIntoIter<T> {
    head: Option<T>,
    tail: vec::IntoIter<T>,
}

impl<T> Iterator for NonEmptyVecIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.head.take() {
            Some(value)
        } else {
            self.tail.next()
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for NonEmptyVecIntoIter<T> {
    fn len(&self) -> usize {
        let head_len = if self.head.is_some() { 1 } else { 0 };
        head_len + self.tail.len()
    }
}

impl<T> DoubleEndedIterator for NonEmptyVecIntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.tail.next_back() {
            Some(value)
        } else {
            self.head.take()
        }
    }
}

impl<T> NonEmptyVec<T> {
    /// Create a new non-empty list with an initial element.
    pub fn new(head: T) -> Self {
        NonEmptyVec {
            head,
            tail: Vec::new(),
        }
    }

    /// Create a non-empty list with an initial element and a `Vec`.
    pub fn from_vec(head: T, tail: Vec<T>) -> Self {
        NonEmptyVec { head, tail }
    }

    /// Converts a `Vec` to a `NonEmpty`.
    /// Returns `None` if the vector is empty.
    pub fn from_vec_option(vec: Vec<T>) -> Option<Self> {
        let mut iter = vec.into_iter();
        iter.next().map(|head| NonEmptyVec {
            head,
            tail: iter.collect(),
        })
    }

    /// Create a non-empty vector from a single element.
    /// Alias for `new()` to maintain naming consistency.
    pub fn from_element(element: T) -> Self {
        Self::new(element)
    }

    /// Get the first element. Never fails.
    pub fn first(&self) -> &T {
        &self.head
    }

    /// Get the first element mutably.
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.head
    }

    /// Get the possibly-empty tail of the list.
    pub fn tail(&self) -> &[T] {
        &self.tail
    }

    /// Get the possibly-empty tail of the list, mutably.
    pub fn tail_mut(&mut self) -> &mut Vec<T> {
        &mut self.tail
    }

    /// Get the last element. Never fails.
    pub fn last(&self) -> &T {
        self.tail.last().unwrap_or(&self.head)
    }

    /// Get the last element mutably.
    pub fn last_mut(&mut self) -> &mut T {
        if self.tail.is_empty() {
            &mut self.head
        } else {
            self.tail.last_mut().unwrap()
        }
    }

    /// Push an element to the end of the list.
    pub fn push(&mut self, element: T) {
        self.tail.push(element);
    }

    /// Pop an element from the end of the list.
    /// Returns `None` if the list contains only one element.
    pub fn pop(&mut self) -> Option<T> {
        self.tail.pop()
    }

    /// Insert an element at the given index.
    /// If `index == 0`, the new element becomes the `head`.
    /// **Panics** if `index > len`.
    pub fn insert(&mut self, index: usize, element: T) {
        if index == 0 {
            let old_head = std::mem::replace(&mut self.head, element);
            self.tail.insert(0, old_head);
        } else {
            self.tail.insert(index - 1, element);
        }
    }

    /// Remove and return the element at the given index.
    /// Returns `None` if `index == 0` and the list contains only one element.
    /// **Panics** if `index >= len`.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            if self.tail.is_empty() {
                None
            } else {
                let new_head = self.tail.remove(0);
                Some(std::mem::replace(&mut self.head, new_head))
            }
        } else {
            Some(self.tail.remove(index - 1))
        }
    }

    /// The length of the list.
    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }

    /// Whether the list has only one element.
    pub fn is_singleton(&self) -> bool {
        self.tail.is_empty()
    }

    /// Get an element by index.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            Some(&self.head)
        } else {
            self.tail.get(index - 1)
        }
    }

    /// Get an element by index, mutably.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index == 0 {
            Some(&mut self.head)
        } else {
            self.tail.get_mut(index - 1)
        }
    }

    /// Truncate the list to the given length.
    /// If `len <= 1`, the tail becomes empty.
    pub fn truncate(&mut self, len: usize) {
        if len <= 1 {
            self.tail.clear();
        } else {
            self.tail.truncate(len - 1);
        }
    }

    /// An iterator over the elements of the list.
    pub fn iter(&self) -> NonEmptyVecIter<'_, T> {
        NonEmptyVecIter {
            head: Some(&self.head),
            tail: &self.tail,
        }
    }

    /// A mutable iterator over the elements of the list.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: Some(&mut self.head),
            tail: &mut self.tail,
        }
    }

    /// Map a function over the list.
    pub fn map<U, F>(self, f: F) -> NonEmptyVec<U>
    where
        F: Fn(T) -> U,
    {
        NonEmptyVec {
            head: f(self.head),
            tail: self.tail.into_iter().map(f).collect(),
        }
    }

    /// Try to map a function over the list.
    /// If any application fails, the entire operation fails.
    pub fn try_map<U, E, F>(self, f: F) -> Result<NonEmptyVec<U>, E>
    where
        F: Fn(T) -> Result<U, E>,
    {
        let new_head = f(self.head)?;
        let new_tail: Result<Vec<U>, E> = self.tail.into_iter().map(f).collect();
        Ok(NonEmptyVec {
            head: new_head,
            tail: new_tail?,
        })
    }

    /// Flatten a `NonEmpty<NonEmpty<T>>` to `NonEmpty<T>`.
    pub fn flatten(self) -> NonEmptyVec<T::Item>
    where
        T: IntoIterator,
        T::Item: Clone,
    {
        let mut head_iter = self.head.into_iter();
        let first_item = head_iter
            .next()
            .expect("IntoIterator should yield at least one item");
        let mut result = NonEmptyVec::new(first_item);

        // Add remaining items from head
        for item in head_iter {
            result.push(item.clone());
        }

        // Add items from tail
        for nested in self.tail {
            for item in nested {
                result.push(item.clone());
            }
        }
        result
    }

    /// Sort the list.
    pub fn sort(&mut self)
    where
        T: Ord + Clone,
    {
        if !self.tail.is_empty() {
            let mut all_elements = self.clone().into_vec();
            all_elements.sort();

            let mut iter = all_elements.into_iter();
            self.head = iter.next().unwrap();
            self.tail = iter.collect();
        }
    }

    /// Sort the list by a key function.
    pub fn sort_by_key<K, F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
        T: Clone,
    {
        if !self.tail.is_empty() {
            let mut all_elements = self.clone().into_vec();
            all_elements.sort_by_key(&mut f);

            let mut iter = all_elements.into_iter();
            self.head = iter.next().unwrap();
            self.tail = iter.collect();
        }
    }

    /// Reverse the list.
    pub fn reverse(&mut self)
    where
        T: Clone,
    {
        if !self.tail.is_empty() {
            let mut all_elements = self.clone().into_vec();
            all_elements.reverse();

            let mut iter = all_elements.into_iter();
            self.head = iter.next().unwrap();
            self.tail = iter.collect();
        }
    }

    /// Convert to a `Vec`.
    pub fn into_vec(self) -> Vec<T> {
        let mut vec = vec![self.head];
        vec.extend(self.tail);
        vec
    }

    /// Split the list into head and tail.
    pub fn split(self) -> (T, Vec<T>) {
        (self.head, self.tail)
    }
}

impl<T> IntoIterator for NonEmptyVec<T> {
    type Item = T;
    type IntoIter = NonEmptyVecIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        NonEmptyVecIntoIter {
            head: Some(self.head),
            tail: self.tail.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = NonEmptyVecIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut NonEmptyVec<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> From<NonEmptyVec<T>> for Vec<T> {
    fn from(non_empty: NonEmptyVec<T>) -> Self {
        non_empty.into_vec()
    }
}

impl<T> From<(T, Vec<T>)> for NonEmptyVec<T> {
    fn from((head, tail): (T, Vec<T>)) -> Self {
        NonEmptyVec { head, tail }
    }
}

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = ();

    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        NonEmptyVec::from_vec_option(vec).ok_or(())
    }
}

impl<T> std::ops::Index<usize> for NonEmptyVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.head
        } else {
            &self.tail[index - 1]
        }
    }
}

impl<T> std::ops::IndexMut<usize> for NonEmptyVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self.head
        } else {
            &mut self.tail[index - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ne = NonEmptyVec::new(42);
        assert_eq!(ne.head, 42);
        assert!(ne.tail.is_empty());
        assert_eq!(ne.len(), 1);
        assert!(ne.is_singleton());
    }

    #[test]
    fn test_macro() {
        let ne = nonempty![1, 2, 3];
        assert_eq!(ne.head, 1);
        assert_eq!(ne.tail, vec![2, 3]);
        assert_eq!(ne.len(), 3);

        let ne = nonempty![42];
        assert_eq!(ne.head, 42);
        assert!(ne.tail.is_empty());
    }

    #[test]
    fn test_from_vec() {
        let ne = NonEmptyVec::from_vec(1, vec![2, 3]);
        assert_eq!(ne.head, 1);
        assert_eq!(ne.tail, vec![2, 3]);
    }

    #[test]
    fn test_from_vec_option() {
        let ne = NonEmptyVec::from_vec_option(vec![1, 2, 3]).unwrap();
        assert_eq!(ne.head, 1);
        assert_eq!(ne.tail, vec![2, 3]);

        let empty: Option<NonEmptyVec<i32>> = NonEmptyVec::from_vec_option(vec![]);
        assert!(empty.is_none());
    }

    #[test]
    fn test_first_last() {
        let ne = nonempty![1, 2, 3];
        assert_eq!(*ne.first(), 1);
        assert_eq!(*ne.last(), 3);

        let singleton = nonempty![42];
        assert_eq!(*singleton.first(), 42);
        assert_eq!(*singleton.last(), 42);
    }

    #[test]
    fn test_push_pop() {
        let mut ne = nonempty![1];
        ne.push(2);
        ne.push(3);
        assert_eq!(ne.len(), 3);
        assert_eq!(*ne.last(), 3);

        assert_eq!(ne.pop(), Some(3));
        assert_eq!(ne.len(), 2);
        assert_eq!(ne.pop(), Some(2));
        assert_eq!(ne.len(), 1);
        assert_eq!(ne.pop(), None);
        assert_eq!(ne.len(), 1);
    }

    #[test]
    fn test_insert_remove() {
        let mut ne = nonempty![2, 3];
        ne.insert(0, 1);
        assert_eq!(ne.head, 1);
        assert_eq!(ne.tail, vec![2, 3]);

        ne.insert(3, 4);
        assert_eq!(ne.tail, vec![2, 3, 4]);

        assert_eq!(ne.remove(0), Some(1));
        assert_eq!(ne.head, 2);
        assert_eq!(ne.tail, vec![3, 4]);

        assert_eq!(ne.remove(1), Some(3));
        assert_eq!(ne.tail, vec![4]);

        assert_eq!(ne.remove(1), Some(4));
        assert_eq!(ne.tail, vec![]);
        assert_eq!(ne.remove(0), None);
    }

    #[test]
    fn test_get() {
        let ne = nonempty![1, 2, 3];
        assert_eq!(ne.get(0), Some(&1));
        assert_eq!(ne.get(1), Some(&2));
        assert_eq!(ne.get(2), Some(&3));
        assert_eq!(ne.get(3), None);
    }

    #[test]
    fn test_truncate() {
        let mut ne = nonempty![1, 2, 3, 4];
        ne.truncate(2);
        assert_eq!(ne.head, 1);
        assert_eq!(ne.tail, vec![2]);

        ne.truncate(1);
        assert_eq!(ne.head, 1);
        assert!(ne.tail.is_empty());

        ne.truncate(0);
        assert!(ne.tail.is_empty());
    }

    #[test]
    fn test_iter() {
        let ne = nonempty![1, 2, 3];
        let collected: Vec<_> = ne.iter().cloned().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn test_iter_mut() {
        let mut ne = nonempty![1, 2, 3];
        for item in ne.iter_mut() {
            *item *= 2;
        }
        assert_eq!(ne.head, 2);
        assert_eq!(ne.tail, vec![4, 6]);
    }

    #[test]
    fn test_into_iter() {
        let ne = nonempty![1, 2, 3];
        let collected: Vec<_> = ne.into_iter().collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn test_map() {
        let ne = nonempty![1, 2, 3];
        let mapped = ne.map(|x| x * 2);
        assert_eq!(mapped.head, 2);
        assert_eq!(mapped.tail, vec![4, 6]);
    }

    #[test]
    fn test_try_map() {
        let ne = nonempty![1, 2, 3];
        let result = ne.try_map(|x| if x > 0 { Ok(x * 2) } else { Err("negative") });
        assert!(result.is_ok());
        let mapped = result.unwrap();
        assert_eq!(mapped.head, 2);
        assert_eq!(mapped.tail, vec![4, 6]);

        let ne = nonempty![-1, 2, 3];
        let result = ne.try_map(|x| if x > 0 { Ok(x * 2) } else { Err("negative") });
        assert!(result.is_err());
    }

    #[test]
    fn test_sort() {
        let mut ne = nonempty![3, 1, 2];
        ne.sort();
        assert_eq!(ne.head, 1);
        assert_eq!(ne.tail, vec![2, 3]);
    }

    #[test]
    fn test_reverse() {
        let mut ne = nonempty![1, 2, 3];
        ne.reverse();
        assert_eq!(ne.head, 3);
        assert_eq!(ne.tail, vec![2, 1]);

        let mut singleton = nonempty![42];
        singleton.reverse();
        assert_eq!(singleton.head, 42);
        assert!(singleton.tail.is_empty());
    }

    #[test]
    fn test_into_vec() {
        let ne = nonempty![1, 2, 3];
        let vec = ne.into_vec();
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_split() {
        let ne = nonempty![1, 2, 3];
        let (head, tail) = ne.split();
        assert_eq!(head, 1);
        assert_eq!(tail, vec![2, 3]);
    }

    #[test]
    fn test_index() {
        let ne = nonempty![1, 2, 3];
        assert_eq!(ne[0], 1);
        assert_eq!(ne[1], 2);
        assert_eq!(ne[2], 3);
    }

    #[test]
    fn test_index_mut() {
        let mut ne = nonempty![1, 2, 3];
        ne[0] = 10;
        ne[1] = 20;
        assert_eq!(ne[0], 10);
        assert_eq!(ne[1], 20);
    }

    #[test]
    fn test_conversions() {
        let ne = nonempty![1, 2, 3];
        let vec: Vec<i32> = ne.clone().into();
        assert_eq!(vec, vec![1, 2, 3]);

        let tuple = (1, vec![2, 3]);
        let ne2: NonEmptyVec<i32> = tuple.into();
        assert_eq!(ne2, ne);

        let vec = vec![1, 2, 3];
        let ne3: NonEmptyVec<i32> = vec.try_into().unwrap();
        assert_eq!(ne3, ne);

        let empty_vec: Vec<i32> = vec![];
        let result: Result<NonEmptyVec<i32>, ()> = empty_vec.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_double_ended_iterator() {
        let ne = nonempty![1, 2, 3];
        let mut iter = ne.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next_back(), Some(&2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
}
