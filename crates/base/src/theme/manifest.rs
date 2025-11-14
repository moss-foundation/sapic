use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::theme::types::primitives::ThemeMode;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThemeFile {
    pub identifier: String,
    pub display_name: String,
    pub mode: ThemeMode,
    pub palette: HashMap<String, ColorValue>,
    pub colors: HashMap<String, ColorValue>,
    pub box_shadows: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum ColorValue {
    Solid(String),
    Gradient(String),
    Variable(String),
}
