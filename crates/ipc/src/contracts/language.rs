use sapic_base::language::types::LanguageInfo;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use ts_rs::TS;

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations.ts")]
pub struct GetTranslationNamespaceInput {
    pub language: String,
    pub namespace: String,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct GetTranslationNamespaceOutput {
    #[ts(type = "JsonValue")]
    pub contents: JsonValue,
}

/// @category Operation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "operations.ts")]
pub struct ListLanguagesOutput(#[ts(type = "LanguageInfo[]")] pub Vec<LanguageInfo>);
