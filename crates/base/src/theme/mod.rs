pub mod contribution;
pub mod manifest;
pub mod types;

use crate::theme::manifest::{ColorValue, ThemeFile};

pub async fn convert(theme: &ThemeFile) -> joinerror::Result<String> {
    const INDENT: &str = "  ";

    let mut lines =
        Vec::with_capacity(theme.palette.len() + theme.colors.len() + theme.box_shadows.len() + 7);
    lines.push(":root {".to_string());
    lines.push(format!("{INDENT}/* Palette */"));
    for (token, color) in &theme.palette {
        lines.push(format!(
            "{INDENT}{}: {};",
            convert_token(token),
            convert_color(color),
        ));
    }

    lines.push("".to_string());
    lines.push(format!("{INDENT}/* Colors */"));

    for (token, color) in &theme.colors {
        lines.push(format!(
            "{INDENT}{}: {};",
            convert_token(token),
            convert_color(color),
        ));
    }

    lines.push("".to_string());
    lines.push(format!("{INDENT}/* BoxShadows */"));

    for (token, value) in &theme.box_shadows {
        lines.push(format!(
            "{INDENT}{}: {};",
            convert_token(token),
            value.to_string(),
        ));
    }
    lines.push("}".to_string());

    Ok(lines.join("\n"))
}

fn convert_color(color: &ColorValue) -> String {
    match color {
        ColorValue::Solid(color) => color.to_string(),
        ColorValue::Gradient(color) => color.to_string(),
        ColorValue::Variable(token) => format!("var({})", convert_token(token)),
    }
}

fn convert_token(token: &str) -> String {
    format!("--{}", token.replace(".", "-"))
}
