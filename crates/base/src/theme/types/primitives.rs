use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

/// @category Primitive
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, TS, Display)]
#[ts(export, export_to = "theme/primitives.ts")]
pub struct ThemeId(Arc<String>);

/// @category Primitive
#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "theme/primitives.ts")]
pub enum ThemeMode {
    Light,
    Dark,
}
