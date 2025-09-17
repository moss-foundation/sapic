use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};
use ts_rs::TS;

use crate::models::primitives::{CssValue, ThemeMode};

/// @category Type
#[derive(Deserialize, Serialize, Debug, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "types.ts")]
pub struct Theme {
    pub identifier: String,
    pub display_name: String,
    pub mode: ThemeMode,
    #[ts(type = "Record<string, CssValue>")]
    pub tokens: HashMap<String, CssValue>,
}
