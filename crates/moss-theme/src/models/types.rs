use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::models::primitives::ThemeMode;

/// @category Type
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub identifier: String,
    pub display_name: String,
    pub mode: ThemeMode,
    pub palette: IndexMap<String, ColorValue>,
    pub colors: IndexMap<String, ColorValue>,
    pub box_shadows: IndexMap<String, String>,
}

/// @category Type
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum ColorValue {
    Solid(String),
    Gradient(String),
    Variable(String),
}
