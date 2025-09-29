use joinerror::{Result, ResultExt};
use regorus::Value as RegoValue;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};

use crate::models::primitives::ThemeMode;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ThemeManifestFile {
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

pub(crate) fn validate(policy_path: &Path, theme: &ThemeManifestFile) -> Result<()> {
    let mut engine = regorus::Engine::new();
    let policy_text =
        std::fs::read_to_string(policy_path).join_err::<()>("failed to read policy file")?;

    engine
        .add_policy("theme.rego".to_string(), policy_text)
        .join_err::<()>("failed to add theme rego policy")?;

    engine.set_input(RegoValue::from(serde_json::to_value(theme)?));

    let result = engine
        .eval_rule("data.theme.errors".to_string())
        .join_err::<()>("failed to evaluate theme rego rule")?;

    let errors = result
        .as_set()?
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(joinerror::Error::new::<()>(format!(
            "invalid theme:\n{}",
            errors.join("\n")
        )))
    }
}
