use serde::{Deserialize, Serialize};
use slotmap::KeyData;
use ts_rs::TS;

slotmap::new_key_type! {
    pub struct ResourceKey;
}

// TODO: add as a feature
impl Serialize for ResourceKey {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.as_u64())
    }
}

// TODO: add as a feature
impl<'de> Deserialize<'de> for ResourceKey {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(ResourceKey::from(u64::deserialize(deserializer)?))
    }
}

// TODO: add as a feature
impl TS for ResourceKey {
    type WithoutGenerics = Self;

    fn name() -> String {
        "ResourceKey".into()
    }

    fn inline() -> String {
        "number".into()
    }

    fn decl() -> String {
        format!("export type {} = {};", Self::name(), Self::inline())
    }

    fn decl_concrete() -> String {
        Self::decl()
    }

    fn inline_flattened() -> String {
        Self::inline()
    }
}

impl From<u64> for ResourceKey {
    fn from(value: u64) -> Self {
        Self(KeyData::from_ffi(value))
    }
}

impl ResourceKey {
    pub fn as_u64(self) -> u64 {
        self.0.as_ffi()
    }
}

impl std::fmt::Display for ResourceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u64())
    }
}
