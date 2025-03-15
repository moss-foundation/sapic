use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/parse.ts")]
pub struct ParseOptions {
    html_parse_mode: HtmlParseMode,
}

#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export, export_to = "operations/parse.ts")]
pub enum HtmlParseMode {
    Strict,
    Relaxed,
}
