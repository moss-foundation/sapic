use sapic_runtime::app::settings_storage::SettingScope;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(optional_fields)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "SettingScope")]
#[ts(export, export_to = "primitives.ts")]
pub enum SettingScopeForFrontend {
    User,
    Workspace(String),
}

impl From<SettingScopeForFrontend> for SettingScope {
    fn from(scope: SettingScopeForFrontend) -> Self {
        match scope {
            SettingScopeForFrontend::User => SettingScope::User,
            SettingScopeForFrontend::Workspace(workspace) => SettingScope::Workspace(workspace),
        }
    }
}
