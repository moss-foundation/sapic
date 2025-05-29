use redb::{Key, TypeName, Value};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
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

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct AnyValue(Vec<u8>);

impl AnyValue {
    pub fn new(value: impl Into<Vec<u8>>) -> Self {
        Self(value.into())
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn serialize<T: Serialize>(value: &T) -> Result<Self, serde_json::Error> {
        serde_json::to_vec(value).map(AnyValue)
    }

    pub fn deserialize<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.0)
    }
}

impl std::borrow::Borrow<[u8]> for AnyValue {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Display for AnyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl Value for AnyValue {
    type SelfType<'a>
        = AnyValue
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
        TypeName::new("AnyValue")
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
        &value.0
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        AnyValue(data.to_vec())
    }
}

impl Key for AnyValue {
    fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
        data1.cmp(data2)
    }
}
