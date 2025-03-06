use super::types::ThemeDescriptor;
use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct ListThemesOutput {
    pub contents: Vec<ThemeDescriptor>,
}
