use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "application.ts")]
pub struct Preferences {
    // TODO
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
// #[serde(rename_all = "camelCase")]
#[ts(export, export_to = "application.ts")]
pub struct Defaults {
    // TODO
}

#[derive(Debug, Deserialize, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "application.ts")]
pub struct AppState {
    /// The user preferences for the application.
    pub preferences: Preferences,
    pub defaults: Defaults,
}