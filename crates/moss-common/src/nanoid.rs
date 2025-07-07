use nanoid::nanoid;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::Arc;

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

pub fn new_nanoid() -> NanoId {
    NanoId(Arc::new(nanoid!(ID_LENGTH)))
}

pub fn new_nanoid_string() -> String {
    nanoid!(ID_LENGTH)
}
