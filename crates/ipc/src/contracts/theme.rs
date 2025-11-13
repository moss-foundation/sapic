use sapic_base::theme::types::{ColorThemeInfo, primitives::ThemeId};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// @category Operation
#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetColorThemeInput {
    #[ts(type = "ThemeId")]
    pub id: ThemeId,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetColorThemeOutput {
    pub css_content: String,
}

/// @category Operation
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListColorThemesOutput(#[ts(type = "Array<ColorThemeInfo>")] pub Vec<ColorThemeInfo>);
