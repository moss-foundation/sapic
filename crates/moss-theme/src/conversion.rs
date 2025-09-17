use joinerror::Result;

use crate::models::{primitives::CssValue, types::Theme};

const INDENT: &str = "  ";

pub fn convert_theme_json_to_css(content: &str) -> Result<String> {
    let theme: Theme = serde_json::from_str(content)?;

    if theme.tokens.is_empty() {
        return Ok(":root {}\n".to_string());
    }

    let mut lines = Vec::with_capacity(theme.tokens.len() + 2);
    lines.push(":root {".to_string());

    for (token, value) in &theme.tokens {
        lines.push(format!(
            "{INDENT}{}: {};",
            convert_token(token),
            convert_value(value)
        ));
    }
    lines.push("}".to_string());

    Ok(lines.join("\n"))
}

fn convert_value(value: &CssValue) -> String {
    match value {
        CssValue::StringValue { value } => value.to_string(),
        CssValue::VariableValue { value } => {
            let var = convert_token(value);
            format!("var({})", var)
        }
    }
}

fn convert_token(token: &str) -> String {
    format!("--{}", token.replace(".", "-"))
}
