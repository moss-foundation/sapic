use crate::primitives::ThemeId;

use super::types::ColorThemeDescriptor;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetColorThemeInput {
    pub id: ThemeId,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetColorThemeOutput {
    pub css_content: String,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListColorThemesOutput(pub Vec<ColorThemeDescriptor>);
