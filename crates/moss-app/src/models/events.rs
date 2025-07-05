use serde::Serialize;
use ts_rs::TS;

use crate::models::primitives::ThemeId;

/// @category Event
#[derive(Debug, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "events.ts")]
pub struct ColorThemeChangeEventPayload<'a> {
    pub id: &'a ThemeId,
}

impl<'a> ColorThemeChangeEventPayload<'a> {
    pub fn new(id: &'a ThemeId) -> Self {
        Self { id }
    }
}
