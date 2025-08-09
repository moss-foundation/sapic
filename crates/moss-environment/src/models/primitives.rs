use derive_more::Deref;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};
use ts_rs::TS;

/// @category Primitive
#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VariableId(Arc<String>);
impl VariableId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }
}

impl From<String> for VariableId {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl AsRef<str> for VariableId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for VariableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[rustfmt::skip]
impl TS for VariableId {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name() -> String { "string".to_string() }
    fn inline() -> String { "string".to_string() }
    fn inline_flattened() -> String { "string".to_string() }
    fn decl() -> String { unreachable!() }
    fn decl_concrete() -> String { unreachable!() }
    fn dependencies() -> Vec<ts_rs::Dependency> { vec![] }
}

/// @category Primitive
#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EnvironmentId(Arc<String>);
impl EnvironmentId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }
}

impl From<String> for EnvironmentId {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl AsRef<str> for EnvironmentId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for EnvironmentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[rustfmt::skip]
impl TS for EnvironmentId {
    type WithoutGenerics = Self;
    type OptionInnerType = Self;

    fn name() -> String { "string".to_string() }
    fn inline() -> String { "string".to_string() }
    fn inline_flattened() -> String { "string".to_string() }
    fn decl() -> String { unreachable!() }
    fn decl_concrete() -> String { unreachable!() }
    fn dependencies() -> Vec<ts_rs::Dependency> { vec![] }
}
