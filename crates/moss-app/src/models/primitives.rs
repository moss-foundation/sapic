use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, sync::Arc};
use tracing::{
    Level,
    field::{Field, Visit},
};
use ts_rs::TS;

// ########################################################
// ###                      Id                          ###
// ########################################################

pub type LocaleId = String;
pub type ThemeId = String;

#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkspaceId(Arc<String>);
impl WorkspaceId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }
}

impl From<String> for WorkspaceId {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl AsRef<str> for WorkspaceId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for WorkspaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq, Deref, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogEntryId(Arc<String>);
impl LogEntryId {
    pub fn new() -> Self {
        Self(Arc::new(nanoid!(10)))
    }
}

impl From<String> for LogEntryId {
    fn from(s: String) -> Self {
        Self(Arc::new(s))
    }
}

impl AsRef<str> for LogEntryId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for LogEntryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ########################################################
// ###                      Theme                       ###
// ########################################################

/// @category Primitive
#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "primitives.ts")]
pub enum ThemeMode {
    Light,
    Dark,
}

// #########################################################
// ###                      Log                          ###
// #########################################################

/// @category Primitive
#[derive(Debug, Copy, Clone, Serialize, Deserialize, TS, PartialEq)]
#[ts(export, export_to = "primitives.ts")]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl Into<Level> for LogLevel {
    fn into(self) -> Level {
        match self {
            LogLevel::TRACE => Level::TRACE,
            LogLevel::DEBUG => Level::DEBUG,
            LogLevel::INFO => Level::INFO,
            LogLevel::WARN => Level::WARN,
            LogLevel::ERROR => Level::ERROR,
        }
    }
}
