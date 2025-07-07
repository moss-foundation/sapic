use nanoid::nanoid;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Display, hash::Hash, sync::Arc};

const ID_LENGTH: usize = 10;

#[derive(Debug)]
pub struct NanoId(Arc<String>);

impl Clone for NanoId {
    fn clone(&self) -> Self {
        NanoId(self.0.clone())
    }
}

impl Serialize for NanoId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_ref())
    }
}

impl<'de> Deserialize<'de> for NanoId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(NanoId(s.into()))
    }
}

impl AsRef<str> for NanoId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<NanoId> for String {
    fn from(value: NanoId) -> Self {
        value.0.to_string()
    }
}

impl From<String> for NanoId {
    fn from(value: String) -> Self {
        NanoId(Arc::new(value))
    }
}

impl PartialEq for NanoId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for NanoId {}

impl Hash for NanoId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Display for NanoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn new_nanoid() -> NanoId {
    NanoId(Arc::new(nanoid!(ID_LENGTH)))
}

pub fn new_nanoid_string() -> String {
    nanoid!(ID_LENGTH)
}
