use moss_id_macro::ids;
use serde::{Deserialize, Serialize};
use tracing::Level;
use ts_rs::TS;

ids!([WorkspaceId, LocaleId, ThemeId]);

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export, export_to = "primitives.ts")]
pub enum ConfigurationTarget {
    Profile,
    Workspace,
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
