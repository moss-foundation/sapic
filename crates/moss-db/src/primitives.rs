use redb::{Key, TypeName, Value};
use std::{fmt::Debug, hash::Hash};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AnyKey(Vec<u8>);

impl AnyKey {
    pub fn new(key: &str) -> Self {
        Self(key.as_bytes().to_vec())
    }
}

impl From<Vec<u8>> for AnyKey {
    fn from(value: Vec<u8>) -> Self {
        Self(value)
    }
}

impl std::borrow::Borrow<[u8]> for AnyKey {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Display for AnyKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl Value for AnyKey {
    type SelfType<'a>
        = AnyKey
    where
        Self: 'a;

    type AsBytes<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn type_name() -> TypeName {
        TypeName::new("AnyKey")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
        &value.0
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        AnyKey(data.to_vec())
    }
}

impl Key for AnyKey {
    fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
        data1.cmp(data2)
    }
}
