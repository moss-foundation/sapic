use serde::{Deserialize, Serialize};

/// @category Primitive
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ThemeMode {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
}
