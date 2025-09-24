use joinerror::{Result, ResultExt};
use regorus::Value as RegoValue;
use std::path::Path;

use crate::theme::models::types::Theme;

pub(crate) fn validate_theme(policy_path: &Path, theme: &Theme) -> Result<()> {
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
